use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::Mode;
use crate::buffer::keymaps::{Action, GoToAction};
use crate::buffer::macros::actions;

/// Actions to be taken as a result of a keypress
#[derive(Debug, PartialEq, Eq)]
pub enum Actions {
    /// List of buffer actions to be followed
    List(Vec<Action>),
    /// The given keycode is not supported or has no meaning
    Unsupported,
}

impl Actions {
    /// Constant that represents no actions.
    ///
    /// This can happen when an event is swallowed by a pending operation.
    pub const NONE: Self = Self::List(Vec::new());

    /// Repeats the action `occurrences` times, if possible.
    pub fn repeat(self, occurrences: usize) -> Self {
        match self {
            Self::List(actions) => Self::List(actions.repeat(occurrences)),
            Self::Unsupported => Self::Unsupported,
        }
    }
}

impl From<Vec<Action>> for Actions {
    fn from(list: Vec<Action>) -> Self {
        Self::List(list)
    }
}

impl From<Action> for Actions {
    fn from(action: Action) -> Self {
        actions![action]
    }
}

impl From<Mode> for Actions {
    fn from(action: Mode) -> Self {
        Action::from(action).into()
    }
}

impl From<GoToAction> for Actions {
    fn from(action: GoToAction) -> Self {
        Action::from(action).into()
    }
}

/// Handle incoming terminal events, like keypresses.
pub trait HandleKeyPress {
    /// Handle incoming terminal events off any kind.
    fn default_handle_key(&mut self, event: &Event) -> Actions {
        event.as_key_press_event().map_or(
            Actions::Unsupported,
            |mut key_event| {
                fix_shift_modifier(&mut key_event);
                self.dispatch_on_modifiers(&key_event)
            },
        )
    }

    /// Dispatch to the right handler depending on the modifiers of the event.
    fn dispatch_on_modifiers(&mut self, key_event: &KeyEvent) -> Actions {
        match key_event.modifiers {
            KeyModifiers::NONE => self.handle_blank_key_press(key_event.code),
            KeyModifiers::CONTROL => self.handle_ctrl_key_press(key_event.code),
            KeyModifiers::SHIFT => self.handle_shift_key_press(key_event.code),
            _ => Actions::Unsupported,
        }
    }

    /// Handle incoming terminal events that are keypresses with no modifiers.
    fn handle_blank_key_press(&mut self, code: KeyCode) -> Actions;

    /// Handle incoming terminal events that are keypresses with the control
    /// modifier.
    fn handle_ctrl_key_press(&mut self, code: KeyCode) -> Actions;

    /// Handle incoming terminal events off any kind.
    fn handle_key(&mut self, event: &Event) -> Actions {
        self.default_handle_key(event)
    }

    /// Handle incoming terminal events that are keypresses with the shift
    /// modifier.
    fn handle_shift_key_press(&mut self, code: KeyCode) -> Actions;
}

/// Adds [`KeyModifiers::SHIFT`] if the event is a capital char, and capitalises
/// the char if the modifiers contain shift.
const fn fix_shift_modifier(key_event: &mut KeyEvent) {
    #[expect(clippy::else_if_without_else, reason = "checked")]
    if let KeyCode::Char(ch) = &mut key_event.code {
        if ch.is_ascii_uppercase() {
            key_event.modifiers =
                key_event.modifiers.union(KeyModifiers::SHIFT);
        } else if key_event.modifiers.contains(KeyModifiers::SHIFT) {
            *ch = ch.to_ascii_uppercase();
        }
    }
}
