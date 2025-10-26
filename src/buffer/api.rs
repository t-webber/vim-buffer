use crate::Mode;
use crate::buffer::bounded_usize::BoundedUsize;
use crate::buffer::keymaps::OPending;

/// Buffer that supports vim keymaps
#[derive(Debug, Default)]
pub struct Buffer {
    /// Content of the buffer
    pub(super) content: String,
    /// Position of the cursor within the buffer
    pub(super) cursor: BoundedUsize,
    /// Vim mode of the buffer
    pub(super) mode: Mode,
    /// Pending actions that require more keymaps
    pub(super) pending: Option<OPending>,
}

impl Buffer {
    /// Returns the inner text content of the buffer
    #[must_use]
    pub const fn as_content(&self) -> &String {
        &self.content
    }

    /// Returns the cursor position in the buffer
    #[must_use]
    pub const fn as_cursor(&self) -> usize {
        self.cursor.as_value()
    }

    /// Returns the vim mode of the buffer (insert, normal, etc.)
    #[must_use]
    pub const fn as_mode(&self) -> Mode {
        self.mode
    }

    /// Returns `true` if the buffer is empty, and `false` otherwise.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Returns the length of the buffer
    #[must_use]
    pub const fn len(&self) -> usize {
        self.content.len()
    }
}

impl From<String> for Buffer {
    fn from(value: String) -> Self {
        Self {
            cursor: BoundedUsize::with_capacity(value.len()),
            content: value,
            ..Default::default()
        }
    }
}

impl From<&str> for Buffer {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}
