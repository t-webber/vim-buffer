use core::iter::{Rev, Skip};
use core::str::CharIndices;

use crate::Buffer;

impl Buffer {
    /// Capitalise part of the buffer
    pub(super) fn apply<F>(&mut self, start: usize, end: usize, apply: F)
    where F: Fn(&char) -> char {
        self.content = self
            .as_content()
            .char_indices()
            .map(
                |(idx, ch)| {
                    if idx < start || idx >= end { ch } else { apply(&ch) }
                },
            )
            .collect();
    }

    /// Returns the char pointed by the cursor
    ///
    /// # Panics
    ///
    /// If the cursor went out of bounds of the buffer
    #[expect(clippy::unwrap_used, reason = "in bound")]
    pub(super) fn as_char(&self) -> char {
        self.content.chars().nth(self.as_cursor()).unwrap()
    }

    /// Returns the index of the cursor, starting from the end of the
    /// string.
    #[expect(clippy::arithmetic_side_effects, reason = "cursor <= len")]
    pub(super) const fn as_end_index(&self) -> usize {
        self.len() - self.as_cursor()
    }

    /// Returns [`CharIndices`] iterator for all chars located after the
    /// cursor in the buffer.
    pub(super) fn chars_after_cursor(&self) -> Skip<CharIndices<'_>> {
        self.as_content().char_indices().skip(self.as_cursor())
    }

    /// Returns [`CharIndices`] iterator for all chars located before the
    /// cursor in the buffer, and this in a reverse order.
    pub(super) fn chars_before_cursor_rev(&self) -> Skip<Rev<CharIndices<'_>>> {
        self.as_content().char_indices().rev().skip(self.as_end_index())
    }
}
