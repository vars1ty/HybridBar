use crate::proc;
use json::JsonValue;
use std::{any::TypeId, fmt::Display, fs, io::Error};

/// Parses the config and returns it.
pub fn read_config() -> JsonValue {
    let path = format!(
        "/home/{}/.config/HybridBar/config.json",
        proc::execute(String::from("whoami"))
    );
    let b_path = &path;
    json::parse(
        &fs::read_to_string(b_path)
            .expect(format!("[ERROR] Failed reading config file from '{b_path}'!\n").as_str()),
    )
    .expect("[ERROR] Failed parsing config!\n")
}

/// If the `key` exists inside `root`, the value of it is returned.
/// If not, an default value is instead returned.
pub fn try_get<T>(root: &str, key: &str) -> Result<(String, i32), Error>
where
    T: Display + 'static,
{
    let cfg = &read_config()[root];
    // This is probably the wrong way of doing it, but I can't think of a better way rn.
    let is_string = TypeId::of::<T>() == TypeId::of::<String>();
    const ERROR_I32: &str = "[ERROR] Failed parsing content as i32!";
    if cfg.has_key(key) {
        if !is_string {
            return Ok((String::default(), cfg[key].as_i32().expect(ERROR_I32)));
        }

        Ok((cfg[key].to_string(), 0))
    } else {
        Ok((String::default(), 0))
    }
}
