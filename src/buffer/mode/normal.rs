use crossterm::event::KeyCode;

use crate::buffer::keymaps::{
    Action, CombinablePending, GoToAction, OPending, Operator, OperatorScope
};
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Struct to handle keypresses in normal mode
pub struct Normal;

#[expect(clippy::wildcard_enum_match_arm, reason = "only support a few")]
impl HandleKeyPress for Normal {
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('$') => GoToAction::EndOfLine.into(),
            KeyCode::Char('0') => GoToAction::BeginningOfLine.into(),
            KeyCode::Char('^') => GoToAction::FirstNonSpace.into(),
            KeyCode::Char('a') =>
                vec![GoToAction::Right.into(), Mode::Insert.into()].into(),
            KeyCode::Char('b') => GoToAction::BeginningOfWord.into(),
            KeyCode::Char('c') => Operator::Change.into(),
            KeyCode::Char('d') => Operator::Delete.into(),
            KeyCode::Char('e') => GoToAction::EndWord.into(),
            KeyCode::Char('f') => CombinablePending::FindNext.into(),
            KeyCode::Char('g') => OPending::GoTo.into(),
            KeyCode::Char('h') | KeyCode::Backspace => GoToAction::Left.into(),
            KeyCode::Char('i') => Mode::Insert.into(),
            KeyCode::Char('l') => GoToAction::Right.into(),
            KeyCode::Char('x') => Action::DeleteNextChar.into(),
            KeyCode::Char('r') => OPending::ReplaceOne.into(),
            KeyCode::Char('s') => vec![
                GoToAction::Right.into(),
                Action::DeletePreviousChar,
                Mode::Insert.into(),
            ]
            .into(),
            KeyCode::Char('t') => CombinablePending::FindNextDecrement.into(),
            KeyCode::Char('u') => Action::Undo.into(),
            KeyCode::Char('w') => GoToAction::NextWord.into(),
            KeyCode::Char('~') =>
                vec![Action::ToggleCapitalisation, GoToAction::Right.into()]
                    .into(),
            _ => Actions::default(),
        }
    }

    fn handle_ctrl_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('r') => Action::Redo.into(),
            _ => Actions::default(),
        }
    }

    fn handle_shift_key_press(&self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('A') =>
                vec![GoToAction::EndOfLine.into(), Mode::Insert.into()].into(),
            KeyCode::Char('B') => GoToAction::BeginningOfWORD.into(),
            KeyCode::Char('C') => vec![
                Action::Delete(GoToAction::EndOfLine.into()),
                Mode::Insert.into(),
            ]
            .into(),
            KeyCode::Char('D') =>
                Action::Delete(GoToAction::EndOfLine.into()).into(),
            KeyCode::Char('E') => GoToAction::EndWORD.into(),
            KeyCode::Char('F') => CombinablePending::FindPrevious.into(),
            KeyCode::Char('I') =>
                vec![GoToAction::FirstNonSpace.into(), Mode::Insert.into()]
                    .into(),
            KeyCode::Char('S') => vec![
                Action::Delete(OperatorScope::WholeLine),
                Mode::Insert.into(),
            ]
            .into(),
            KeyCode::Char('T') =>
                CombinablePending::FindPreviousIncrement.into(),
            KeyCode::Char('W') => GoToAction::NextWORD.into(),
            KeyCode::Char('X') => Action::DeletePreviousChar.into(),
            _ => Actions::default(),
        }
    }
}
