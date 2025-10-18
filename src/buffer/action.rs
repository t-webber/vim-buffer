use crate::Mode;

/// Action to be done on the buffer
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Deletes the last written char
    Backspace,
    /// Action to move the cursor to a location denotated by a condition
    GoTo(GoToAction),
    /// Inserts a char in the buffer
    InsertChar(char),
    /// Switches to a new mode
    SelectMode(Mode),
}

/// Actions to move the cursor
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoToAction {
    /// End of line, like with `$` and `A`
    Eol,
    /// First non space character, like with `I` and `^`
    FirstNonSpace,
    /// Move the cursor left by one character
    Left,
    /// Move the cursor right by one character
    Right,
}
