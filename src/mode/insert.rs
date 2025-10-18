use crossterm::event::KeyCode;

use crate::action::{Action, GoToAction};
use crate::mode::{HandleKeyPress, Mode};

/// Struct to handle keypresses in insert mode
pub struct Insert;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleKeyPress for Insert {
    fn handle_blank_key_press(&self, code: KeyCode) -> Vec<Action> {
        match code {
            KeyCode::Esc => vec![
                Action::GoTo(GoToAction::Left),
                Action::SelectMode(Mode::Normal),
            ],
            KeyCode::Char(ch) => vec![Action::InsertChar(ch)],
            KeyCode::Backspace => vec![Action::Backspace],
            _ => vec![],
        }
    }

    fn handle_shift_key_press(&self, _: KeyCode) -> Vec<Action> {
        vec![]
    }
}
