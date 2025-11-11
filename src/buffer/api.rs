use crate::Mode;
use crate::buffer::bounded_usize::BoundedUsize;
use crate::buffer::history::History;
use crate::buffer::keymaps::OPending;

/// Buffer that supports vim keymaps
///
/// # Examples
///
/// ```
/// use vim_buffer::crossterm::event::{Event, KeyCode, KeyEvent};
/// use vim_buffer::{Buffer, Mode};
///
/// let mut buffer = Buffer::default();
/// assert_eq!(buffer.as_mode(), Mode::Normal);
///
/// // Update it with crossterm events
/// buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char('i'))));
/// for ch in "hello".chars() {
///     buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char(ch))));
/// }
/// assert_eq!(buffer.as_content(), "hello");
///
/// // Update with Vim string
/// buffer.update_from_string("<Esc>0rHA, World!");
/// assert_eq!(buffer.as_content(), "Hello, World!");
/// ```
#[derive(Debug, Default)]
pub struct Buffer {
    /// Content of the buffer
    pub(super) content: String,
    /// Position of the cursor within the buffer
    pub(super) cursor: BoundedUsize,
    /// Buffer history to restore old versions
    pub(super) history: History<Box<str>>,
    /// Vim mode of the buffer
    pub(super) mode: Mode,
    /// Pending actions that require more keymaps
    pub(super) pending: Option<OPending>,
}

impl Buffer {
    /// Returns the inner text content of the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use vim_buffer::Buffer;
    ///
    /// assert_eq!(Buffer::from("abcdef").as_content(), "abcdef");
    /// ```
    #[must_use]
    pub const fn as_content(&self) -> &String {
        &self.content
    }

    /// Returns the cursor position in the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use vim_buffer::Buffer;
    ///
    /// assert_eq!(Buffer::from("abcdef").as_cursor(), 0);
    ///
    /// let mut buffer = Buffer::default();
    /// buffer.update_from_string("i abc");
    /// assert_eq!(buffer.as_cursor(), 4);
    /// buffer.update_from_string("<Esc>^");
    /// assert_eq!(buffer.as_cursor(), 1);
    /// ```
    #[must_use]
    pub const fn as_cursor(&self) -> usize {
        self.cursor.as_value()
    }

    /// Returns the vim mode of the buffer (insert, normal, etc.)
    ///
    /// ```
    /// use vim_buffer::{Buffer, Mode};
    ///
    /// let mut buffer = Buffer::default();
    /// assert_eq!(buffer.as_mode(), Mode::Normal);
    /// buffer.update_from_string("i");
    /// assert_eq!(buffer.as_mode(), Mode::Insert);
    /// buffer.update_from_string("<Esc>");
    /// assert_eq!(buffer.as_mode(), Mode::Normal);
    /// ```
    #[must_use]
    pub const fn as_mode(&self) -> Mode {
        self.mode
    }

    /// Returns `true` if the buffer is empty, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vim_buffer::Buffer;
    ///
    /// assert!(Buffer::default().is_empty());
    /// assert!(!Buffer::from("hello").is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Returns the length of the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use vim_buffer::Buffer;
    ///
    /// assert_eq!(Buffer::default().len(), 0);
    /// assert_eq!(Buffer::from("hello").len(), 5);
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        self.content.len()
    }
}

impl From<String> for Buffer {
    fn from(value: String) -> Self {
        Self {
            cursor: BoundedUsize::with_capacity(value.len()),
            history: History::with_initial_value(
                value.clone().into_boxed_str(),
            ),
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
