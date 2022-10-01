// This file is for holding constant error messages.
// Dynamic messages (ones using format!(*)) are excluded.

pub const CANNOT_ACCESS_VEC: &str = "[ERROR] Failed to access VEC!\n";

pub const INVALID_IDENTIFIER: &str = "[ERROR] Invalid identifier passed!\n";
pub const INVALID_WIDGET_ALIGNMENT: &str = "[ERROR] Invalid (or missing) widget alignment!\n";
pub const INVALID_WIDGET_IDENTIFIER: &str = "[ERROR] Invalid (or missing) widget identifier!\n";
pub const INVALID_BAR_POSITION: &str =
    "[ERROR] Invalid position! Possible values: [ TOP, BOTTOM ]\n";

pub const MISSING_DISPLAY: &str = "[ERROR] Could not connect to a display, fix your PC.\n";

pub const FAILED_PAINTING: &str = "[ERROR] Failed painting!\n";
pub const FAILED_PARSING_CONFIG: &str = "[ERROR] Failed parsing config!\n";
pub const FAILED_PARSING_CONTENT: &str = "[ERROR] Failed parsing content as i32!\n";
