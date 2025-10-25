use crossterm::event::KeyCode;

use crate::buffer::keymaps::{Action, OPending};

/// Actions to be taken as a result of a keypress
pub enum Actions {
    /// List of buffer actions to be followed
    List(Vec<Action>),
    /// New opending state to use
    OPending(OPending),
}

impl Default for Actions {
    fn default() -> Self {
        Self::List(vec![])
    }
}

impl From<OPending> for Actions {
    fn from(opending: OPending) -> Self {
        Self::OPending(opending)
    }
}

impl From<Vec<Action>> for Actions {
    fn from(list: Vec<Action>) -> Self {
        Self::List(list)
    }
}

/// Handle incomming terminal events, like keypresses.
pub trait HandleKeyPress {
    /// Handle incomming terminal events that are keypresses with no modifiers.
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions;

    /// Handle incomming terminal events that are keypresses with no modifiers.
    fn handle_shift_key_press(&self, code: KeyCode) -> Actions;
}
