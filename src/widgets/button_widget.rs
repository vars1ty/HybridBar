use crate::{aliases::use_aliases, config::with_variables, structures::Align, ui, widget::HWidget};
use gtk::{traits::*, *};
use std::{mem::take, time::Duration};

/// Creates a new button widget.
pub struct ButtonWidget {
    pub tooltip: String,
    pub tooltip_command: String,
    pub command: String,
    pub button: Button,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for ButtonWidget {
    fn add(
        mut self,
        name: &str,
        align: Align,
        left: &Box,
        centered: &Box,
        right: &Box,
        box_holder: Option<&Box>,
    ) {
        self.button.set_widget_name(name);
        // 0.2.8: Support tooltips for buttons
        self.button
            .set_tooltip_markup(Some(&with_variables(self.tooltip.to_owned())));

        // 0.3.6: Support for commands on tooltips.
        if !self.tooltip_command.is_empty() {
            self.start_loop();
        }

        // If the command isn't empty, subscribe to click events.
        if !self.command.is_empty() {
            self.button.connect_clicked(move |_| {
                execute!(&self.command);
            });
        }

        ui::add_and_align(&self.button, align, left, centered, right, box_holder);
        log!("Added a new button widget");
    }

    fn start_loop(&mut self) {
        let button = self.button.clone();
        let tooltip = take(&mut self.tooltip);
        let tooltip_command = take(&mut self.tooltip_command);
        let tick = move || {
            let mut new_tooltip = String::default();
            new_tooltip.push_str(&with_variables(tooltip.to_owned()));
            new_tooltip.push_str(&use_aliases(&tooltip_command));

            if let Some(tooltip_markup) = button.tooltip_markup() {
                if !tooltip_markup.eq(&new_tooltip) {
                    // Markup support here, the user therefore has to deal with any upcoming issues due to
                    // the command output, on their own.
                    button.set_tooltip_markup(Some(&new_tooltip));
                }
            }

            glib::Continue(true)
        };

        tick();
        // NOTE: This does NOT respect update_rate, since it's not meant to update super fast.
        glib::timeout_add_local(Duration::from_millis(1000), tick);
    }
}
