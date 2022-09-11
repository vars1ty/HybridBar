use crate::{debug::debug_log, proc, ui};
use gtk::traits::*;
use std::time::Duration;

/// Updates dynamic bar content.
pub fn update() {
    let tick = move || {
        unsafe {
            update_labels();
        }

        // Indicates that we want to continue using our timer, false makes it stop.
        glib::Continue(true)
    };

    // Executes the "tick" closure every 100ms.
    glib::timeout_add_local(Duration::from_millis(100), tick);
}

/// Updates all of the labels.
unsafe fn update_labels() {
    for widget in ui::VEC.as_mut().unwrap() {
        if !widget.label.is_some() {
            // Not assigned, skip.
            continue;
        }

        let label = widget
            .label
            .as_ref()
            .expect("[ERROR] Failed retrieving Label!\n");
        let mut text = widget.properties.text.clone();
        // Append to the cloned text if the command isn't empty.
        if !widget.properties.command.is_empty() {
            // TODO: This is slow and causes frequent micro-blocking, so it should really be
            // reconsidered in the future.
            // The root cause seems to be the cloning of command.
            text += &proc::execute(widget.properties.command.clone());
        }

        // Check: never cause a redraw of the label by setting the text, if the new text is the
        // exact same as the current one.
        if text != label.text() {
            debug_log("Redrawing bar");
            label.set_text(&text);
        }
    }
}
