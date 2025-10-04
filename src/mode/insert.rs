use crossterm::event::{Event, KeyCode};

use crate::action::Action;
use crate::mode::{HandleEvent, Mode};

/// Struct to handle keypresses in insert mode
pub struct Insert;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleEvent for Insert {
    fn handle_event(self, event: &Event) -> Option<Action> {
        let key_press_event = event.as_key_press_event()?;
        match key_press_event.code {
            KeyCode::Esc => Some(Action::SelectMode(Mode::Normal)),
            KeyCode::Char(ch) => Some(Action::InsertChar(ch)),
            _ => None,
        }
    }
}
