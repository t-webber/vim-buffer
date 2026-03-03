use crossterm::event::KeyCode;

use crate::Mode;
use crate::buffer::keymaps::{
    Action, CombinablePending, GoToAction, OPending, Operator
};
use crate::buffer::macros::actions;

/// Actions to be taken as a result of a keypress
#[derive(Debug, PartialEq, Eq)]
pub enum Actions {
    /// List of buffer actions to be followed
    List(Vec<Action>),
    /// New opending state to use
    OPending(OPending),
}

impl Default for Actions {
    fn default() -> Self {
        actions![]
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

impl From<Operator> for Actions {
    fn from(op: Operator) -> Self {
        Self::OPending(op.into())
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

impl From<CombinablePending> for Actions {
    fn from(action: CombinablePending) -> Self {
        Self::OPending(OPending::CombinablePending(action))
    }
}

/// Handle incoming terminal events, like keypresses.
#[expect(unused_variables, reason = "trait default")]
pub trait HandleKeyPress {
    /// Handle incoming terminal events that are keypresses with no modifiers.
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions;

    /// Handle incoming terminal events that are keypresses with the control
    /// modifier.
    fn handle_ctrl_key_press(&self, code: KeyCode) -> Actions {
        Actions::default()
    }

    /// Handle incoming terminal events that are keypresses with the shift
    /// modifier.
    fn handle_shift_key_press(&self, code: KeyCode) -> Actions;
}
