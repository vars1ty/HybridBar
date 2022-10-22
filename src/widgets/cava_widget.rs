use crate::{structures::Align, ui, widget::HWidget};
use gtk::{traits::*, *};
use std::fmt::Display;

/// Creates a new label widget.
pub struct CavaWidget {
    pub label: Label,
}

unsafe impl Send for CavaWidget {}
unsafe impl Sync for CavaWidget {}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for CavaWidget {
    fn add(self, align: Align, left: &Box, centered: &Box, right: &Box) {
        ui::add_and_align(&self.label, align, left, centered, right);
        ui::CAVA_INSTANCES
            .lock()
            .expect("[ERROR] Couldn't access ui::CAVA_INSTANCES!\n")
            .push(self)
    }

    fn update_label(&self, new_content: &(impl Display + Clone)) {
        self.label.set_text(&new_content.to_string())
    }
}
