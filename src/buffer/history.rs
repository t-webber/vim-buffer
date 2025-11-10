use crate::buffer::bounded_usize::BoundedUsize;

/// Holds the history of the buffer, with the following invariant: there are
/// never 2 successive entries that are equal.
#[derive(Debug)]
pub struct History(Vec<Box<str>>, BoundedUsize);

impl History {
    /// Returns the entry at the current cursor.
    #[expect(clippy::indexing_slicing, reason = "usize bounded by len")]
    fn as_cursor_entry(&self) -> &str {
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
    pub fn redo(&mut self, current: &str) -> Option<&str> {
        assert_eq!(current, self.as_cursor_entry(), "invalid history");
        self.1.increment().then(|| self.as_cursor_entry())
    }

    /// Saves the current buffervalue in the history
    pub fn save(&mut self, entry: &str) {
        if self.is_cursor_at_end() {
            self.0.push(Box::from(entry));
            self.1.increment_with_capacity_unchecked();
            return;
        }
        if entry == self.as_cursor_entry() {
            return;
        }
        self.1.increment();
        self.0.truncate(self.1.as_value());
        self.0.push(Box::from(entry));
    }

    /// Moves backward into the history
    pub fn undo(&mut self, current: &str) -> Option<&str> {
        assert_eq!(current, self.as_cursor_entry(), "invalid history");
        self.1.decrement().then(|| self.as_cursor_entry())
    }

    /// Creates a new [`History`], starting with the given string.
    pub fn with_initial_value(value: Box<str>) -> Self {
        let len = value.len();
        Self(vec![value], BoundedUsize::with_capacity(len))
    }
}

impl Default for History {
    fn default() -> Self {
        Self::with_initial_value(Box::default())
    }
}
