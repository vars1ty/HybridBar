// This file is meant for holding constant strings / repeatedly-used numerical values.

/// Master Hybrid JSON Key.
pub const HYBRID_ROOT_JSON: &str = "hybrid";
/// Master Variables JSON Key.
pub const HYBRID_V_ROOT_JSON: &str = "variables";
/// Master Features JSON Key.
pub const HYBRID_F_ROOT_JSON: &str = "features";
/// Process to be used for calling external commands.
pub const PROC_TARGET: &str = "sh";
/// Default stylesheet file.
pub const DEFAULT_CSS: &str = "style.css";
/// Default config file.
pub const DEFAULT_CONFIG: &str = "config.json";
/// Cava temporary config path.
pub const CAVA_TMP_CONFIG: &str = "/tmp/cava_tmp_hybrid.conf";

// Constant errors.

pub const ERR_PARSE_CAVA_UPDATE_RATE: &str =
    "[ERROR] hybrid:cava_update_rate couldn't be parsed into a 64-bit unsigned integer (u64)!";
pub const ERR_PARSE_UPDATE_RATE: &str = "[ERROR] Cannot convert update_rate into u64!";
pub const ERR_ACCESS_CAVA_INSTANCES: &str = "[ERROR] Couldn't access ui::CAVA_INSTANCES!";
pub const ERR_INVALID_POS: &str =
    "[ERROR] Invalid position! Values: [ TOP, BOTTOM ] - casing doesn't matter.";
pub const ERR_GET_DISPLAY: &str =
    "[ERROR] Couldn't find a valid display, is your compositor doing alright?";
pub const ERR_GET_MONITOR: &str = "[ERROR] Couldn't find a valid monitor.";
pub const ERR_SCREEN_DEFAULT: &str = "[ERROR] Couldn't find a valid screen!";
pub const ERR_LOAD_SAMPLE_CSS: &str = "[ERROR] Failed loading the example stylesheet!";
pub const ERR_CUSTOM_DRAW: &str =
    "[ERROR] Failed drawing Hybrid using custom color sources, which is needed for transparency!";
pub const ERR_INVALID_WIDGET_FORMAT: &str =
    "[ERROR] Widgets should be named as [alignment]-[widget_type]_[name]";
pub const ERR_EMPTY_NAME: &str =
    "[ERROR] Found an empty widget name, this is not currently supported!";
pub const ERR_INVALID_ALIGNMENT: &str =
    "[ERROR] Invalid widget alignment! Valid values are: [ left, centered, right ]";
pub const ERR_TAKE_STDOUT: &str = "[ERROR] Cannot take stdout from child process!";
pub const ERR_NO_LINES: &str = "[ERROR] There are no more lines available!";
pub const ERR_STRING_NONE: &str = "[ERROR] The string value is None!";
pub const ERR_NO_LXINFO: &str =
    "System Info isn't available for this system, therefore aliases have been disabled.";
pub const ERR_WRITE_TMP_CONF: &str = "[ERROR] Failed writing to the temporary Cava config!";
pub const ERR_START_CAVA: &str = "[ERROR] Cannot start Cava script!";
pub const ERR_CREATE_RT: &str = "[ERROR] Couldn't create the Tokio runtime!";
pub const ERR_SEND_MSG_UI: &str = "[ERROR] Couldn't send tray messages to the UI!";
pub const ERR_ACCESS_CONFIG: &str = "[ERROR] Couldn't access CONFIG!";
pub const ERR_WRONG_LABEL_RANIM: &str =
    "[ERROR] Invalid revealer animation! Use `crossfade`, `slide_left` or `slide_right`.";
pub const ERR_READING_MAIN_RN: &str = "[ERROR] Couldn't find (or failed reading) main.rn!";

// Constant warnings.

pub const WARN_CAVA_NO_LINES: &str = "[WARN] Cava: There are no more lines available. Hybrid will keep on running but Cava will be stopped!";
pub const WARN_CAVA_NO_BARS_INSTANCE: &str = "[WARN] Cava: Failed accessing cava::BARS, stopping!";
pub const WARN_CAVA_NO_CRASHED_INSTANCE: &str =
    "[WARN] Cava: Failed accessing cava::HAS_CAVA_CRASHED, stopping!";
pub const WARN_NO_MAIN: &str = "[WARN] No `main` function found, skipping call.";
pub const WARN_NO_TICK: &str = "[WARN] No `tick` function found, skipping loop.";
