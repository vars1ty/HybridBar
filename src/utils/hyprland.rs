use hyprland::{
    data::{Client, Workspace},
    shared::{HyprDataActive, HyprDataActiveOptional},
};
use rune::Any;

/// Hyprland data.
#[derive(Debug, Any)]
pub struct HyprlandData {
    pub workspace: i32,
    pub window: String,
}

impl HyprlandData {
    /// Gets the workspace and active window.
    pub fn get_data() -> HyprlandData {
        HyprlandData {
            workspace: Workspace::get_active().unwrap().id,
            window: if let Some(window) = Client::get_active().unwrap() {
                window.title
            } else {
                String::default()
            },
        }
    }
}