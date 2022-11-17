use crate::{
    cava::{get_current_bars, HAS_CAVA_CRASHED},
    ui,
    widget::HWidget,
};
use glib::Continue;
use std::time::Duration;

/// Updates dynamic bar content.
pub fn update() {
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
