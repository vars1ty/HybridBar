use crate::{config, debug::log, proc, ui};
use gtk::traits::*;
use std::time::Duration;

/// Updates dynamic bar content.
pub fn update() {
    let tick = move || {
        update_labels();
        // Indicates that we want to continue using our timer, false makes it stop.
        glib::Continue(true)
    };

    // Executes the "tick" closure for every millisecond specified in hybrid:update_rate.

    glib::timeout_add_local(Duration::from_millis(get_update_rate()), tick);
}

/// Returns the set update-rate.
fn get_update_rate() -> u64 {
    let mut update_rate = config::try_get::<i32>("hybrid", "update_rate").1;
    // Clamp the value to a minimum of 5.
    if update_rate < 5 {
        update_rate = 5;
    }

    if update_rate < 100 {
        println!("[HYBRID] [CRITICAL WARNING] Your update-rate is {update_rate}ms! Expect performance drawbacks")
    }

    update_rate
        .try_into()
        .expect("[ERROR] Cannot convert update_rate into u64!\n")
}

/// Updates all of the labels.
fn update_labels() {
    for widget in ui::VEC
        .lock()
        .expect("[ERROR] Cannot access ui::VEC!\n")
        .iter()
    {
        let mut text = widget.text.clone();
        // Append to the cloned text if the command isn't empty.
        if !widget.command.is_empty() {
            // TODO: This is slow and causes frequent micro-blocking, so it should really be
            // reconsidered in the future.
            // The root cause is because execute() is interrupting the UI Thread.
            text.push_str(&proc::execute(widget.command.clone()))
        }

        // Check: never cause a redraw of the label by setting the text, if the new text is the
        // exact same as the current one.
        if text != widget.label.text() {
            log(format!(
                "Label update received (from => \"{}\", to => \"{text}\") -- redrawing",
                widget.label.text()
            ));
            widget.label.set_text(&text)
        }
    }
}
