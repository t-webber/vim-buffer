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
///
/// # Examples
///
/// ```
/// use vim_buffer::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// use vim_buffer::parse_events;
///
/// let events = parse_events("i<CR><C-S-M-A>H").unwrap();
/// assert_eq!(
///     events[0].as_key_press_event().unwrap().code,
///     KeyCode::Char('i')
/// );
/// assert_eq!(events[1].as_key_press_event().unwrap().code, KeyCode::Enter);
/// assert_eq!(
///     events[2].as_key_press_event().unwrap(),
///     KeyEvent::new(
///         KeyCode::Char('A'),
///         KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT
///     )
/// );
/// assert_eq!(
///     events[3].as_key_press_event().unwrap().modifiers,
///     KeyModifiers::SHIFT
/// );
/// ```
pub fn parse_events(keymaps: &str) -> Result<Vec<Event>, EventParsingError> {
    let mut parser = EventParserState::default();
    keymaps.chars().filter_map(|ch| parser.parse_char(ch).transpose()).collect()
}
