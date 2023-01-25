use crate::{structures::Align, ui, widget::HWidget};
use gtk::{traits::*, *};

/// Creates a new basic box widget.
pub struct BoxWidget {
    pub width: i32,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for BoxWidget {
    fn add<'a>(self, name: &'a str, align: Align, left: &Box, centered: &Box, right: &Box) {
        let widget = Box::new(Orientation::Horizontal, 0);
        widget.set_widget_name(name);
        widget.set_width_request(self.width);

        ui::add_and_align(&widget, align, left, centered, right);
        log!("Added a new box widget");
    }
}
