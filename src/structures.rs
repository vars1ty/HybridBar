use crate::widget::Align;

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
