use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::action::{Action, GoToAction};
use crate::mode::{HandleEvent, Mode};

/// Struct to handle keypresses in insert mode
pub struct Normal;

impl HandleEvent for Normal {
    fn handle_event(self, event: &Event) -> Vec<Action> {
        event.as_key_press_event().map_or_else(Vec::new, |key_press_event| {
            match (key_press_event.code, key_press_event.modifiers) {
                (KeyCode::Char('a'), KeyModifiers::NONE) => vec![
                    Action::GoTo(GoToAction::Right),
                    Action::SelectMode(Mode::Insert),
                ],
                (KeyCode::Char('i'), KeyModifiers::NONE) =>
                    vec![Action::SelectMode(Mode::Insert)],
                (KeyCode::Char('I'), KeyModifiers::SHIFT) => vec![
                    Action::GoTo(GoToAction::FirstNonSpace),
                    Action::SelectMode(Mode::Insert),
                ],
                (KeyCode::Char('A'), KeyModifiers::SHIFT) => vec![
                    Action::GoTo(GoToAction::Eol),
                    Action::SelectMode(Mode::Insert),
                ],
                _ => vec![],
            }
        })
    }
}
