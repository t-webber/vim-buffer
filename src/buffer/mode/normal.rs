use crossterm::event::KeyCode;

use crate::buffer::action::{Action, GoToAction};
use crate::buffer::mode::{HandleKeyPress, Mode};

/// Struct to handle keypresses in insert mode
pub struct Normal;

#[expect(clippy::wildcard_enum_match_arm, reason = "only support a few")]
impl HandleKeyPress for Normal {
    fn handle_blank_key_press(&self, code: KeyCode) -> Vec<Action> {
        match code {
            KeyCode::Char('a') => vec![
                Action::GoTo(GoToAction::Right),
                Action::SelectMode(Mode::Insert),
            ],
            KeyCode::Char('i') => vec![Action::SelectMode(Mode::Insert)],
            KeyCode::Char('h') => vec![Action::GoTo(GoToAction::Left)],
            KeyCode::Char('l') => vec![Action::GoTo(GoToAction::Right)],
            _ => vec![],
        }
    }

    fn handle_shift_key_press(&self, code: KeyCode) -> Vec<Action> {
        match code {
            KeyCode::Char('I') => vec![
                Action::GoTo(GoToAction::FirstNonSpace),
                Action::SelectMode(Mode::Insert),
            ],
            KeyCode::Char('A') => vec![
                Action::GoTo(GoToAction::Eol),
                Action::SelectMode(Mode::Insert),
            ],
            _ => vec![],
        }
    }
}
