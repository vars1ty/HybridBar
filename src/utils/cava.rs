use crate::{cava_widget::CavaWidget, constants::*, math};
use std::{fs::File, io::Write, process::Stdio, sync::Mutex};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

lazy_static! {
    /// Current Cava bars.
    static ref BARS: Mutex<String> = Mutex::new(String::default());
    /// Has Cava crashed? If true, don't keep `update_cava` running.
    pub static ref HAS_CAVA_CRASHED: Mutex<bool> = Mutex::new(false);
    /// All active Cava widget instances.
    pub static ref CAVA_INSTANCES: Mutex<heapless::Vec<CavaWidget, 8>> = Mutex::new(heapless::Vec::new());
}

/// Gets the sed to use for Cava.
pub fn get_sed() -> String {
    conf!(HYBRID_ROOT_JSON, "cava_sed", true, false)
        .string
        .unwrap_or(
            "s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;".to_owned(),
        )
}

/// Returns the amount of bars that should be present.
fn get_bars() -> i32 {
    let bars = conf!(HYBRID_ROOT_JSON, "cava_bars", false, false)
        .number
        .unwrap_or(5);
    math::clamp_i32(bars, 2, 16)
}

/// Returns the current Cava bars.
pub fn get_current_bars() -> String {
    BARS.lock().unwrap().to_string()
}

/// Returns the desired framerate to use for Cava updates.
fn get_framerate() -> i32 {
    let framerate = conf!(HYBRID_ROOT_JSON, "cava_framerate", false, false)
        .number
        .unwrap_or(60);
    math::clamp_i32(framerate, 60, 360)
}

/// Builds the temporary Cava configuration and then returns the path to it,
pub fn get_temp_config() -> String {
    let path = String::from(CAVA_TMP_CONFIG);
    let mut file = File::create(&path).expect("[ERROR] Couldn't create the temporary Cava config!");
    // 0.2.7: Support for dynamically configuring the temporary config to an extent.
    let bars = get_bars();
    let framerate = get_framerate();
    file.write_all(
        format!(
            r#"
# Cava Configuration for Hybrid
[general]
framerate = {framerate}
bars = {bars}
[output]
method = raw
raw_target = /dev/stdout
data_format = ascii
ascii_max_range = 7
                   "#,
        )
        .as_bytes(),
    )
    .expect("[ERROR] Failed writing to the temporary Cava config!");
    path
}

/// Updates the `BARS` value with Cava.
/// Only call this once as it's a loop.
pub fn update_bars() {
    task::spawn(async move {
        let mut bars;
        let sed = get_sed();
        let path = get_temp_config();
        let mut child = Command::new(PROC_TARGET)
            .args(["-c", &format!("cava -p {path} | sed -u '{sed}'")])
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .expect("[ERROR] Cannot start Cava script!");

        let out = child
            .stdout
            .take()
            .expect("[ERROR] Cannot take stdout from child process!");

        // Drop to free the resources as we don't need to access them anymore.
        drop(sed);
        drop(path);
        let mut reader = BufReader::new(out).lines();
        loop {
            bars = {
                let this = {
                    let next_line = reader.next_line().await;
                    match next_line {
                        Ok(t) => t,
                        Err(_) => {
                            *HAS_CAVA_CRASHED.lock().unwrap() = true;
                            BARS.lock().unwrap().clear();
                            panic!("[WARN] Cava: There are no more lines available. Hybrid will keep on running but Cava will be stopped!")
                        }
                    }
                };

                match this {
                    Some(val) => val,
                    None => {
                        *HAS_CAVA_CRASHED.lock().unwrap() = true;
                        BARS.lock().unwrap().clear();
                        panic!("[WARN] Cava: The string value is None, Hybrid will keep on running but Cava will be stopped!")
                    }
                }
            };

            *BARS.lock().unwrap() = bars;
        }
    });
}
