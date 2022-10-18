use crate::{debug::log, structures::Align, widget::HWidget};
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
                log(format!("Button '{name_clone}' -> Clicked"));
            });
        }

        // Align and add the widget
        match align {
            Align::LEFT => left.add(&self.button),
            Align::CENTERED => centered.add(&self.button),
            Align::RIGHT => right.add(&self.button),
        }

        log(format!("Added a new button widget named '{}'", self.name));
    }
}
