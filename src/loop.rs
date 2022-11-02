use crate::{
    cava::{get_current_bars, HAS_CAVA_CRASHED},
    config, ui,
    widget::HWidget,
};
use glib::Continue;
use std::time::Duration;
use tokio::task;

/// Updates dynamic bar content.
pub fn update() {
    update_labels();
    // Only start the tick-loop if there are actually Cava widgets available.
    if ui::CAVA_INSTANCES
        .lock()
        .expect("[ERROR] Cannot access ui::CAVA_INSTANCES!\n")
        .is_empty()
    {
        return;
    }

    // Run `update_cava` closure every 1ms.
    glib::timeout_add_local(Duration::from_millis(1), update_cava);
}

/// Updates all Cava widgets.
fn update_cava() -> Continue {
    let bars = &get_current_bars();
    // Loop through all Cava widget instances and sync the text.
    for widget in ui::CAVA_INSTANCES
        .lock()
        .expect("[ERROR] Cannot access ui::CAVA_INSTANCES!\n")
        .iter()
    {
        widget.update_label_reg(bars);
    }

    // If unwrap fails here, then I have lost all faith in computers.
    glib::Continue(!*HAS_CAVA_CRASHED.read().unwrap())
}

/// Updates all labels with a `command` set.
/// Only call this once as it's a loop.
fn update_labels() {
    task::spawn(async move {
        let update_rate = config::get_update_rate();
        loop {
            for widget in ui::VEC
                .lock()
                .expect("[ERROR] Cannot access ui::VEC!\n")
                .iter()
            {
                // If listen is set, don't execute a one-shot command.
                if !widget.listen {
                    let mut text = widget.text.clone();
                    text.push_str(&execute!(&widget.command));
                    widget.update_label_reg(&text);
                } else {
                    widget.update_label_internal()
                }
            }

            tokio::time::sleep(Duration::from_millis(update_rate)).await;
        }
    });
}
