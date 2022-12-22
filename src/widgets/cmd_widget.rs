use crate::{structures::Align, ui, widget::HWidget};
use gtk::{traits::*, *};
use std::process::Command;

/// Creates a new cmd (`Entry`) widget.
pub struct CmdWidget {}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for CmdWidget {
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box) {
        let widget = Entry::new();
        widget.set_widget_name(&name);
        ui::add_and_align(&widget, align, left, centered, right);

        // .clone() because otherwise it starts crying about move.
        widget.clone().connect_key_press_event(move |_, key| {
            let real_key = key.keycode().expect("[ERROR] No keycode retrieved (???)");
            // 36 = Enter
            if real_key == 36 {
                // Could use execute!() but it waits for the process to finish, which we don't
                // want.
                let process = &widget.text();
                if !process.is_empty() && Command::new(process).spawn().is_err() {
                    log!(format!("[WARN] Failed spawning process '{process}'!"))
                }

                // Cleanup.
                widget.set_text("");
            }

            Inhibit(false)
        });

        log!("Added a new cmd widget");
    }
}
