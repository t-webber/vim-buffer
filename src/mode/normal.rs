use crossterm::event::{Event, KeyCode};

use crate::Mode;
use crate::action::Action;
use crate::mode::HandleEvent;

/// Struct to handle keypresses in insert mode
pub struct Normal;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleEvent for Normal {
    fn handle_event(self, event: Event) -> Option<Action> {
        let key_press_event = event.as_key_press_event()?;
        match key_press_event.code {
            KeyCode::Char('i') => Some(Action::SelectMode(Mode::Insert)),
            _ => None,
        }
    }
}
