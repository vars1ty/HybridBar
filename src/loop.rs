use crate::{
    constants::{
        ERR_ACCESS_CAVA_INSTANCES, ERR_PARSE_CAVA_UPDATE_RATE, HYBRID_ROOT_JSON,
        WARN_CAVA_NO_BARS_INSTANCE, WARN_CAVA_NO_CRASHED_INSTANCE, WARN_NO_MAIN, WARN_NO_TICK,
    },
    utils::cava::{self, HAS_CAVA_CRASHED},
    widget::HWidget,
};
use glib::Continue;
use rune::FromValue;
use rune::Vm;
use std::time::Duration;

/// Updates dynamic bar content.
pub fn update(vm: Option<Vm>) {
    if let Some(vm) = vm {
        start_script_loop(vm);
    }

    start_cava_loop();
}

/// Attempts to start the script loop.
fn start_script_loop(mut vm: Vm) {
    if let Ok(func) = vm.lookup_function(["main"]) {
        func.call::<(), ()>(()).unwrap();
    } else {
        log!(WARN_NO_MAIN)
    }

    if vm.lookup_function(["tick"]).is_err() {
        log!(WARN_NO_TICK);
        return;
    }

    // If `get_update_rate()` is present, use the value from that function.
    // If not (or the parsing fails), use 250.
    let update_rate = if let Ok(call) = vm.call(["get_update_rate"], ()) {
        u64::from_value(call).unwrap_or(250)
    } else {
        250
    };

    glib::timeout_add_local(Duration::from_millis(update_rate), move || {
        let res = vm.call(["tick"], ());
        if let Err(err) = res {
            let (kind, _) = err.as_unwound();
            panic!("[ERROR] [RUNE]: Calling `tick` resulted in an error: {kind:?}");
        }

        glib::Continue(true)
    });
}

/// Attempts to start the Cava update loop.
fn start_cava_loop() {
    // Only start the cava loop if there are actually Cava widgets available.
    let widgets = cava::CAVA_INSTANCES
        .lock()
        .expect(ERR_ACCESS_CAVA_INSTANCES);
    if widgets.is_empty() {
        return;
    }

    // Run the `update_cava` closure every x ms.
    glib::timeout_add_local(
        Duration::from_millis(
            conf!(HYBRID_ROOT_JSON, "cava_update_rate", false, false)
                .number
                .unwrap_or_else(|| 1)
                .try_into()
                .expect(ERR_PARSE_CAVA_UPDATE_RATE),
        ),
        update_cava,
    );
}

/// Updates all Cava widgets.
fn update_cava() -> Continue {
    if let Ok(ref bars) = cava::BARS.lock() {
        // Loop through all Cava widget instances and sync the text.
        let widgets = cava::CAVA_INSTANCES
            .lock()
            .expect(ERR_ACCESS_CAVA_INSTANCES);
        let widgets = widgets.iter();
        for widget in widgets {
            widget.update_label_direct(bars);
        }

        if let Ok(has_cava_crashed) = HAS_CAVA_CRASHED.lock() {
            glib::Continue(!*has_cava_crashed)
        } else {
            log!(WARN_CAVA_NO_CRASHED_INSTANCE);
            glib::Continue(false)
        }
    } else {
        log!(WARN_CAVA_NO_BARS_INSTANCE);
        glib::Continue(false)
    }
}
