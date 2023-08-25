use crate::ui::UI;
use gtk::Box;

/// Widget alignment.
#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Centered,
    Right,
}

/// Implements `from_str` for the Align structure.
impl Align {
    /// Tries to get the enum by its string-identifier.
    /// Note: Only lowercase letters will be detected.
    pub fn from_str(string: &str) -> Option<Align> {
        match string {
            "left" => Some(Align::Left),
            "centered" => Some(Align::Centered),
            "right" => Some(Align::Right),
            _ => None,
        }
    }
}

/// Implements basic traits for custom user-defined widgets.
pub trait HWidget {
    /// Invoked when the widget should be added.
    fn add(self, ui: &UI, name: &str, align: Align, box_holder: Option<&Box>);

    /// Label Widget: Tells the label to update content to the specified new content.
    #[allow(unused_variables)]
    fn update_label_direct(&self, new_content: &str) {}

    /// Label Widget: Tells the label to update with custom-defined behavior, for example from a local buffer.
    fn update_label_internal(&self) {}

    /// Function dedicated to starting optional loops.
    fn start_loop(&mut self) {}
}
