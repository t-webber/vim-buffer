use vim_buffer::Buffer;

#[test]
fn start_empty() {
    let mut buffer = Buffer::default();
    for ch in ['%', '#', ':', '/', '"', '_', '='] {
        buffer.update_from_string(&format!(r#""{ch}p"#)).unwrap();
    }
    assert_eq!(buffer.as_content(), "");
}
