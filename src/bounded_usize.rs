/// `usize` bounded by a value, for safe incrementation and decrementation.
#[derive(Default, Debug)]
pub struct BoundedUsize {
    /// Maximum value the `value` field can hold.
    max_value: usize,
    /// Inner value of the [`BoundedUsize`]
    value:     usize,
}

impl BoundedUsize {
    /// Returns the inner value of the [`BoundedUsize`]
    pub const fn as_value(&self) -> usize {
        self.value
    }

    /// Decrements the inner value and the maximum value.
    pub const fn decrement(&mut self, amount: usize) {
        self.value = self.value.saturating_sub(amount);
    }

    /// Decrements the inner value and the maximum value.
    pub const fn decrement_with_capacity(&mut self) {
        self.max_value = self.max_value.saturating_sub(1);
        self.value = self.value.saturating_sub(1);
    }

    /// Increments the inner value and the maximum value.
    pub const fn increment(&mut self, amount: usize) {
        self.value = self.value.saturating_add(amount);
    }

    /// Increments the inner value and the maximum value.
    ///
    /// # Panics
    ///
    /// Panics if `max_value` is [`usize::MAX`]
    #[expect(clippy::arithmetic_side_effects, reason = "documented")]
    pub const fn increment_with_capacity_unchecked(&mut self) {
        debug_assert!(self.max_value < usize::MAX, "Value too large");
        self.max_value += 1;
        self.value += 1;
    }

    /// Tries to set the cursor to the given index, and defaults to the maximum
    /// value if it overflows.
    pub const fn set(&mut self, value: usize) {
        self.value =
            if value < self.max_value { value } else { self.max_value };
    }
}
