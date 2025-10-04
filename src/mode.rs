//! Handles the vim modes and the keypresses on those modes

/// Represents the vim mode of the buffer.
#[non_exhaustive]
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
    Normal,
}
