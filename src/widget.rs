use crate::structures::Align;
use gtk::Box;
use std::fmt::Display;

/// Implements basic traits for custom user-defined widgets.
pub trait HWidget {
    /// Invoked when the widget should be added.
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box);

    /// If the widget is a label of some sort, tell it to update its content.
    fn update_label(&self, _new_content: &(impl Display + Clone)) {}
}
