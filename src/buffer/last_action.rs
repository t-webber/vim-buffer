use crate::buffer::keymaps::Action;
use crate::{Buffer, Mode};

/// Last action done on the buffer, used by the `.` keymap
#[derive(Debug, Default)]
pub struct LastAction {
    /// List of actions to be performed
    actions: Vec<Action>,
    /// On what buffer mode they should be performed
    mode: Mode,
}

impl LastAction {
    /// Performs the last action on the given buffer.
    pub fn perform(&self, buffer: &mut Buffer) -> bool {
        let old_mode = buffer.as_mode();
        if buffer.update_once(self.mode.into())
            && self.actions.iter().all(|action| buffer.update_once(*action))
            && buffer.update_once(old_mode.into())
        {
            buffer.save_to_history();
            true
        } else {
            false
        }
    }

    /// Updates the [`LastAction`] with a list of actions.
    pub fn update(&mut self, actions: Vec<Action>, mode: Mode) {
        if actions == [Action::Repeat]
            || actions.iter().all(|action| {
                matches!(
                    action,
                    Action::GoTo(_)
                        | Action::SelectMode(_)
                        | Action::ClearUndoReplace
                )
            })
        {
            return;
        }
        if mode != Mode::Normal && self.mode == mode {
            self.actions.extend(actions);
        } else {
            self.actions = actions;
        }
        self.mode = mode;
    }
}
