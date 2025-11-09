use vim_buffer::Buffer;

mod common;

#[test]
fn normal_u_empty() {
    let mut buffer = Buffer::default();
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "");
}

#[test]
fn normal_u_word() {
    let mut buffer = Buffer::default();
    do_evt!(buffer, 'i');
    do_evt!(buffer, 'a');
    do_evt!(buffer, 'b');
    do_evt!(buffer, 'c');
    do_evt!(buffer, Esc);
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "");
}

#[test]
fn normal_u_2_words() {
    let mut buffer = Buffer::default();
    do_evt!(buffer, 'i');
    do_evt!(buffer, 'a');
    do_evt!(buffer, 'b');
    do_evt!(buffer, 'c');
    do_evt!(buffer, Esc);
    do_evt!(buffer, 'i');
    buffer.update(&evt!('d'));
    do_evt!(buffer, 'e');
    do_evt!(buffer, 'f');
    do_evt!(buffer, Esc);
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "abc");
}

#[test]
fn normal_u_x() {
    let mut buffer = Buffer::from("abcdefghijk");
    do_evt!(buffer, 'x');
    do_evt!(buffer, 'x');
    do_evt!(buffer, 'x');
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "cdefghijk");
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "bcdefghijk");
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "abcdefghijk");
}

#[test]
fn normal_u_normal_u() {
    let mut buffer = Buffer::from("");

    buffer.update_from_string("iabc<Esc>").unwrap();
    buffer.update_from_string("idef<Esc>").unwrap();
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "abc");

    buffer.update_from_string("ighi<Esc>").unwrap();
    do_evt!(buffer, 'u');
    assert_eq!(buffer.as_content(), "abc");
}
