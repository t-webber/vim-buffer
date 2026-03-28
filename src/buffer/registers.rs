/// Associates key name to value
macro_rules! key {
    ($name:ident : $value:literal) => {
        const $name: usize = Registers::to_key($value, true).unwrap().0;
        const _: () = assert!(!Registers::to_key($value, true).unwrap().1);
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
            |ch| self.0[Self::to_key(ch, false)?.0].as_deref(),
        )
    }

    /// Insert a new value at the given register
    pub fn insert(
        &mut self,
        value: &str,
        is_delete: bool,
        reg: Option<char>,
    ) -> bool {
        if reg == Some('_') {
            return true;
        }
        self.insert_key(DEFAULT, value, false);
        if is_delete {
            self.insert_key(DELETE, value, false);
        } else {
            self.insert_key(COPY, value, false);
        }
        reg.is_none_or(|ch| {
            Self::to_key(ch, true).is_some_and(|(key, append)| {
                self.insert_key(key, value, append);
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
    fn insert_key(&mut self, key: usize, value: &str, append: bool) {
        if let Some(old) = &mut self.0[key] {
            if append {
                old.push_str(value);
            } else {
                value.clone_into(old);
            }
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
    const fn to_key(reg: char, edit: bool) -> Option<(usize, bool)> {
        let key = match reg {
            '0'..='9' => reg as usize - '0' as usize,
            'a'..='z' => reg as usize - 'a' as usize + 10,
            'A'..='Z' => return Some((reg as usize - 'A' as usize + 10, true)),
            '"' => 36,
            '-' => 37,
            '=' => 38,
            _ if edit => return None,
            '%' => 39,
            '#' => 40,
            ':' => 41,
            '/' => 42,
            _ => return None,
        };
        Some((key, false))
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self([const { None }; LEN])
    }
}
