use crossterm::event::Event;

use crate::action::Action;
use crate::bounded_usize::BoundedUsize;
use crate::mode::{HandleEvent as _, Mode};

/// Buffer that supports vim keymaps
#[derive(Default)]
pub struct Buffer {
    /// Content of the buffer
    content: String,
    /// Position of the cursor within the buffer
    cursor:  BoundedUsize,
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
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer exceeds [`usize::MAX`]
    pub fn update(&mut self, event: &Event) -> bool {
        let Some(action) = self.mode.handle_event(event) else {
            return false;
        };
        match action {
            Action::InsertChar(ch) => {
                self.content.insert(self.cursor.as_value(), ch);
                self.cursor.increment_with_capacity_unchecked();
            }
            Action::SelectMode(mode) => self.mode = mode,
            Action::Backspace => {
                self.content.pop();
                self.cursor.decrement_with_capacity();
            }
        }
        true
    }
}
