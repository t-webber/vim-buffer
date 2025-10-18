/// Handles keypresses in insert mode
mod insert;
/// Handles keypresses in normal mode
mod normal;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::action::Action;
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
        match self {
            Self::Insert => Insert.handle_key_press(code, modifiers),
            Self::Normal => Normal.handle_key_press(code, modifiers),
        }
    }
}

#[cfg(test)]
mod tests;
