use strum::EnumString;

/// Fetched config data.
#[derive(Default)]
pub struct ConfigData {
    pub string: Option<String>,
    pub number: Option<i32>,
}

/// Implements `new` for Config Data.
impl ConfigData {
    /// Creates a new Config Data instance and returns it.
    pub fn new(string: Option<String>, number: Option<i32>) -> ConfigData {
        ConfigData { string, number }
    }
}

/// Base keys.
pub struct BaseKeys {
    pub text: String,
    pub command: String,
    pub update_rate: u64,
    pub tooltip: String,
    pub tooltip_command: String,
    pub alignment: Align,
}

/// Widget alignment.
// Allow for uppercase enum namings here.
// TODO: Move this, or rename the file because this is no struct.
#[allow(clippy::upper_case_acronyms)]
#[derive(EnumString)]
pub enum Align {
    LEFT,
    CENTERED,
    RIGHT,
}
