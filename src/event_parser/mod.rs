/// Module that parses parts of the keyamps string that are between `<` and `>`
mod chevron_parser;

#[cfg(test)]
mod tests;

use core::result;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::event_parser::chevron_parser::ChevronGroupParser;
pub use crate::event_parser::chevron_parser::ChevronParsingError;


/// Current state of the parser, to know what char to expect next.
#[derive(Default)]
enum EventParserState {
    /// A chevron group was opened, handing the parsing over to
    /// [`ChevronGroupParser`]
    InChevron(ChevronGroupParser),
    /// Nothing special, waiting for any char.
    #[default]
    None,
}

impl EventParserState {
    /// Push a normal char to the list of events.
    const fn char_event(ch: char) -> Event {
        Event::Key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE))
    }

    /// Actions to open a new chevron group
    const fn open_chevron(&mut self) {
        *self = Self::InChevron(ChevronGroupParser::new());
    }

    /// Parses one more char with the given state.
    const fn parse_char(&mut self, ch: char) -> Result<Option<Event>> {
        match self {
            Self::None if ch == '<' => self.open_chevron(),
            Self::None if ch == '>' =>
                return Err(EventParsingError::MismatchedClosingChevron),
            Self::None => return Ok(Some(Self::char_event(ch))),
            Self::InChevron(chevron_parser) =>
                return match chevron_parser.parse_char(ch) {
                    Ok(Some(event)) => {
                        *self = Self::None;
                        Ok(Some(event))
                    }
                    Ok(None) => Ok(None),
                    Err(err) => Err(EventParsingError::ChevronGroupError(err)),
                },
        }
        Ok(None)
    }
}

/// Errors that may occur when trying to parse a keymaps string into a list of
/// key events.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventParsingError {
    /// Error occurred while parsing a chevron group
    ///
    /// A chevron group is anything between `<` and `>`.
    ChevronGroupError(ChevronParsingError),
    /// Found a closing `>` without a corresponding `<`.
    MismatchedClosingChevron,
}

type Result<T> = result::Result<T, EventParsingError>;

/// Transforms a string input of keymaps into a list of events.
///
/// Only chars are supported for now.
///
/// # Errors
///
/// Returns a [`EventParsingError`] when the input has an invalid format.
pub fn parse_events(keymaps: &str) -> Result<Vec<Event>> {
    let mut parser = EventParserState::default();
    keymaps.chars().filter_map(|ch| parser.parse_char(ch).transpose()).collect()
}
