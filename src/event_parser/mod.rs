/// Module that parses parts of the keyamps string that are between `<` and `>`
mod chevron_parser;
/// Main state for event parsing
mod state;

use crossterm::event::Event;

pub use crate::event_parser::chevron_parser::{
    ChevronGroupError, ModifiedKeyError
};
use crate::event_parser::state::EventParserState;
pub use crate::event_parser::state::EventParsingError;

/// Trait to define a parsing methodology
trait EventParser {
    /// Errors that can be returned by the parsing
    type Error;

    /// Parses one more char with the given state.
    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error>;
}

/// Transforms a string input of keymaps into a list of events.
///
/// Only chars are supported for now.
///
/// # Errors
///
/// Returns a [`EventParsingError`] when the input has an invalid format.
pub fn parse_events(keymaps: &str) -> Result<Vec<Event>, EventParsingError> {
    let mut parser = EventParserState::default();
    keymaps.chars().filter_map(|ch| parser.parse_char(ch).transpose()).collect()
}
