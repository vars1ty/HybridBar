use crate::{config, debug::log, load_css_from, proc, ui};
use gtk::traits::*;
use std::{path::Path, time::Duration};
use tokio::task;

/// Updates dynamic bar content.
pub fn update() {
    let mut css_path = config::get_path();
    css_path.push_str("style.css");
    update_labels(get_update_rate());
    let tick = move || {
        update_css(&css_path);
        // Indicates that we want to continue using our timer, false makes it stop.
        glib::Continue(true)
    };

    // Executes the "tick" closure for every millisecond specified in hybrid:update_rate.
    glib::timeout_add_local(Duration::from_millis(get_update_rate()), tick);
}

/// Updates the CSS.
fn update_css(css_path: &String) {
    // Only watch the file if it actually exists.
    if !Path::new(&css_path).is_file() {
        return;
    }

    let path = Path::new(&css_path);
    load_css_from(path)
}

/// Returns the set update-rate.
fn get_update_rate() -> u64 {
    let mut update_rate = config::try_get("hybrid", "update_rate", false).1;
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
fn update_labels(update_rate: u64) {
    // Async looping task in order not to interrupt the UI and cause lag to widgets like button
    // animations.
    task::spawn(async move {
        log("created update_labels task");
        loop {
            for widget in ui::VEC
                .lock()
                .expect("[ERROR] Cannot access ui::VEC!\n")
                .iter()
            {
                let mut text = widget.text.clone();
                // Append to the cloned text if the command isn't empty.
                if !widget.command.is_empty() {
                    text.push_str(&proc::execute(&widget.command))
                }

                // Check: never cause a redraw of the label by setting the text, if the new text is the
                // exact same as the current one.
                if text != widget.label.text() {
                    log(format!(
                        "Label update received (from => \"{}\", to => \"{text}\")",
                        widget.label.text()
                    ));

                    log("redrawing");
                    widget.label.set_text(&text)
                }
            }

            // We could even remove this line, but it's staying because there's no need to update
            // it 24/7.
            tokio::time::sleep(Duration::from_millis(update_rate)).await;
        }
    });
}
