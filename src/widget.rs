use crate::structures::Align;
use gtk::Box;
use std::fmt::Display;

/// Implements basic traits for custom user-defined widgets.
pub trait HWidget {
    /// Invoked when the widget should be added.
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box);

    /// Label Widget: Tells the label to update content to the specified new content.
    fn update_label_direct(&self, _new_content: &(impl Display + Clone)) {}

    /// Label Widget: Tells the label to update with custom-defined behavior, for example from a local buffer.
    fn update_label_internal(&self) {}

    /// Function dedicated to starting optional loops.
    fn start_loop(&self) {}
}
