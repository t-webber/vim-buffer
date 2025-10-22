use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::event_parser::EventParser;
use crate::event_parser::chevron_parser::{
    ChevronGroupError, ChevronGroupParser
};

/// Current state of the parser, to know what char to expect next.
#[derive(Default)]
pub enum EventParserState {
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
        Event::Key(KeyEvent::new(
            KeyCode::Char(ch),
            if ch.is_uppercase() {
                KeyModifiers::SHIFT
            } else {
                KeyModifiers::NONE
            },
        ))
    }

    /// Actions to open a new chevron group
    const fn open_chevron(&mut self) {
        *self = Self::InChevron(ChevronGroupParser::new());
    }
}

impl EventParser for EventParserState {
    type Error = EventParsingError;

    /// Minimum length possible
    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error> {
        match self {
            Self::None if ch == '<' => {
                self.open_chevron();
                Ok(None)
            }
            Self::None if ch == '>' =>
                Err(EventParsingError::MismatchedClosingChevron),
            Self::None => Ok(Some(Self::char_event(ch))),
            Self::InChevron(chevron_parser) =>
                match chevron_parser.parse_char(ch) {
                    Ok(Some(event)) => {
                        *self = Self::None;
                        Ok(Some(event))
                    }
                    Ok(None) => Ok(None),
                    Err(err) => Err(EventParsingError::ChevronGroup(err)),
                },
        }
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
    ChevronGroup(ChevronGroupError),
    /// The provided named key isn't a valid key name.
    InvalidNamedKey,
    /// Found a closing `>` without a corresponding `<`.
    MismatchedClosingChevron,
}
