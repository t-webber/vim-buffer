use crossterm::event::KeyCode;

use crate::Mode;
use crate::buffer::keymaps::{Action, GoToAction, OPending};
use crate::buffer::mode::HandleKeyPress;

/// Struct to handle keypresses in insert mode
pub struct Insert;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleKeyPress for Insert {
    fn handle_blank_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action> {
        debug_assert!(pending.is_none(), "o-pending prevents mode switch");
        match code {
            KeyCode::Esc => vec![
                Action::GoTo(GoToAction::Left),
                Action::SelectMode(Mode::Normal),
            ],
            KeyCode::Char(ch) => vec![Action::InsertChar(ch)],
            KeyCode::Backspace => vec![Action::DeleteChar],
            _ => vec![],
        }
    }

    fn handle_shift_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action> {
        debug_assert!(pending.is_none(), "o-pending prevents mode switch");
        if let KeyCode::Char(ch) = code {
            vec![Action::InsertChar(ch.to_ascii_uppercase())]
        } else {
            vec![]
        }
    }
}
