use std::{fs::File, io::Write};

use crate::{config, math};

/// Gets the sed to use for Cava.
pub fn get_sed() -> String {
    let mut sed = config::try_get("hybrid", "cava_sed", true).0;
    if sed.is_empty() {
        sed =
            String::from("s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;")
    }

    sed
}

/// Returns the amount of bars that should be present.
fn get_bars() -> i32 {
    math::clamp_i32(config::try_get("hybrid", "cava_bars", false).1, 2, 16)
}

/// Returns the desired framerate to use for Cava updates.
fn get_framerate() -> i32 {
    math::clamp_i32(config::try_get("hybrid", "cava_framerate", false).1, 60, 200)
}

/// Builds the temporary Cava configuration and then returns the path to it,
pub fn get_temp_config() -> String {
    let path = String::from("/tmp/cava_tmp_hybrid.conf");
    let mut file =
        File::create(&path).expect("[ERROR] Couldn't create the temporary Cava config!\n");
    // 0.2.7: Support for dynamically configuring the temporary config to an extent.
    let bars = get_bars();
    let framerate = get_framerate();
    // TODO: Make bars configurable from hybrid:cava_bars?
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
