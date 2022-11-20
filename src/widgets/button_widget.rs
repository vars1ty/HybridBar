use crate::{structures::Align, ui, widget::HWidget};
use glib::GString;
use gtk::{traits::*, *};
use std::time::Duration;

/// Creates a new button widget.
pub struct ButtonWidget {
    pub tooltip: String,
    pub tooltip_command: String,
    pub command: String,
    pub button: Button,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for ButtonWidget {
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box) {
        self.button.set_widget_name(&name);
        // 0.2.8: Support tooltips for buttons
        self.button.set_tooltip_markup(Some(&self.tooltip));

        // 0.3.6: Support for commands on tooltips.
        if !self.tooltip_command.is_empty() {
            self.start_loop();
        }

        // If the command isn't empty, subscribe to click events.
        if !self.command.is_empty() {
            self.button.connect_clicked(move |_| {
                log!(format!("Button '{}' -> Clicked", name));
                execute!(&self.command);
            });
        }

        ui::add_and_align(&self.button, align, left, centered, right);
        log!("Added a new button widget");
    }

    fn start_loop(&self) {
        let button_clone = self.button.clone();
        let tooltip_clone = self.tooltip.clone();
        let tooltip_command_clone = self.tooltip_command.clone();
        const EMPTY: &str = "";
        let tick = move || {
            let mut new_tooltip = String::default();
            new_tooltip.push_str(&tooltip_clone);
            new_tooltip.push_str(execute!(&tooltip_command_clone).as_str());

            let tooltip_markup = button_clone
                .tooltip_markup()
                .unwrap_or_else(|| GString::from(EMPTY));

            if !tooltip_markup.eq(&new_tooltip) {
                // Markup support here, the user therefore has to deal with any upcoming issues due to
                // the command output, on their own.
                button_clone.set_tooltip_markup(Some(&new_tooltip));
            }

            glib::Continue(true)
        };

        tick();
        // NOTE: This does NOT respect update_rate, since it's not meant to update super fast.
        glib::timeout_add_local(Duration::from_millis(1000), tick);
    }
}
