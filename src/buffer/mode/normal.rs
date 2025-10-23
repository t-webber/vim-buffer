use core::mem::take;

use crossterm::event::KeyCode;

use crate::buffer::action::{Action, GoToAction};
use crate::buffer::mode::{HandleKeyPress, Mode};
use crate::buffer::o_pending::OPending;

/// Struct to handle keypresses in insert mode
pub struct Normal;

#[expect(clippy::wildcard_enum_match_arm, reason = "only support a few")]
impl HandleKeyPress for Normal {
    fn handle_blank_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action> {
        match take(pending) {
            Some(OPending::FindNext) =>
                if let KeyCode::Char(ch) = code {
                    vec![Action::GoTo(GoToAction::NextOccurrenceOf(ch))]
                } else {
                    vec![]
                },
            None => match code {
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
                    *pending = Some(OPending::FindNext);
                    vec![]
                }
                KeyCode::Char('0') => vec![Action::GoTo(GoToAction::Bol)],
                KeyCode::Char('^') =>
                    vec![Action::GoTo(GoToAction::FirstNonSpace)],
                KeyCode::Char('$') => vec![Action::GoTo(GoToAction::Eol)],
                _ => vec![],
            },
        }
    }

    fn handle_shift_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action> {
        *pending = None;
        match code {
            KeyCode::Char('I' | 'i') => vec![
                Action::GoTo(GoToAction::FirstNonSpace),
                Action::SelectMode(Mode::Insert),
            ],
            KeyCode::Char('A' | 'a') => vec![
                Action::GoTo(GoToAction::Eol),
                Action::SelectMode(Mode::Insert),
            ],
            KeyCode::Char('X') => vec![
                Action::GoTo(GoToAction::Left),
                Action::DeleteChar,
                Action::GoTo(GoToAction::Right),
            ],
            _ => vec![],
        }
    }
}
