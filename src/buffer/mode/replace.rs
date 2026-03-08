use crossterm::event::KeyCode;

use crate::buffer::keymaps::{Action, GoToAction};
use crate::buffer::macros::actions;
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Struct to handle keypresses in replace mode
pub struct Replace;

#[expect(clippy::wildcard_enum_match_arm, reason = "only support a few")]
impl HandleKeyPress for Replace {
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Esc => Mode::Normal.into(),
            KeyCode::Char(ch) =>
                actions![Action::ReplaceOrInsert(ch), GoToAction::Right],
            KeyCode::Left => GoToAction::Left.into(),
            KeyCode::Right => GoToAction::Right.into(),
            _ => Actions::default(),
        }
    }

    fn handle_shift_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char(ch) =>
                actions![Action::ReplaceOrInsert(ch), GoToAction::Right],
            _ => Actions::default(),
        }
    }
}
