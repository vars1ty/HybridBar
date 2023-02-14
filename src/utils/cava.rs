use crate::{constants::*, math, types::MediumString, widgets::cava_widget::CavaWidget};
use arraystring::{typenum::U25, ArrayString};
use std::{fs::File, io::Write, process::Stdio, sync::Mutex};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

type SmallString = ArrayString<U25>;

lazy_static! {
    /// Has Cava been started yet?
    pub static ref HAS_CAVA_STARTED: Mutex<bool> = Mutex::new(false);
    /// Current Cava bars.
    pub static ref BARS: Mutex<SmallString> = Mutex::new(SmallString::default());
    /// Has Cava crashed? If true, don't keep `update_cava` running.
    pub static ref HAS_CAVA_CRASHED: Mutex<bool> = Mutex::new(false);
    /// All active Cava widget instances.
    pub static ref CAVA_INSTANCES: Mutex<Vec<CavaWidget>> = Mutex::new(Vec::new());
}

/// Gets the sed to use for Cava.
pub fn get_sed() -> MediumString {
    conf!(HYBRID_ROOT_JSON, "cava_sed", true, false)
        .string
        .unwrap_or(str!(
            MediumString,
            "s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;",
            false
        ))
}

/// Returns the amount of bars that should be present.
fn get_bars() -> i32 {
    let bars = conf!(HYBRID_ROOT_JSON, "cava_bars", false, false)
        .number
        .unwrap_or(5);
    math::clamp_i32(bars, 2, 16)
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
    let mut file = File::create(&path).expect(ERR_CREATE_TMP_CONF);
    // 0.2.7: Support for dynamically configuring the temporary config to an extent.
    let bars = get_bars();
    let framerate = get_framerate();
    let mut conf = include_str!("../../resources/cava_tmp.conf");
    let formatted = conf
        .replace("[framerate]", &framerate.to_string())
        .replace("[bars]", &bars.to_string());

    conf = &formatted;
    file.write_all(conf.as_bytes()).expect(ERR_WRITE_TMP_CONF);
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
            .expect(ERR_START_CAVA);

        let out = child.stdout.take().expect(ERR_TAKE_STDOUT);

        // Drop to free the resources as we don't need to access them anymore.
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
                            panic!("{}", WARN_CAVA_NO_LINES)
                        }
                    }
                };

                match this {
                    Some(val) => val,
                    None => {
                        *HAS_CAVA_CRASHED.lock().unwrap() = true;
                        BARS.lock().unwrap().clear();
                        panic!("{}", WARN_CAVA_NO_LINES)
                    }
                }
            };

            if let Ok(mut r_bars) = BARS.lock() {
                *r_bars = SmallString::try_from_str(bars).unwrap();
            }
        }
    });
}
