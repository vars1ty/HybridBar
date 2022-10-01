use crate::structures::Align;
use gtk::Box;

/// Implements basic traits for custom user-defined widgets.
pub trait HWidget {
    /// Invoked when the widget should be added.
    fn add(self, align: Align, left: &Box, centered: &Box, right: &Box);
}
