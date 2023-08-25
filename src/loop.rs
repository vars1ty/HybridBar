use crate::{
    config::Config,
    constants::{
        ERR_UPDATE_RATE_TYPE, HYBRID_ROOT_JSON, UPDATE_RATE_HASH, WARN_CAVA_NO_BARS_INSTANCE,
        WARN_CAVA_NO_CRASHED_INSTANCE, WARN_NO_MAIN, WARN_NO_TICK,
    },
    utils::cava::{self, HAS_CAVA_CRASHED},
    widget::HWidget,
};
use glib::ControlFlow::{self, *};
use rune::Vm;
use std::time::Duration;

/// Updates dynamic bar content.
pub fn update(vm: Option<Vm>, config: &'static Config) {
    if let Some(vm) = vm {
        start_script_loop(vm);
    }

    start_cava_loop(config);
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
    let mut update_rate = 250u64;
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

        Continue
    });
}

/// Attempts to start the Cava update loop.
fn start_cava_loop(config: &'static Config) {
    // Only start the cava loop if there are actually Cava widgets available.
    let widgets = cava::CAVA_INSTANCES.read();
    if widgets.is_empty() {
        return;
    }

    // Run the `update_cava` closure every x ms.
    glib::timeout_add_local(
        Duration::from_millis(
            config.read_config_raw()[HYBRID_ROOT_JSON]["cava_update_rate"]
                .as_u64()
                .unwrap_or(1),
        ),
        update_cava,
    );
}

/// Updates all Cava widgets.
fn update_cava() -> ControlFlow {
    if let Some(bars) = cava::BARS.try_read() {
        // Loop through all Cava widget instances and sync the text.
        let widgets = cava::CAVA_INSTANCES.read();
        let widgets = widgets.iter();
        for widget in widgets {
            widget.update_label_direct(&bars);
        }

        if let Some(has_cava_crashed) = HAS_CAVA_CRASHED.try_read() {
            if !*has_cava_crashed {
                Continue
            } else {
                Break
            }
        } else {
            log!(WARN_CAVA_NO_CRASHED_INSTANCE);
            Break
        }
    } else {
        log!(WARN_CAVA_NO_BARS_INSTANCE);
        Break
    }
}
