use std::collections::HashMap;
use std::sync::LazyLock;

use crossterm::event::KeyCode;

/// Defines the key interface
macro_rules! key {
    ($($name:ident: $code:expr,)*) => {
        /// Minimum and maximum length of the string representation of a key
        pub const LENGTHS:Bounds = {

            let keys = &[
                $(
                    stringify!($name)
                ),*
            ];

            let mut min = keys[0].len();
            let mut max = min;
            let mut idx = 1;
            #[expect(clippy::indexing_slicing, reason="compile time")]
            #[expect(clippy::else_if_without_else, reason="useless here")]
            while idx < keys.len() {
                let len = keys[idx].len();
                if len > max {
                    max = len;
                } else if len < min {
                    min = len
                };
                idx += 1;
            }

            Bounds { max, min }
        };


        static KEYS: LazyLock<HashMap<&'static str, KeyCode>> = LazyLock::new(|| {
            let mut map = HashMap::new();

            $(
             map.insert(
                 stringify!($name),
                 $code
                );
            )*

            map
        });
}}

key! {
Nul: KeyCode::Char('\0'),
BS: KeyCode::Backspace,
Tab: KeyCode::Tab,
NL: KeyCode::Char('\n'),
CR: KeyCode::Enter,
Return: KeyCode::Enter,
Enter: KeyCode::Enter,
Esc: KeyCode::Esc,
Space: KeyCode::Char(' '),
lt: KeyCode::Char('<'),
gt: KeyCode::Char('>'),
Bslash: KeyCode::Char('\\'),
Bar: KeyCode::Char('|'),
 Del: KeyCode::Delete,
// CSI,
// EOL,
// Ignore,
// NOP,
Up: KeyCode::Up,
Down: KeyCode::Down,
Left: KeyCode::Left,
Right: KeyCode::Right,
F1: KeyCode::F(1),
F2: KeyCode::F(2),
F3: KeyCode::F(3),
F4: KeyCode::F(4),
F5: KeyCode::F(5),
F6: KeyCode::F(6),
F7: KeyCode::F(7),
F8: KeyCode::F(8),
F9: KeyCode::F(9),
F10: KeyCode::F(10),
F11: KeyCode::F(11),
F12: KeyCode::F(12),
// Help,
// Undo,
// Find,
// Select,
// Insert,
Home: KeyCode::Home,
End: KeyCode::End,
PageUp: KeyCode::PageUp,
PageDown: KeyCode::PageDown,
// kUp,
// kDown,
// kLeft,
// kRight,
// kHome,
// kEnd,
// kOrigin,
// kPageUp,
// kPageDown,
// kDel,
// kPlus,
// kMinus,
// kMultiply,
// kDivide,
// kPoint,
// kComma,
// kEqual,
// kEnter,
// k0,
// k1,
// k2,
// k3,
// k4,
// k5,
// k6,
// k7,
// k8,
// k9
}

/// Bounds to store a maximum and minimum length
pub struct Bounds {
    /// Maximum possible length
    pub max: usize,
    /// Minimum possible length
    pub min: usize,
}

/// Converts a keycode to a string
pub fn build_named_key(name: &str) -> Option<KeyCode> {
    KEYS.get(name).copied()
}
