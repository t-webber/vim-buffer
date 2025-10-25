use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    ChevronGroupError, EventParsingError, ModifiedKeyError, evt, parse_events
};


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
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::ExpectedLetter { got: '>' }
        ))
    );
}

#[test]
fn missing_char() {
    assert_eq!(
        parse_events("<C->"),
        Err(EventParsingError::ChevronGroup(ChevronGroupError::ModifiedKey(
            ModifiedKeyError::MissingChar
        ))),
    );
}

#[test]
fn missing_char_and_hyphen() {
    assert_eq!(
        parse_events("<C>"),
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::SingleCharGroup
        )),
    );
}

#[test]
fn missing_modifier_and_hyphen() {
    assert_eq!(
        parse_events("<c>"),
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::SingleCharGroup
        )),
    );
}


#[test]
fn too_many_separated_chars() {
    assert_eq!(
        parse_events("<C-a-b>"),
        Err(EventParsingError::ChevronGroup(ChevronGroupError::ModifiedKey(
            ModifiedKeyError::ExpectedChevron { got: '-' }
        )))
    );
}

#[test]
fn too_many_successive_chars() {
    assert_eq!(
        parse_events("<C-ab>"),
        Err(EventParsingError::ChevronGroup(ChevronGroupError::ModifiedKey(
            ModifiedKeyError::ExpectedChevron { got: 'b' }
        )))
    );
}


#[test]
fn missing_modifier() {
    assert_eq!(
        parse_events("<->"),
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::ExpectedLetter { got: '-' }
        )),
    );
}

#[test]
fn missing_hypen() {
    assert_eq!(
        parse_events("<Ca>"),
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::InvalidNamedKey
        )),
    );
}


#[test]
fn true_meta_t() {
    assert_eq!(
        parse_events("<T-T>"),
        Ok(vec![mod_evt('t', KeyModifiers::META)])
    );
}

#[test]
fn control_shift_alt_s() {
    assert_eq!(
        parse_events("<C-S-A>"),
        Ok(vec![mod_evt('A', KeyModifiers::CONTROL | KeyModifiers::SHIFT)])
    );
}


#[test]
fn modifier_chars() {
    assert_eq!(
        parse_events("<C-C><A-M><C-S><T-D>"),
        Ok(vec![
            mod_evt('c', KeyModifiers::CONTROL),
            mod_evt('m', KeyModifiers::ALT),
            mod_evt('s', KeyModifiers::CONTROL),
            mod_evt('d', KeyModifiers::META),
        ])
    );
}

#[test]
fn control_x() {
    assert_eq!(
        parse_events("<C-X>"),
        Ok(vec![mod_evt('x', KeyModifiers::CONTROL)])
    );
}

#[test]
fn alternate() {
    assert_eq!(
        parse_events("a<M-b>c<D-d>e"),
        Ok(vec![
            evt!('a'),
            mod_evt('b', KeyModifiers::ALT),
            evt!('c'),
            mod_evt('d', KeyModifiers::SUPER),
            evt!('e')
        ])
    );
}

#[test]
fn backspace() {
    assert_eq!(parse_events("<BS>"), Ok(vec![evt!(Backspace),]));
}

#[test]
fn enter_return() {
    assert_eq!(parse_events("<Return>"), Ok(vec![evt!(Enter),]));
}


#[test]
fn mismatched_chevron() {
    assert_eq!(
        parse_events(">"),
        Err(EventParsingError::MismatchedClosingChevron),
    );
}

#[test]
fn invalid_modifier() {
    assert_eq!(
        parse_events("<c-s>"),
        Err(EventParsingError::ChevronGroup(ChevronGroupError::ModifiedKey(
            ModifiedKeyError::InvalidModifier('c')
        ))),
    );
}

#[test]
fn non_u8_char() {
    assert_eq!(
        parse_events("<a\u{fff}>"),
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::InvalidNamedKey
        )),
    );
}


#[test]
fn to_many_letters() {
    assert_eq!(
        parse_events("<C-Cs>"),
        Err(EventParsingError::ChevronGroup(ChevronGroupError::ModifiedKey(
            ModifiedKeyError::ExpectedChevronOrHyphen { got: 's' }
        ))),
    );
}

#[test]
fn key_too_long() {
    assert_eq!(
        parse_events("<Caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa>"),
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::InvalidNamedKey
        )),
    );
}


#[test]
fn shift_without_modifiers() {
    let event = parse_events("S")
        .unwrap()
        .first()
        .unwrap()
        .as_key_press_event()
        .unwrap();
    assert_eq!(event.modifiers, KeyModifiers::SHIFT);
}

#[test]
fn named_keys() {
    assert_eq!(
        parse_events("<Up><Down><Left><Right><Tab><BS><CR><Esc>"),
        Ok(vec![
            evt!(Up),
            evt!(Down),
            evt!(Left),
            evt!(Right),
            evt!(Tab),
            evt!(Backspace),
            evt!(Enter),
            evt!(Esc)
        ])
    );
}
