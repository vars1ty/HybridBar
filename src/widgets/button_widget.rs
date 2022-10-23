use crate::{structures::Align, ui, widget::HWidget};
use gtk::{traits::*, *};

/// Creates a new button widget.
pub struct ButtonWidget {
    pub name: String,
    pub command: String,
    pub button: Button,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for ButtonWidget {
    fn add(self, align: Align, left: &Box, centered: &Box, right: &Box) {
        self.button.set_widget_name(&self.name);

        // If the command isn't empty, subscribe to click events.
        if !self.command.is_empty() {
            let name_clone = self.name.clone();
            self.button.connect_clicked(move |_| {
                log!(format!("Button '{name_clone}' -> Clicked"));
            });
        }

        ui::add_and_align(&self.button, align, left, centered, right);
        log!(format!("Added a new button widget named '{}'", self.name));
    }
}
