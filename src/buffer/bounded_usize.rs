/// `usize` bounded by a value, for safe incrementation and decrementation.
#[derive(Default, Debug)]
pub struct BoundedUsize {
    /// Maximum value the `value` field can hold.
    max_value: usize,
    /// Inner value of the [`BoundedUsize`]
    value: usize,
}

impl BoundedUsize {
    /// Returns the inner value of the [`BoundedUsize`]
    pub const fn as_value(&self) -> usize {
        self.value
    }

    /// Decrements the inner value and the maximum value.
    pub const fn decrement(&mut self) -> bool {
        let old = self.value;
        self.value = self.value.saturating_sub(1);
        old != self.value
    }

    /// Decrements the inner value and the maximum value.
    pub const fn decrement_with_capacity(&mut self) {
        self.max_value = self.max_value.saturating_sub(1);
        self.value = self.value.saturating_sub(1);
    }

    /// Increments the inner value and the maximum value.
    pub const fn increment(&mut self) -> bool {
        let old = self.value;
        let sum = self.value.saturating_add(1);
        self.value = if sum < self.max_value { sum } else { self.max_value };
        old != self.value
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

    /// Changes the maximum value of the [`BoundedUsize`], and revalidates the
    /// cursor to ensure it is still in bounds.
    pub const fn set_max(&mut self, max: usize) {
        self.max_value = max;
        self.set(self.value);
    }

    /// Sets the cursor to the maximum position, i.e. at the end of the buffer.
    pub const fn set_to_max(&mut self) {
        self.value = self.max_value;
    }

    /// Creates a new [`BoundedUsize`] with a given maximum value.
    pub const fn with_capacity(max_value: usize) -> Self {
        Self { max_value, value: 0 }
    }
}
