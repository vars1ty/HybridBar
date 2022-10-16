use crate::{debug::log, structures::Align, widget::HWidget};
use gtk::{
    traits::{ContainerExt, WidgetExt},
    Box, Orientation,
};

/// Creates a new basic spacing widget.
pub struct SpacingWidget {
    pub spacing_start: i32,
    pub spacing_end: i32,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for SpacingWidget {
    fn add(self, align: Align, left: &Box, centered: &Box, right: &Box) {
        let widget = Box::new(Orientation::Horizontal, 0);
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
