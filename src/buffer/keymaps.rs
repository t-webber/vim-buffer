use crate::Mode;
use crate::buffer::macros::actions;
use crate::buffer::mode::Actions;

/// Action to be done on the buffer
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Delete
    Delete(OperatorScope),
    /// Action to move the cursor to a location denotated by a condition
    GoTo(GoToAction),
    /// Inserts a char in the buffer
    InsertChar(char),
    /// Undo the last undo action
    Redo,
    /// Replace the char under the cursor with
    ReplaceWith(char),
    /// Switches to a new mode
    SelectMode(Mode),
    /// Capitalises minuscules and lowers capitals
    ToggleCapitalisation,
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
    BeginningOfLine,
    /// Move to the beginning of the previous WORD
    BeginningOfWORD,
    /// Move to the beginning of the previous word
    BeginningOfWord,
    /// End of line, like with `$` and `A`
    EndOfLine,
    /// Move to the end of the previous WORD, reached with `gE`
    EndOfPreviousWORD,
    /// Move to the end of the previous word, reached with `ge`
    EndOfPreviousWord,
    /// End of current or next WORD, reached with `E`
    EndWORD,
    /// End of current or next word, reached with `e`
    EndWord,
    /// First non space character, like with `I` and `^`
    FirstNonSpace,
    /// Move the cursor left by one character
    Left,
    /// Move the cursor right by one character, stopping at the last character
    ///
    /// Differs from [`Self::Right`] as it will never go beyond the last
    /// character.
    NextChar,
    /// Find next occurrence of char and place cursor on it
    NextOccurrenceOf(char),
    /// Move to the beginning of the next WORD
    NextWORD,
    /// Move to the beginning of the next word
    NextWord,
    /// Find previous occurrence of char and place cursor on it
    PreviousOccurrenceOf(char),
    /// Move the cursor right by one character
    Right,
}

/// Action that is pending for another keypress
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OPending {
    /// Pending action that only requires 1 character to form a goto action.
    ///
    /// Combinable with an [`Operator`], see [`Self::OperatorAction`].
    CombinablePending(CombinablePending),
    /// Applies a single char action to a motion.
    GoTo,
    /// Operator action, like `d`, `c`, `g~`
    Operator(Operator),
    /// Operator action that has the motion pending, like `df`, `cf`, `g~f`
    OperatorAction(Operator, CombinablePending),
    /// Replace one character
    ReplaceOne,
}

impl From<Operator> for OPending {
    fn from(value: Operator) -> Self {
        Self::Operator(value)
    }
}

/// Operator actions that can contain a motion and applied a function to that
/// motion (delete, yank, change, etc.)
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    /// Change content of motion
    Change,
    /// Delete content of motion
    Delete,
}

impl Operator {
    /// Char that represents this operator. It is the char needed to apply the
    /// operator to the whole line.
    pub(super) const fn as_char(self) -> char {
        match self {
            Self::Change => 'c',
            Self::Delete => 'd',
        }
    }

    /// Adds the 1 or 2 go-to actions to fully define the current operator.
    pub(super) fn into_actions(self, scope: OperatorScope) -> Actions {
        match self {
            Self::Change => actions![Action::Delete(scope), Mode::Insert],
            Self::Delete => Action::Delete(scope).into(),
        }
    }
}

/// Scope that an operator can be applied to, usually denotated by a list of
/// goto actions. It can also be applied to a whole line.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorScope {
    /// Apply the operator on simply those actions
    Goto(GoToAction, Option<GoToAction>),
    /// Apply operator on the whole line
    WholeLine,
}

impl From<GoToAction> for OperatorScope {
    fn from(value: GoToAction) -> Self {
        Self::Goto(value, None)
    }
}

/// Pending action that only requires 1 character to form a goto action.
///
/// Can be combined with an [`Operator`] (change, delete, toggle case, etc.),
/// see [`OPending::OperatorAction`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CombinablePending {
    /// Find next char that is equal to...
    FindNext,
    /// Find next char that is equal to... and decrement
    FindNextDecrement,
    /// Find previous char that is equal to...
    FindPrevious,
    /// Find previous char that is equal to... and increment
    FindPreviousIncrement,
}
