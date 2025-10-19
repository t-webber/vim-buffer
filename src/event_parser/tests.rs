use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::event_parser::chevron_parser::ChevronParsingError;
use crate::{EventParsingError, evt, parse_events};


fn mod_evt(ch: char, modifiers: KeyModifiers) -> Event {
    Event::Key(KeyEvent::new_with_kind(
        KeyCode::Char(ch),
        modifiers,
        KeyEventKind::Press,
    ))
}


#[test]
fn empty_chevron_group() {
    assert_eq!(
        parse_events("<>"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::MissingModifier
        )),
    );
}

#[test]
fn missing_char() {
    assert_eq!(
        parse_events("<C->"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::MissingChar
        )),
    );
}

#[test]
fn missing_char_and_hyphen() {
    assert_eq!(
        parse_events("<C>"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::MissingChar
        )),
    );
}

#[test]
fn missing_modifier_and_hyphen() {
    assert_eq!(
        parse_events("<c>"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::MissingModifier
        )),
    );
}


#[test]
fn too_many_separated_chars() {
    assert_eq!(
        parse_events("<C-a-b>"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::ExpectedChevron { got: '-' }
        ))
    );
}

#[test]
fn too_many_successive_chars() {
    assert_eq!(
        parse_events("<C-ab>"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::ExpectedChevron { got: 'b' }
        ))
    );
}


#[test]
fn missing_modifier() {
    assert_eq!(
        parse_events("<->"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::MissingModifier
        )),
    );
}

#[test]
fn missing_hypen() {
    assert_eq!(
        parse_events("<Ca>"),
        Err(EventParsingError::ChevronGroupError(
            ChevronParsingError::ExpectedChevronOrHyphen { got: 'a' }
        )),
    );
}


#[test]
fn control_s() {
    assert_eq!(
        parse_events("<C-s>"),
        Ok(vec![mod_evt('s', KeyModifiers::CONTROL)])
    );
}

#[test]
fn control_shift_s() {
    assert_eq!(
        parse_events("<C-S-s>"),
        Ok(vec![mod_evt('s', KeyModifiers::CONTROL | KeyModifiers::SHIFT)])
    );
}


#[test]
fn modifier_chars() {
    assert_eq!(
        parse_events("<C-C><C-M><C-S>"),
        Ok(vec![
            mod_evt('c', KeyModifiers::CONTROL),
            mod_evt('m', KeyModifiers::CONTROL),
            mod_evt('s', KeyModifiers::CONTROL),
        ])
    );
}


#[test]
fn alternate() {
    assert_eq!(
        parse_events("a<M-b>c<S-d>e"),
        Ok(vec![
            evt!('a'),
            mod_evt('b', KeyModifiers::META),
            evt!('c'),
            mod_evt('d', KeyModifiers::SHIFT),
            evt!('e')
        ])
    );
}

#[test]
fn mismatched_chevron() {
    assert_eq!(
        parse_events(">"),
        Err(EventParsingError::MismatchedClosingChevron),
    );
}
