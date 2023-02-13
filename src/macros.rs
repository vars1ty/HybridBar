#[macro_export]
/// Logs a [HYBRID] [DEBUG] formatted message to stdout.
macro_rules! log {
    ($msg:expr) => {
        if $crate::environment::try_get_var("HYBRID_LOG", "0") == "1" {
            println!("[LOG]: {}", $msg)
        }
    };
}

#[macro_export]
/// Executes a bash command and outputs it to `result`.
macro_rules! execute {
    ($cmd:expr) => {{
        let mut result = String::from_utf8_lossy(
            &std::process::Command::new($crate::constants::PROC_TARGET)
                .args(["-c", $cmd])
                .output()
                .unwrap()
                .stdout,
        )
        .to_string();

        // Remove the last character as its a new line.
        result.pop();

        result
    }};
}

#[macro_export]
/// Gets a value from the config.
macro_rules! conf {
    ($root:expr, $key:expr, $is_string:expr, $with_custom_variables:expr) => {
        $crate::config::try_get($root, $key, $is_string, $with_custom_variables)
    };
}

#[macro_export]
/// Gets a `bool` value from the config.
/// If there is no value assigned, the value from `default` is returned.
macro_rules! conf_bool {
    ($root:expr, $key:expr, $default:expr) => {
        if let Some(ref res) = conf!($root, $key, true, false).string {
            res == "true"
        } else {
            $default
        }
    };
}

#[macro_export]
/// Are experimental features enabled?
macro_rules! experimental {
    () => {
        conf_bool!($crate::constants::HYBRID_ROOT_JSON, "experimental", false)
    };
}

#[macro_export]
macro_rules! define {
    ($type:ty, $name:ident, $val:expr) => {
        pub const $name: $type = $val;
    };
}
