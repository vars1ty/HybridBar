use crate::{environment, proc};
use json::JsonValue;
use std::{any::TypeId, fmt::Display, fs, io::Error};

/// Gets the root home path to Hybrid.
pub fn get_path() -> String {
    format!(
        "/home/{}/.config/HybridBar/",
        proc::execute(String::from("whoami"))
    )
}

/// Parses the config and returns it.
pub fn read_config() -> JsonValue {
    let mut conf_path = get_path();
    conf_path.push_str(&environment::try_get_var("HYBRID_CONFIG"));
    json::parse(
        &fs::read_to_string(&conf_path)
            .expect(format!("[ERROR] Failed reading config file from '{conf_path}'!\n").as_str()),
    )
    .expect("[ERROR] Failed parsing config!\n")
}

/// If the `key` exists inside `root`, the value of it is returned.
/// If not, a default value is instead returned.
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
