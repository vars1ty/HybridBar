use crate::{
    structures::Align,
    ui::{self, VEC},
    widget::HWidget,
};
use gtk::{traits::*, *};
use std::fmt::Display;

/// Creates a new label widget.
pub struct LabelWidget {
    pub tooltip: String,
    pub text: String,
    pub command: String,
    pub label: Label,
}

// For VEC to work.
unsafe impl Send for LabelWidget {}
unsafe impl Sync for LabelWidget {}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for LabelWidget {
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box) {
        self.label.set_widget_name(&name);
        // 0.2.7: Support for tooltips
        self.label.set_tooltip_markup(Some(&self.tooltip));

        ui::add_and_align(&self.label, align, left, centered, right);
        log!(format!("Added a new label widget named '{}'", name));
        VEC.lock()
            .expect("[ERROR] Cannot access ui::VEC!\n")
            .push(self);
    }

    fn update_label(&self, new_content: &(impl Display + Clone)) {
        // 0.2.7: Support for markup as long as the command is empty.
        // It doesn't support markup with commands because some strings may cause GTK to mistreat
        // it, which I may fix in the future.
        if self.command.is_empty() {
            self.label.set_markup(&new_content.to_string());
        } else {
            self.label.set_text(&new_content.to_string());
        }
    }
}
