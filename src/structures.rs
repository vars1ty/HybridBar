use gtk::RevealerTransitionType;

use crate::widget::Align;

/// Fetched config data.
#[derive(Default)]
pub struct ConfigData {
    pub string: Option<String>,
    pub number: Option<i32>,
}

/// Root, Left, Centered and Right boxes.
pub struct WidgetHolders {
    pub root: gtk::Box,
    pub left: gtk::Box,
    pub centered: gtk::Box,
    pub right: gtk::Box,
}

unsafe impl Send for WidgetHolders {}
unsafe impl Sync for WidgetHolders {}

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

pub trait RevealerExtensions {
    fn from_str(string: &str) -> Option<RevealerTransitionType>;
}

impl RevealerExtensions for RevealerTransitionType {
    /// Tries to get the transition type based on the string input.
    /// Note: The string has to be lowercase and spaces replaced with underscores.
    /// This can only return `Crossfade`, `SlideLeft` and `SlideRight`.
    fn from_str(string: &str) -> Option<RevealerTransitionType> {
        match string {
            "crossfade" => Some(RevealerTransitionType::Crossfade),
            "slide_left" => Some(RevealerTransitionType::SlideLeft),
            "slide_right" => Some(RevealerTransitionType::SlideRight),
            _ => None,
        }
    }
}
