use crate::structures::Align;
use gtk::Box;

/// Implements basic traits for custom user-defined widgets.
pub trait HWidget {
    /// Invoked when the widget should be added.
    fn add(
        self,
        name: &str,
        align: Align,
        left: &Box,
        centered: &Box,
        right: &Box,
        box_holder: Option<&Box>,
    );

    /// Label Widget: Tells the label to update content to the specified new content.
    #[allow(unused_variables)]
    fn update_label_direct(&self, new_content: &str) {}

    /// Label Widget: Tells the label to update with custom-defined behavior, for example from a local buffer.
    fn update_label_internal(&self) {}

    /// Function dedicated to starting optional loops.
    fn start_loop(&mut self) {}
}
