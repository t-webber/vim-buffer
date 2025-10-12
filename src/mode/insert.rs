use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::action::Action;
use crate::mode::{HandleEvent, Mode};

/// Struct to handle keypresses in insert mode
pub struct Insert;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "partially implement events"
)]
impl HandleEvent for Insert {
    fn handle_event(self, event: &Event) -> Vec<Action> {
        if let Some(key_press_event) = event.as_key_press_event()
            && key_press_event.modifiers == KeyModifiers::NONE
        {
            match key_press_event.code {
                KeyCode::Esc => vec![
                    Action::DecrementCursor(1),
                    Action::SelectMode(Mode::Normal),
                ],
                KeyCode::Char(ch) => vec![Action::InsertChar(ch)],
                KeyCode::Backspace => vec![Action::Backspace],
                _ => vec![],
            }
        } else {
            vec![]
        }
    }
}
