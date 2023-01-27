use crate::{
    cava::{self, BARS, HAS_CAVA_CRASHED},
    constants::HYBRID_ROOT_JSON,
    widget::HWidget,
};
use glib::Continue;
use std::time::Duration;

/// Updates dynamic bar content.
pub fn update() {
    // Only start the tick-loop if there are actually Cava widgets available.
    let widgets = cava::CAVA_INSTANCES
        .lock()
        .expect("[ERROR] Cannot access ui::CAVA_INSTANCES!");
    if widgets.is_empty() {
        return;
    }

    // Run the `update_cava` closure every x ms.
    glib::timeout_add_local(
        Duration::from_millis(
            conf!(HYBRID_ROOT_JSON, "cava_update_rate", false, false)
                .number
                .unwrap_or(1)
                .try_into()
                .unwrap(),
        ),
        update_cava,
    );
}

/// Updates all Cava widgets.
fn update_cava() -> Continue {
    let bars = &*BARS.lock().unwrap();
    // Loop through all Cava widget instances and sync the text.
    let widgets = cava::CAVA_INSTANCES
        .lock()
        .expect("[ERROR] Cannot access ui::CAVA_INSTANCES!");
    let widgets = widgets.iter();
    for widget in widgets {
        widget.update_label_direct(bars);
    }

    // If unwrap fails here, then I have lost all faith in computers.
    glib::Continue(!*HAS_CAVA_CRASHED.lock().unwrap())
}
