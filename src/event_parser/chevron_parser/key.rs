use std::collections::HashMap;
use std::sync::LazyLock;

use crossterm::event::KeyCode;

/// Defines the key interface
macro_rules! key {
    ($($name:ident: $code:ident,)*) => {
        /// Minimum and maximum length of the string represetation of a key
        pub const LENGTHS:Bounds = {

            let keys = &[
                $(
                    stringify!($name)
                ),*
            ];

            let mut min = keys[0].len();
            let mut max = min;
            let mut i = 1;
            #[expect(clippy::indexing_slicing, reason="compile time")]
            #[expect(clippy::else_if_without_else, reason="useless here")]
            while i < keys.len() {
                let len = keys[i].len();
                if len > max {
                    max = len;
                } else if len < min {
                    min = len
                };
                i += 1;
            }

            Bounds { max, min }
        };


        static KEYS: LazyLock<HashMap<&'static str, KeyCode>> = LazyLock::new(|| {
            let mut map = HashMap::new();

            $(
             map.insert(
                 stringify!($name),
                 KeyCode::$code
                );
            )*

            map
        });
}}

key! {
// Nul,
BS: Backspace,
Tab: Tab,
// NL,
CR: Enter,
Return: Enter,
Enter: Enter,
Esc: Esc,
// Space,
// lt,
// Bslash,
// Bar,
 Del: Delete,
// CSI,
// EOL,
// Ignore,
// NOP,
Up: Up,
Down: Down,
Left: Left,
Right: Right,
// F1,
// F2,
// F3,
// F4,
// F5,
// F6,
// F7,
// F8,
// F9,
// F10,
// F11,
// F12,
// Help,
// Undo,
// Find,
// Select,
// Insert,
Home: Home,
End: End,
PageUp: PageUp,
PageDown: PageDown,
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
pub fn build_named_key(name: &[u8]) -> Option<KeyCode> {
    if let Ok(name_str) = str::from_utf8(name)
        && let Some(event) = KEYS.get(name_str)
    {
        Some(*event)
    } else {
        None
    }
}
