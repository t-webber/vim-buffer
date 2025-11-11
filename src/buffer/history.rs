use crate::buffer::bounded_usize::BoundedUsize;

/// Holds the history of the buffer, with the following invariant: there are
/// never 2 successive entries that are equal.
#[derive(Debug)]
pub struct History<T>(Vec<T>, BoundedUsize);

impl<T> History<T> {
    /// Returns the entry at the current cursor.
    #[expect(clippy::indexing_slicing, reason = "usize bounded by len")]
    fn as_cursor_entry(&self) -> &T {
        &self.0[self.1.as_value()]
    }

    /// Returns `true` if the cursor is at the end of the history.
    ///
    /// This means that you can't redo.
    #[expect(clippy::arithmetic_side_effects, reason = "value < len")]
    const fn is_cursor_at_end(&self) -> bool {
        self.1.as_value() + 1 == self.0.len()
    }

    /// Moves forward into the history
    pub fn redo(&mut self) -> Option<&T> {
        self.1.increment().then(|| self.as_cursor_entry())
    }

    /// Saves a clone if it is different.
    #[expect(
        clippy::else_if_without_else,
        reason = "do nothing if entry is the same"
    )]
    fn save_clone<F: Fn() -> T>(
        &mut self,
        cloner: F,
        is_entry_same_as_cursor: bool,
    ) {
        if self.is_cursor_at_end() {
            self.0.push(cloner());
            self.1.increment_with_capacity_unchecked();
        } else if !is_entry_same_as_cursor {
            self.1.increment();
            self.0.truncate(self.1.as_value());
            self.0.push(cloner());
        }
    }

    /// Moves backward into the history
    pub fn undo(&mut self) -> Option<&T> {
        self.1.decrement().then(|| self.as_cursor_entry())
    }

    /// Creates a new [`History`], starting with the given string.
    pub fn with_initial_value(value: T) -> Self {
        Self(vec![value], BoundedUsize::default())
    }
}

impl History<Box<str>> {
    /// Saves the current buffervalue in the history
    pub fn save(&mut self, entry: &str) {
        self.save_clone(
            || Box::from(entry),
            *entry == **self.as_cursor_entry(),
        );
    }
}

impl<T: Default> Default for History<T> {
    fn default() -> Self {
        Self::with_initial_value(T::default())
    }
}
