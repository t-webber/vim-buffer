use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::action::Action;
use crate::mode::HandleEvent;

/// Struct to handle keypresses that are valid in all modes
pub struct All;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleEvent for All {
    fn handle_event(self, event: &Event) -> Vec<Action> {
        if let Some(key_press_event) = event.as_key_press_event()
            && key_press_event.modifiers == KeyModifiers::NONE
        {
            match key_press_event.code {
                KeyCode::Left => vec![Action::DecrementCursor(1)],
                KeyCode::Right => vec![Action::IncrementCursor(1)],
                _ => vec![],
            }
        } else {
            vec![]
        }
    }
}
