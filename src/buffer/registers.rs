/// Name of the registers
mod keys {
    #![expect(clippy::as_conversions, reason = "compile time")]

    /// Default register that is always updated
    pub const DEFAULT: usize = '"' as usize;
}

use keys::DEFAULT;

/// Register values and handling
#[derive(Debug)]
pub struct Registers([Option<String>; 128]);

impl Registers {
    /// Returns the value held by a register
    pub fn get(&self) -> Option<&str> {
        self.0[DEFAULT].as_deref()
    }

    /// Insert a new value at the given register
    pub fn insert(&mut self, value: &str) {
        if let Some(old) = &mut self.0[DEFAULT] {
            value.clone_into(old);
        } else {
            self.0[DEFAULT] = Some(value.to_owned());
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self([const { None }; 128])
    }
}
