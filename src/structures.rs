use crate::widget::Align;
use gtk::RevealerTransitionType;

/// Root, Left, Centered and Right boxes.
pub struct WidgetHolders {
    pub root: gtk::Box,
    pub left: gtk::Box,
    pub centered: gtk::Box,
    pub right: gtk::Box,
}

unsafe impl Send for WidgetHolders {}
unsafe impl Sync for WidgetHolders {}

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
    fn from_str(string: &str) -> RevealerTransitionType;
}

impl RevealerExtensions for RevealerTransitionType {
    /// Tries to get the transition type based on the string input.
    /// Note: The string has to be lowercase and spaces replaced with underscores.
    /// This can only return `Crossfade`, `SlideLeft` and `SlideRight`.
    fn from_str(string: &str) -> RevealerTransitionType {
        match string {
            "crossfade" => RevealerTransitionType::Crossfade,
            "slide_left" => RevealerTransitionType::SlideLeft,
            "slide_right" => RevealerTransitionType::SlideRight,
            _ => RevealerTransitionType::None,
        }
    }
}
