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
        let mut result = unsafe {
            String::from_utf8_unchecked(
                std::process::Command::new($crate::constants::PROC_TARGET)
                    .args(["-c", $cmd])
                    .output()
                    .unwrap()
                    .stdout,
            )
        };

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
        conf!($crate::constants::HYBRID_ROOT_JSON, "experimental", false)
    };
}
