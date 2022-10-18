use crate::{debug::log, structures::Align, widget::HWidget};
use gtk::{traits::*, *};

/// Creates a new basic spacing widget.
pub struct SpacingWidget {
    pub name: String,
    pub spacing_start: i32,
    pub spacing_end: i32,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for SpacingWidget {
    fn add(self, align: Align, left: &Box, centered: &Box, right: &Box) {
        let widget = Box::new(Orientation::Horizontal, 0);
        // 0.2.2: Allow for named spacings
        widget.set_widget_name(&self.name);
        widget.set_margin_start(self.spacing_start);
        widget.set_margin_end(self.spacing_end);

        // Align and add the widget
        match align {
            Align::LEFT => left.add(&widget),
            Align::CENTERED => centered.add(&widget),
            Align::RIGHT => right.add(&widget),
        }

        log("Added a new spacing widget");
    }
}
