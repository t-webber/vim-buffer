#![allow(dead_code, reason = "todo")]

/// Fixed capacity vec stored on the stack
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Array<T: Copy, const N: usize>([Option<T>; N], usize);

impl<T: Copy, const N: usize> Array<T, N> {
    /// Returns the only element if it only contains one element
    pub const fn as_lone(self) -> Option<T> {
        if self.1 == 1 { self.0[0] } else { None }
    }

    /// Returns the current length of the [`Array`].
    pub const fn len(&self) -> usize {
        self.1
    }

    /// Creates a new [`Array`] from a list of values
    #[expect(
        clippy::arithmetic_side_effects,
        clippy::indexing_slicing,
        reason = "index in bounds"
    )]
    pub const fn maybe_from(values: &[T]) -> Option<Self> {
        let mut this = Self::new();
        let mut idx = 0;
        while idx < values.len() {
            if !this.push(values[idx]) {
                return None;
            }
            idx += 1;
        }
        Some(this)
    }

    /// Creates a new empty [`Array`]
    pub const fn new() -> Self {
        Self([None; N], 0)
    }

    /// Pops the last added value if the [`Array`] is not empty.
    #[expect(clippy::indexing_slicing, reason = "index is in bound")]
    #[expect(clippy::arithmetic_side_effects, reason = "self.1-1>=0")]
    pub const fn pop(&mut self) -> Option<T> {
        if self.1 == 0 {
            None
        } else {
            self.1 -= 1;
            self.0[self.1]
        }
    }

    /// Pushes a value into the [`Array`]
    ///
    /// # Returns
    ///
    /// - `true` if the push was successful.
    /// - `false` if the array was full, in that case, the value was not added.
    #[must_use]
    #[expect(clippy::indexing_slicing, reason = "index is in bound")]
    #[expect(clippy::arithmetic_side_effects, reason = "self.1+1<=N")]
    pub const fn push(&mut self, value: T) -> bool {
        if self.1 == N {
            false
        } else {
            self.0[self.1] = Some(value);
            self.1 += 1;
            true
        }
    }
}

impl<const N: usize> Array<char, N> {
    /// Concatenates the chars of the array into a string
    pub fn concat(&self) -> String {
        self.0.iter().filter_map(|x| *x).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::array::Array;

    #[test]
    fn pop_empty() {
        let mut arr: Array<u32, 4> = Array::new();
        assert_eq!(arr.pop(), None);
    }

    #[test]
    fn pop() {
        let mut arr: Array<u32, 4> = Array::new();
        assert!(arr.push(0));
        assert!(arr.push(1));
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.pop(), Some(0));
        assert_eq!(arr.pop(), None);
    }

    #[test]
    fn push_full() {
        let mut arr: Array<u32, 1> = Array::new();
        assert!(arr.push(0));
        assert!(!arr.push(1));
        assert_eq!(arr.pop(), Some(0));
    }

    #[test]
    fn fmt() {
        let mut arr: Array<char, 1024> = Array::new();
        assert!(arr.push('1'));
        assert!(arr.push('3'));
        assert!(arr.push('2'));
        assert_eq!(&arr.concat(), "132");
    }

    #[test]
    fn from() {
        let mut arr: Array<u32, 4> = Array::new();
        assert!(arr.push(31));
        assert!(arr.push(18));
        assert_eq!(Some(arr), Array::<u32, 4>::maybe_from(&[31, 18]));
    }

    #[test]
    fn len() {
        let mut arr: Array<u32, 4> = Array::new();
        assert_eq!(arr.len(), 0);
        assert!(arr.push(0));
        assert_eq!(arr.len(), 1);
        assert!(arr.push(1));
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn lone() {
        let mut arr: Array<u32, 4> = Array::new();
        assert_eq!(arr.as_lone(), None);
        assert!(arr.push(0));
        assert_eq!(arr.as_lone(), Some(0));
        assert!(arr.push(1));
        assert_eq!(arr.as_lone(), None);
    }

    #[test]
    fn maybe_from() {
        assert_eq!(Array::<u32, 1>::maybe_from(&[1, 2, 3, 4]), None);
    }
}
