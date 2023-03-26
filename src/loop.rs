use crate::{
    constants::{
        ERR_ACCESS_CAVA_INSTANCES, ERR_PARSE_CAVA_UPDATE_RATE, ERR_UPDATE_RATE_TYPE,
        HYBRID_ROOT_JSON, UPDATE_RATE_HASH, WARN_CAVA_NO_BARS_INSTANCE,
        WARN_CAVA_NO_CRASHED_INSTANCE, WARN_NO_MAIN, WARN_NO_TICK,
    },
    utils::cava::{self, HAS_CAVA_CRASHED},
    widget::HWidget,
};
use glib::Continue;
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
fn start_script_loop(vm: Vm) {
    if let Ok(func) = vm.lookup_function(["main"]) {
        let res = func.call::<(), ()>(());
        if let Err(err) = res {
            let (kind, _) = err.as_unwound();
            panic!("[ERROR] [RUNE]: Calling `main` resulted in an error: {kind:?}");
        }
    } else {
        log!(WARN_NO_MAIN)
    }

    let tick_func = vm.lookup_function(["tick"]);
    if tick_func.is_err() {
        log!(WARN_NO_TICK);
        return;
    }

    let tick_func = tick_func.unwrap();

    // If `UPDATE_RATE` is present, use the value from that constant.
    // If not (or the parsing fails), use 250.
    let mut update_rate: u64 = 250;
    for (hash, value) in vm.unit().iter_constants() {
        if hash.to_string() != UPDATE_RATE_HASH {
            continue;
        }

        update_rate = value
            .clone()
            .into_value()
            .into_integer()
            .expect(ERR_UPDATE_RATE_TYPE)
            .try_into()
            .unwrap_or(250);
        break;
    }

    glib::timeout_add_local(Duration::from_millis(update_rate), move || {
        let res = tick_func.call::<(), ()>(());
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
        .read()
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
            .read()
            .expect(ERR_ACCESS_CAVA_INSTANCES);
        let widgets = widgets.iter();
        for widget in widgets {
            widget.update_label_direct(bars);
        }

        if let Ok(has_cava_crashed) = HAS_CAVA_CRASHED.read() {
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
