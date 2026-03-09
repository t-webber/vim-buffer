use crossterm::event::KeyCode;

use crate::buffer::keymaps::{Action, GoToAction, Operator};
use crate::buffer::macros::actions;
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Struct to handle keypresses in insert mode
pub struct Insert;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleKeyPress for Insert {
    fn handle_blank_key_press(&mut self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Esc => actions![GoToAction::Left, Mode::Normal],
            KeyCode::Char(ch) => Action::InsertChar(ch).into(),
            KeyCode::Backspace => actions![
                GoToAction::Left,
                (Operator::Delete, GoToAction::Right.into())
            ],
            KeyCode::Left => GoToAction::Left.into(),
            KeyCode::Right => GoToAction::Right.into(),
            _ => Actions::Unsupported,
        }
    }

    fn handle_ctrl_key_press(&mut self, _: KeyCode) -> Actions {
        Actions::Unsupported
    }

    fn handle_shift_key_press(&mut self, code: KeyCode) -> Actions {
        if let KeyCode::Char(ch) = code {
            Action::InsertChar(ch.to_ascii_uppercase()).into()
        } else {
            Actions::Unsupported
        }
    }
}
