use crossterm::event::KeyCode;

use crate::Mode;
use crate::buffer::keymaps::{Action, GoToAction, OPending};

/// Actions to be taken as a result of a keypress
pub enum Actions {
    /// List of buffer actions to be followed
    List(Vec<Action>),
    /// New opending state to use
    OPending(OPending),
}

impl Default for Actions {
    fn default() -> Self {
        vec![].into()
    }
}

impl From<Vec<Action>> for Actions {
    fn from(list: Vec<Action>) -> Self {
        Self::List(list)
    }
}

impl From<OPending> for Actions {
    fn from(opending: OPending) -> Self {
        Self::OPending(opending)
    }
}

impl From<Action> for Actions {
    fn from(action: Action) -> Self {
        vec![action].into()
    }
}

impl From<Mode> for Actions {
    fn from(action: Mode) -> Self {
        vec![action.into()].into()
    }
}

impl From<GoToAction> for Actions {
    fn from(action: GoToAction) -> Self {
        vec![action.into()].into()
    }
}

/// Handle incoming terminal events, like keypresses.
pub trait HandleKeyPress {
    /// Handle incoming terminal events that are keypresses with no modifiers.
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions;

    /// Handle incoming terminal events that are keypresses with no modifiers.
    fn handle_shift_key_press(&self, code: KeyCode) -> Actions;
}
