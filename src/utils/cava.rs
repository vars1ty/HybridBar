use crate::{config, math};
use std::{fs::File, io::Write, process::Stdio, sync::RwLock};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

lazy_static! {
    /// Current Cava bars.
    static ref BARS: RwLock<String> = RwLock::new(String::default());

    /// Has Cava crashed? If true, don't keep `tick` running.
    pub static ref HAS_CAVA_CRASHED: RwLock<bool> = RwLock::new(false);
}

/// Gets the sed to use for Cava.
pub fn get_sed() -> String {
    let mut sed =
        String::from("s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;");

    if let Some(c_sed) = config::try_get("hybrid", "cava_sed", true, false) {
        sed = c_sed.0
    }

    sed
}

/// Returns the amount of bars that should be present.
fn get_bars() -> i32 {
    let mut bars = 5;

    if let Some(c_bars) = config::try_get("hybrid", "cava_bars", false, false) {
        bars = c_bars.1
    }

    math::clamp_i32(bars, 2, 16)
}

/// Returns the current Cava bars.
pub fn get_current_bars() -> String {
    BARS.read().unwrap().to_string()
}

/// Returns the desired framerate to use for Cava updates.
fn get_framerate() -> i32 {
    let mut framerate = 60;

    if let Some(c_framerate) = config::try_get("hybrid", "cava_framerate", false, false) {
        framerate = c_framerate.1
    }

    math::clamp_i32(framerate, 60, 360)
}

/// Builds the temporary Cava configuration and then returns the path to it,
pub fn get_temp_config() -> String {
    let path = String::from("/tmp/cava_tmp_hybrid.conf");
    let mut file =
        File::create(&path).expect("[ERROR] Couldn't create the temporary Cava config!\n");
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
    .expect("[ERROR] Failed writing to the temporary Cava config!\n");
    path
}

/// Updates the `BARS` value with Cava.
/// Only call this once as it's a loop.
pub fn update_bars() {
    task::spawn(async move {
        let mut bars;
        let sed = get_sed();
        let path = get_temp_config();
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
                            *HAS_CAVA_CRASHED.write().unwrap() = true;
                            BARS.write().unwrap().clear();
                            panic!("[WARN] Cava: There are no more lines available. Hybrid will keep on running but Cava will be stopped!\n")
                        }
                    }
                };

                match this {
                    Some(val) => val,
                    None => {
                        *HAS_CAVA_CRASHED.write().unwrap() = true;
                        BARS.write().unwrap().clear();
                        panic!("[WARN] Cava: The string value is None, Hybrid will keep on running but Cava will be stopped!\n")
                    }
                }
            };

            *BARS.write().unwrap() = bars;
        }
    });
}
