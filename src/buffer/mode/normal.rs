use crossterm::event::KeyCode;

use crate::buffer::keymaps::{
    Action, CombinablePending, GoToAction, OPending, Operator, OperatorScope
};
use crate::buffer::macros::actions;
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
            KeyCode::Char('a') => actions![GoToAction::Right, Mode::Insert],
            KeyCode::Char('b') => GoToAction::BeginningOfWord.into(),
            KeyCode::Char('c') => Operator::Change.into(),
            KeyCode::Char('d') => Operator::Delete.into(),
            KeyCode::Char('e') => GoToAction::EndWord.into(),
            KeyCode::Char('f') => CombinablePending::FindNext.into(),
            KeyCode::Char('g') => OPending::GoTo.into(),
            KeyCode::Char('h') | KeyCode::Backspace | KeyCode::Left =>
                GoToAction::Left.into(),
            KeyCode::Char('i') => Mode::Insert.into(),
            KeyCode::Char('l') | KeyCode::Right => GoToAction::NextChar.into(),
            KeyCode::Char('x') => actions![
                (Operator::Delete, GoToAction::Right.into()),
                GoToAction::Right,
                GoToAction::Left
            ],
            KeyCode::Char('p') => Action::PasteAfter.into(),
            KeyCode::Char('r') => OPending::ReplaceOne.into(),
            KeyCode::Char('s') => actions![
                (Operator::Delete, GoToAction::Right.into()),
                Mode::Insert
            ],
            KeyCode::Char('t') => CombinablePending::FindNextDecrement.into(),
            KeyCode::Char('u') => Action::Undo.into(),
            KeyCode::Char('w') => GoToAction::NextWord.into(),
            KeyCode::Char('y') => Operator::Yank.into(),
            KeyCode::Char('~') => actions![
                (Operator::ToggleCase, GoToAction::Right.into()),
                GoToAction::NextChar
            ],
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
            KeyCode::Char('A') => actions![GoToAction::EndOfLine, Mode::Insert],
            KeyCode::Char('B') => GoToAction::BeginningOfWORD.into(),
            KeyCode::Char('C') => actions![
                (Operator::Delete, GoToAction::EndOfLine.into()),
                Mode::Insert
            ],
            KeyCode::Char('D') =>
                vec![(Operator::Delete, GoToAction::EndOfLine.into()).into()]
                    .into(),
            KeyCode::Char('E') => GoToAction::EndWORD.into(),
            KeyCode::Char('F') => CombinablePending::FindPrevious.into(),
            KeyCode::Char('I') =>
                actions![GoToAction::FirstNonSpace, Mode::Insert],
            KeyCode::Char('P') => Action::PasteBefore.into(),
            KeyCode::Char('S') => actions![
                (Operator::Delete, OperatorScope::WholeLine),
                Mode::Insert
            ],
            KeyCode::Char('T') =>
                CombinablePending::FindPreviousIncrement.into(),
            KeyCode::Char('W') => GoToAction::NextWORD.into(),
            KeyCode::Char('X') => actions![
                GoToAction::Left,
                (Operator::Delete, GoToAction::Right.into())
            ],
            KeyCode::Char('Y') =>
                actions![(Operator::Yank, GoToAction::EndOfLine.into())],
            _ => Actions::default(),
        }
    }
}
