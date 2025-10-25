use crossterm::event::KeyCode;

use crate::buffer::keymaps::{Action, GoToAction, OPending};
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Struct to handle keypresses in insert mode
pub struct Normal;

#[expect(clippy::wildcard_enum_match_arm, reason = "only support a few")]
impl HandleKeyPress for Normal {
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('a') => vec![
                Action::GoTo(GoToAction::Right),
                Action::SelectMode(Mode::Insert),
            ],
            KeyCode::Char('i') => vec![Action::SelectMode(Mode::Insert)],
            KeyCode::Char('x') => vec![Action::DeleteChar],
            KeyCode::Backspace | KeyCode::Char('h') =>
                vec![Action::GoTo(GoToAction::Left)],
            KeyCode::Char('l') => vec![Action::GoTo(GoToAction::Right)],
            KeyCode::Char('f') => {
                return Actions::from(OPending::FindNext);
            }
            KeyCode::Char('0') => vec![Action::GoTo(GoToAction::Bol)],
            KeyCode::Char('^') => vec![Action::GoTo(GoToAction::FirstNonSpace)],
            KeyCode::Char('$') => vec![Action::GoTo(GoToAction::Eol)],
            _ => vec![],
        }
        .into()
    }

    fn handle_shift_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('I') => vec![
                Action::GoTo(GoToAction::FirstNonSpace),
                Action::SelectMode(Mode::Insert),
            ],
            KeyCode::Char('A') => vec![
                Action::GoTo(GoToAction::Eol),
                Action::SelectMode(Mode::Insert),
            ],
            KeyCode::Char('X') => vec![
                Action::GoTo(GoToAction::Left),
                Action::DeleteChar,
                Action::GoTo(GoToAction::Right),
            ],
            KeyCode::Char('F') => {
                return Actions::from(OPending::FindPrevious);
            }
            _ => vec![],
        }
        .into()
    }
}
