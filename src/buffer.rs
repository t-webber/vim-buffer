use crossterm::event::Event;

use crate::action::Action;
use crate::mode::{HandleEvent as _, Mode};

/// Buffer that supports vim keymaps
#[derive(Default)]
pub struct Buffer {
    /// Content of the buffer
    content: String,
    /// Vim mode of the buffer
    mode:    Mode,
}

impl Buffer {
    /// Returns the inner text content of the buffer
    #[must_use]
    pub const fn as_content(&self) -> &String {
        &self.content
    }

    /// Returns the vim mode of the buffer (insert, normal, etc.)
    #[must_use]
    pub const fn as_mode(&self) -> Mode {
        self.mode
    }

    /// Updates the buffer with a terminal events
    ///
    /// # Returns
    ///
    /// `true` if the buffer was changed, and `false` if the [`Event`] is
    /// ignored.
    pub fn update(&mut self, event: &Event) -> bool {
        let Some(action) = self.mode.handle_event(event) else {
            return false;
        };
        match action {
            Action::InsertChar(ch) => self.content.push(ch),
            Action::SelectMode(mode) => self.mode = mode,
        }
        true
    }
}
