use crossterm::event::KeyCode;

use crate::buffer::keymaps::{Action, GoToAction};
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Struct to handle keypresses in insert mode
pub struct Insert;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleKeyPress for Insert {
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Esc =>
                vec![GoToAction::Left.into(), Action::SelectMode(Mode::Normal)]
                    .into(),
            KeyCode::Char(ch) => Action::InsertChar(ch).into(),
            KeyCode::Backspace => Action::DeletePreviousChar.into(),
            _ => Actions::default(),
        }
    }

    fn handle_shift_key_press(&self, code: KeyCode) -> Actions {
        if let KeyCode::Char(ch) = code {
            Action::InsertChar(ch.to_ascii_uppercase()).into()
        } else {
            Actions::default()
        }
    }
}
