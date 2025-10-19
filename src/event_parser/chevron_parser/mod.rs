/// Defines the valid modifiers that can be used within a chevron group.
mod char_modifier;
/// Defines logic to push into the modifiers, and get back the last one if
/// needed.
mod non_empty_modifier;
/// Defines the state to parse the chevron groups
mod state;

use crossterm::event::Event;

use crate::event_parser::EventParser;
use crate::event_parser::chevron_parser::state::ChevronGroupParsingState;


/// Parses one chevron group.
///
/// Chevron groups are used to denote keys with keymodifiers.
pub struct ChevronGroupParser(ChevronGroupParsingState);

impl ChevronGroupParser {
    /// Returns a default [`ChevronGroupParser`] for a new chevron group.
    pub const fn new() -> Self {
        Self(ChevronGroupParsingState::None)
    }
}

impl EventParser for ChevronGroupParser {
    type Error = ChevronParsingError;

    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error> {
        self.0.parse_char(ch)
    }
}


/// Errors that may occur whilst parsing a chevron group.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChevronParsingError {
    /// Invalid character in the current context: expected a `>`
    ExpectedChevron {
        /// But got this character
        got: char,
    },
    /// Invalid character in the current context: expected a `>` or a '-'
    ExpectedChevronOrHyphen {
        /// But got this character
        got: char,
    },
    /// The chevron group is missing a char
    MissingChar,
    /// The chevron group is missing a modifier.
    MissingModifier,
}
