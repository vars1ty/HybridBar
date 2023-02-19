use crate::{
    ui,
    widget::{Align, HWidget},
};
use gtk::{traits::*, *};

/// Creates a new basic spacing widget.
pub struct SpacingWidget {
    pub spacing_start: i32,
    pub spacing_end: i32,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for SpacingWidget {
    fn add(
        self,
        name: &str,
        align: Align,
        left: &Box,
        centered: &Box,
        right: &Box,
        box_holder: Option<&Box>,
    ) {
        let widget = Box::new(Orientation::Horizontal, 0);
        // 0.2.2: Allow for named spacings
        widget.set_widget_name(name);
        widget.set_margin_start(self.spacing_start);
        widget.set_margin_end(self.spacing_end);

        ui::add_and_align(&widget, align, left, centered, right, box_holder);
        log!("Added a new spacing widget");
    }
}
