use crate::{
    cava::{get_current_bars, HAS_CAVA_CRASHED},
    config, math, ui,
    widget::HWidget,
};
use gtk::traits::*;
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

    let tick = move || {
        let bars = &get_current_bars();
        // Loop through all Cava widget instances and sync the text.
        for widget in ui::CAVA_INSTANCES
            .lock()
            .expect("[ERROR] Cannot access ui::CAVA_INSTANCES!\n")
            .iter()
        {
            widget.update_label(bars);
        }

        glib::Continue(!*HAS_CAVA_CRASHED.read().unwrap())
    };

    // Run the tick closure every 1ms.
    log!("CAVA WIDGET/WIDGETS ACTIVE, RUN LOOP");
    glib::timeout_add_local(Duration::from_millis(1), tick);
}

/// Returns the set update-rate.
fn get_update_rate() -> u64 {
    let update_rate = math::clamp_i32(config::try_get("hybrid", "update_rate", false).1, 5, 10_000);

    if update_rate < 100 {
        println!(
            "[CRITICAL WARN] Your update-rate is {update_rate}ms! Expect performance drawbacks"
        )
    }

    update_rate
        .try_into()
        .expect("[ERROR] Cannot convert update_rate into u64!\n")
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
                    text.push_str(&execute!(&widget.command))
                }

                // Check: never cause a redraw of the label by setting the text, if the new text is the
                // exact same as the current one.
                if !text.eq(&widget.label.text()) {
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
