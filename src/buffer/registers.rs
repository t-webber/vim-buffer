/// Associates key name to value
macro_rules! key {
    ($name:ident : $value:literal) => {
        const $name: usize = Registers::to_key($value, true).unwrap();
    };
}

key!(DEFAULT: '"');
key!(DELETE: '-');
key!(COPY: '0');

/// Maximum number of registers
const LEN: usize = 43;

#[doc = include_str!("registers.md")]
#[derive(Debug)]
pub struct Registers([Option<String>; LEN]);

impl Registers {
    /// Returns the value held by a register
    #[expect(clippy::indexing_slicing, reason = "to_key returns valid index")]
    pub fn get(&self, reg: Option<char>) -> Option<&str> {
        reg.map_or_else(
            || self.0[DEFAULT].as_deref(),
            |ch| self.0[Self::to_key(ch, false)?].as_deref(),
        )
    }

    /// Insert a new value at the given register
    pub fn insert(
        &mut self,
        value: &str,
        is_delete: bool,
        reg: Option<char>,
    ) -> bool {
        self.insert_key(DEFAULT, value);
        if is_delete {
            self.insert_key(DELETE, value);
        } else {
            self.insert_key(COPY, value);
        }
        reg.is_none_or(|ch| {
            Self::to_key(ch, true).is_some_and(|key| {
                self.insert_key(key, value);
                true
            })
        })
    }

    /// Insert a new value at the given register key
    ///
    /// # Panics
    ///
    /// If key >= 128.
    #[expect(clippy::indexing_slicing, reason = "keys are less than 128")]
    fn insert_key(&mut self, key: usize, value: &str) {
        if let Some(old) = &mut self.0[key] {
            value.clone_into(old);
        } else {
            self.0[key] = Some(value.to_owned());
        }
    }

    /// Returns the key number for the given char register
    #[expect(
        clippy::as_conversions,
        clippy::arithmetic_side_effects,
        reason = "explicit checks"
    )]
    const fn to_key(reg: char, edit: bool) -> Option<usize> {
        Some(match reg {
            '0'..='9' => reg as usize - '0' as usize,
            'a'..='z' => reg as usize - 'a' as usize + 10,
            '"' => 36,
            '-' => 37,
            '=' => 38,
            _ if edit => return None,
            '%' => 39,
            '#' => 40,
            ':' => 41,
            '/' => 42,
            _ => return None,
        })
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self([const { None }; LEN])
    }
}
