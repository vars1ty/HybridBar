use crate::{structures::Align, ui, widget::HWidget};
use gtk::{traits::*, *};

/// Creates a new button widget.
pub struct ButtonWidget {
    pub tooltip: String,
    pub command: String,
    pub button: Button,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for ButtonWidget {
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box) {
        self.button.set_widget_name(&name);
        // 0.2.8: Support tooltips for buttons
        self.button.set_tooltip_markup(Some(&self.tooltip));

        // If the command isn't empty, subscribe to click events.
        if !self.command.is_empty() {
            self.button.connect_clicked(move |_| {
                log!(format!("Button '{}' -> Clicked", name));
                execute!(&self.command);
            });
        }

        ui::add_and_align(&self.button, align, left, centered, right);
        log!(format!("Added a new button widget"));
    }
}
