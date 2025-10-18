/// Handles keypresses in insert mode
mod insert;
/// Handles keypresses in normal mode
mod normal;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::action::{Action, GoToAction};
use crate::mode::insert::Insert;
use crate::mode::normal::Normal;

/// Represents the vim mode of the buffer.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Insert mode
    ///
    /// To type in content.
    ///
    /// Press `<Esc>` to exit it.
    Insert,
    /// Normal mode
    ///
    /// To move and edit with vim motions.
    ///
    /// Press a, i, A, or I to exit it.
    #[default]
    Normal,
}

/// Handle incomming terminal events, like keypresses.
pub trait HandleKeyPress {
    /// Handle incomming terminal events, like keypresses.
    fn handle_key_press(
        self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> Vec<Action>;
}

impl HandleKeyPress for Mode {
    fn handle_key_press(
        self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> Vec<Action> {
        if modifiers == KeyModifiers::NONE {
            #[expect(
                clippy::wildcard_enum_match_arm,
                reason = "take only a few"
            )]
            match code {
                KeyCode::Left => return vec![Action::GoTo(GoToAction::Left)],
                KeyCode::Right => return vec![Action::GoTo(GoToAction::Right)],
                _ => (),
            }
        }
        match self {
            Self::Insert => Insert.handle_key_press(code, modifiers),
            Self::Normal => Normal.handle_key_press(code, modifiers),
        }
    }
}

#[cfg(test)]
mod tests;
