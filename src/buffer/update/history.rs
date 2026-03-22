use crate::{Buffer, Mode};

impl Buffer {
    /// Undos the latest undo
    pub(super) fn redo(&mut self) -> bool {
        if let Some(previous) = self.history.redo() {
            self.content = previous.to_owned().into_string();
            self.cursor.set_max(self.len());
            true
        } else {
            false
        }
    }

    /// Adds the current buffer to the history, if it is different from the
    /// last entry.
    pub(crate) fn save_to_history(&mut self) {
        if matches!(self.as_mode(), Mode::Normal) {
            self.history.save(&self.content);
        }
    }

    /// Pops from history the first different  buffer value
    pub(super) fn undo(&mut self) -> bool {
        if let Some(previous) = self.history.undo() {
            self.content = previous.to_owned().into_string();
            self.cursor.set_max(self.len());
            true
        } else {
            false
        }
    }
}
