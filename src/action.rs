use crate::mode::Mode;

/// Action to be done on the buffer
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Deletes the last written char
    Backspace,
    /// Decrements the buffer cursor
    DecrementCursor(usize),
    /// Action to move the cursor to a location denotated by a condition
    GoTo(GoToAction),
    /// Increments the buffer cursor
    IncrementCursor(usize),
    /// Inserts a char in the buffer
    InsertChar(char),
    /// Switches to a new mode
    SelectMode(Mode),
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoToAction {
    /// First non space character, like with `I` and `^`
    FirstNonSpace,
}
