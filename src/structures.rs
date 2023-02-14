use strum::EnumString;

use crate::types::MediumString;

/// Fetched config data.
#[derive(Default)]
pub struct ConfigData {
    pub string: Option<MediumString>,
    pub number: Option<i32>,
}

/// Implements `new` for Config Data.
impl ConfigData {
    /// Creates a new Config Data instance and returns it.
    pub fn new(string: Option<MediumString>, number: Option<i32>) -> ConfigData {
        ConfigData { string, number }
    }
}

/// Base keys.
pub struct BaseKeys {
    pub text: MediumString,
    pub command: MediumString,
    pub update_rate: u64,
    pub tooltip: MediumString,
    pub tooltip_command: MediumString,
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
