use crossterm::event::Event;

use crate::buffer::mode::insert::Insert;
use crate::buffer::mode::normal::Normal;
use crate::buffer::mode::replace::Replace;
use crate::buffer::mode::traits::{Actions, HandleKeyPress as _};

/// Represents the vim mode of the buffer.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Insert mode
    Insert,
    /// Normal mode
    #[default]
    Normal,
    /// Replace mode
    Replace,
}

/// Represents the vim mode of the buffer.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferMode {
    /// Insert mode
    Insert,
    /// Normal mode
    Normal(Normal),
    /// Replace mode
    Replace,
}

impl Default for BufferMode {
    fn default() -> Self {
        Self::Normal(Normal::new())
    }
}

impl BufferMode {
    /// Handle incoming terminal events off any kind.
    pub fn handle_event(&mut self, event: Event) -> Actions {
        match self {
            Self::Insert => Insert.handle_key(event),
            Self::Normal(normal) => normal.handle_key(event),
            Self::Replace => Replace.handle_key(event),
        }
    }

    /// Edits the current [`BufferMode`] for it to be of mode `mode`. All states
    /// will be lost, even if the new mode is the same as the last one.
    pub const fn switch_to(&mut self, mode: Mode) {
        *self = match mode {
            Mode::Insert => Self::Insert,
            Mode::Normal => Self::Normal(Normal::new()),
            Mode::Replace => Self::Replace,
        };
    }

    /// Returns the [`Mode`] that corresponds to the current [`BufferMode`].
    pub const fn to_mode(self) -> Mode {
        match self {
            Self::Insert => Mode::Insert,
            Self::Normal(_) => Mode::Normal,
            Self::Replace => Mode::Replace,
        }
    }
}
