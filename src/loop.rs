use crate::{cava, config, proc, ui, widget::HWidget};
use gtk::traits::*;
use std::{process::Stdio, sync::RwLock, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

lazy_static! {
    /// Current Cava bars.
    static ref BARS: RwLock<String> = RwLock::new(String::default());
}

/// Returns the current Cava bars.
fn get_bars() -> String {
    BARS.read().unwrap().to_string()
}

/// Updates dynamic bar content.
pub fn update() {
    update_labels();
    let tick = move || {
        // Loop through all Cava widget instances and sync the text.
        for widget in ui::CAVA_INSTANCES
            .lock()
            .expect("[ERROR] Cannot access ui::CAVA_INSTANCES!\n")
            .iter()
        {
            widget.update_label(&get_bars());
        }
        // Indicates that we want to continue using our timer, false makes it stop.
        glib::Continue(true)
    };

    glib::timeout_add_local(Duration::from_millis(1), tick);
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

/// Updates the `BARS` value with Cava.
/// Only call this once as it's a loop.
pub fn update_bars() {
    task::spawn(async move {
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

        // Drop to free the resources in case something unexpected happens.
        drop(sed);
        drop(path);
        let mut reader = BufReader::new(out).lines();
        loop {
            bars = reader
                .next_line()
                .await
                .expect("[ERROR] There are no more lines available!\n")
                .expect("[ERROR] The string value is None!\n");

            *BARS.write().unwrap() = bars;
        }
    });
}

/// Updates all labels with a `command` set.
/// Only call this once as it's a loop.
fn update_labels() {
    task::spawn(async move {
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
                    log!(format!(
                        "Label update received (from => \"{}\", to => \"{text}\")",
                        widget.label.text()
                    ));

                    widget.update_label(&text);
                }
            }

            tokio::time::sleep(Duration::from_millis(get_update_rate())).await;
        }
    });
}
