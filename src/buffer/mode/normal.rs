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
            KeyCode::Char('a') =>
                vec![GoToAction::Right.into(), Mode::Insert.into()].into(),
            KeyCode::Char('i') => Mode::Insert.into(),
            KeyCode::Char('x') => Action::DeleteNextChar.into(),
            KeyCode::Char('s') => vec![
                GoToAction::Right.into(),
                Action::DeletePreviousChar,
                Mode::Insert.into(),
            ]
            .into(),
            KeyCode::Backspace | KeyCode::Char('h') => GoToAction::Left.into(),
            KeyCode::Char('l') => GoToAction::Right.into(),
            KeyCode::Char('f') => OPending::FindNext.into(),
            KeyCode::Char('t') => OPending::FindNextDecrement.into(),
            KeyCode::Char('r') => OPending::ReplaceOne.into(),
            KeyCode::Char('0') => GoToAction::Bol.into(),
            KeyCode::Char('^') => GoToAction::FirstNonSpace.into(),
            KeyCode::Char('$') => GoToAction::Eol.into(),
            KeyCode::Char('w') => GoToAction::NextWord.into(),
            _ => Actions::default(),
        }
    }

    fn handle_shift_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('I') =>
                vec![GoToAction::FirstNonSpace.into(), Mode::Insert.into()]
                    .into(),
            KeyCode::Char('A') =>
                vec![GoToAction::Eol.into(), Mode::Insert.into()].into(),
            KeyCode::Char('X') => Action::DeletePreviousChar.into(),
            KeyCode::Char('S') =>
                vec![Action::DeleteLine, Mode::Insert.into()].into(),
            KeyCode::Char('F') => OPending::FindPrevious.into(),
            KeyCode::Char('T') => OPending::FindPreviousIncrement.into(),
            KeyCode::Char('W') => GoToAction::NextWORD.into(),
            _ => Actions::default(),
        }
    }
}
