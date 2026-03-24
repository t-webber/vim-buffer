/// Register values and handling
#[derive(Debug, Default)]
pub struct Registers(String);

impl Registers {
    /// Returns the value held by a register
    pub fn get(&self) -> &str {
        &self.0
    }

    /// Insert a new value at the given register
    pub fn insert(&mut self, value: &str) {
        value.clone_into(&mut self.0);
    }
}
