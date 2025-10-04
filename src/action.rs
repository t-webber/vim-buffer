use crate::Mode;

/// Action to be done on the buffer
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Inserts a char in the buffer
    InsertChar(char),
    /// Switches to a new mode
    SelectMode(Mode),
}
