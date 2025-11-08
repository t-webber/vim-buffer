/// Holds the history of the buffer, with the following invariant: there are
/// never 2 successive entries that are equal.
#[derive(Debug)]
pub struct History(Vec<Box<str>>);

impl History {
    /// Removes and returns the last element of the history that is different
    /// from the given value.
    pub fn pop_different_from(&mut self, reference: &str) -> Option<Box<str>> {
        while let Some(last) = self.0.pop() {
            if *last != *reference {
                return Some(last);
            }
        }
        None
    }

    /// Pushes a new entry in the history list, if it is different from the last
    /// entry.
    ///
    /// # Returns
    ///
    /// `true` iff it was successfully pushed to the history.
    pub fn push_if_different(&mut self, value: &str) -> bool {
        let differs = self.0.last().is_none_or(|last| **last != *value);
        if differs {
            self.0.push(Box::from(value));
        }
        differs
    }

    /// Returns a [`History`] with an initial value.
    pub fn with_initial_value(value: Box<str>) -> Self {
        Self(vec![value])
    }
}

impl Default for History {
    fn default() -> Self {
        Self::with_initial_value(Box::from(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_entries(hist: &History, entries: &[&str]) {
        for (idx, entry) in entries.iter().enumerate() {
            assert_eq!(Some(*entry), hist.0.get(idx).map(|word| &**word));
        }
    }

    #[test]
    fn empty() {
        assert_eq!(
            History::default().pop_different_from("a").as_deref(),
            Some("")
        );
    }

    #[test]
    fn multiple_entries() {
        let mut hist = History::default();
        for word in ["first", "second", "third"] {
            assert!(hist.push_if_different(word));
        }
        check_entries(&hist, &["", "first", "second", "third"]);
    }

    #[test]
    fn duplicate_push() {
        let mut hist = History::default();
        for word in ["", "first", "second", "second", "first", "third", "third"]
        {
            hist.push_if_different(word);
        }
        check_entries(&hist, &["", "first", "second", "first", "third"]);
    }

    #[test]
    fn pop_equal() {
        let mut hist = History::default();
        for word in ["first", "second", "third"] {
            assert!(hist.push_if_different(word));
        }
        assert_eq!(hist.pop_different_from("third"), Some("second".into()));
        assert_eq!(hist.pop_different_from("second"), Some("first".into()));
    }

    #[test]
    fn pop_empty() {
        let mut hist = History::default();
        assert_eq!(hist.pop_different_from(""), None);
        assert_eq!(hist.pop_different_from("!"), None);
        assert_eq!(hist.pop_different_from(""), None);
    }
}
