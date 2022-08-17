use crate::{proc, ui};
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
    for (label, dynamic) in ui::MAP.as_mut().unwrap().iter() {
        let mut text = dynamic.text.clone();
        if !dynamic.command.is_empty() {
            text += &proc::execute(dynamic.command.clone());
        }

        // Check: never cause a redraw of the label by setting the text, if the new text is the
        // exact same as the current one.
        if text != label.text() {
            label.set_text(&text);
        }
    }
}
