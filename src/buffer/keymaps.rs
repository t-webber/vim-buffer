use crate::Mode;

/// Action to be done on the buffer
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Deletes the whole line
    DeleteLine,
    /// Deletes the char after the cursor
    DeleteNextChar,
    /// Deletes the char before the cursor
    DeletePreviousChar,
    /// Action to move the cursor to a location denotated by a condition
    GoTo(GoToAction),
    /// Inserts a char in the buffer
    InsertChar(char),
    /// Replace the char under the cursor with
    ReplaceWith(char),
    /// Switches to a new mode
    SelectMode(Mode),
    /// Undo the last edition
    Undo,
}

impl From<GoToAction> for Action {
    fn from(value: GoToAction) -> Self {
        Self::GoTo(value)
    }
}

impl From<Mode> for Action {
    fn from(value: Mode) -> Self {
        Self::SelectMode(value)
    }
}

/// Actions to move the cursor
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoToAction {
    /// Beginning of line (column 0), reached with `0`
    Bol,
    /// End of line, like with `$` and `A`
    Eol,
    /// First non space character, like with `I` and `^`
    FirstNonSpace,
    /// Move the cursor left by one character
    Left,
    /// Find next occurrence of char and place cursor on it
    NextOccurrenceOf(char),
    /// Move to the beginning of the next WORD
    NextWORD,
    /// Move to the beginning of the next word
    NextWord,
    /// Find previous occurrence of char and place cursor on it
    PreviousOccurrenceOf(char),
    /// Move to the beginning of the previous WORD
    PreviousWORD,
    /// Move to the beginning of the previous word
    PreviousWord,
    /// Move the cursor right by one character
    Right,
}

/// Action that is pending for another keypress
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OPending {
    /// Find next char that is equal to...
    FindNext,
    /// Find next char that is equal to... and decrement
    FindNextDecrement,
    /// Find previous char that is equal to...
    FindPrevious,
    /// Find previous char that is equal to... and increment
    FindPreviousIncrement,
    /// Replace one character
    ReplaceOne,
}
