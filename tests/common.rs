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
        $buffer.update(&evt!($name))
    };
    ($buffer:ident, $name:literal) => {
        $buffer.update(&evt!($name))
    };
}
