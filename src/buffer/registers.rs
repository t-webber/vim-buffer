/// Associates key name to value
macro_rules! key {
    ($name:ident : $value:literal) => {
        #[expect(clippy::as_conversions, reason = "compile time")]
        const $name: usize = $value as usize;
        const _: () = assert!($name < 128);
    };
}

key!(DEFAULT: '"');
key!(DELETE: '-');
key!(COPY: '0');

/// Register values and handling
#[derive(Debug)]
pub struct Registers([Option<String>; 128]);

impl Registers {
    /// Returns the value held by a register
    pub fn get(&self) -> Option<&str> {
        self.0[DEFAULT].as_deref()
    }

    /// Insert a new value at the given register
    pub fn insert(&mut self, value: &str, is_delete: bool) {
        self.insert_key(DEFAULT, value);
        if is_delete {
            self.insert_key(DELETE, value);
        } else {
            self.insert_key(COPY, value);
        }
    }

    /// Insert a new value at the given register
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
}

impl Default for Registers {
    fn default() -> Self {
        Self([const { None }; 128])
    }
}
