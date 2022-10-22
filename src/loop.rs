use crate::{cava, config, debug::log, load_css_from, proc, ui, widget::HWidget};
use gtk::traits::*;
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

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

    unsafe {
        draw_cava();
    }
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
                    widget.update_label(&text);
                }
            }

            // We could even remove this line, but it's staying because there's no need to update
            // it 24/7.
            tokio::time::sleep(Duration::from_millis(update_rate)).await;
        }
    });
}

/// Draws Cava for all widgets that implement it.
unsafe fn draw_cava() {
    task::spawn(async move {
        log("created draw_cava task");
        let mut bars;
        let sed = cava::get_sed();
        let path = cava::get_temp_config();
        // Start a process which reads cava's output, then sync the labels content with it.
        // This **has** to stay inside this specific scope, because calling it from other functions
        // for w/e reason makes it break.
        let mut child = Command::new("bash")
            .args(["-c", format!("cava -p {path} | sed -u '{sed}'").as_str()])
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .expect("[ERROR] Cannot start cava script!\n");

        let out = child
            .stdout
            .take()
            .expect("[ERROR] Cannot take stdout from child!\n");
        let mut reader = BufReader::new(out).lines();

        loop {
            // Get the next line from stdout.
            bars = reader
                .next_line()
                .await
                .expect("[ERROR] There are no more lines available!\n")
                .expect("[ERROR] The string value is None!\n");

            // Loop through all Cava widget instances and sync the text.
            for widget in ui::CAVA_INSTANCES
                .lock()
                .expect("[ERROR] Cannot access ui::CAVA_INSTANCES!\n")
                .iter()
            {
                widget.update_label(&bars);
            }

            // Hack: Because this function is unsafe due to the nature of GTK and
            // threads/async, we have to slow down by a small margin.
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });
}
