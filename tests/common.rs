#[macro_export]
macro_rules! evt {
    ($name:ident) => {
        evt!(crossterm::event::KeyCode::$name)
    };
    ($name:literal) => {
        evt!(crossterm::event::KeyCode::Char($name))
    };
    ($name:expr) => {
        crossterm::event::Event::Key(crossterm::event::KeyEvent::from($name))
    };
}

#[macro_export]
macro_rules! do_evt {
    ($buffer:ident, $name:ident) => {
        $buffer.update(evt!($name))
    };
    ($buffer:ident, $name:literal) => {
        $buffer.update(evt!($name))
    };
}

#[macro_export]
macro_rules! buffer_tests {
    ($($name:ident: $keymaps:literal => $output:literal,)*) => {
            use vim_buffer::Buffer;
            $(
                #[test]
                fn $name() {
                    let mut buffer = Buffer::default();
                    buffer.update_from_string($keymaps).unwrap();
                    assert_eq!(
                        buffer.as_content(),
                        $output,
                        "Keys: \x1b[35m{}\x1b[0m",
                        $keymaps
                    );
                }
            )*
    };
}
