use crate::{debug::log, structures::Align, ui::VEC, widget::HWidget};
use gtk::{traits::*, *};

/// Creates a new label widget.
pub struct LabelWidget {
    pub name: String,
    pub text: String,
    pub command: String,
    pub label: Label,
}

// For VEC to work.
unsafe impl Send for LabelWidget {}
unsafe impl Sync for LabelWidget {}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for LabelWidget {
    fn add(self, align: Align, left: &Box, centered: &Box, right: &Box) {
        self.label.set_widget_name(&self.name);

        // Align and add the widget
        match align {
            Align::LEFT => left.add(&self.label),
            Align::CENTERED => centered.add(&self.label),
            Align::RIGHT => right.add(&self.label),
        }

        log(format!("Added a new label widget named '{}'", self.name));
        VEC.lock()
            .expect("[ERROR] Cannot access ui::VEC!\n")
            .push(self);
    }
}
