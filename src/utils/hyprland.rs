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
            workspace: Workspace::get_active()
                .expect("[ERROR] Workspace::get_active() failed!")
                .id,
            window: if let Some(window) = Client::get_active().unwrap() {
                window.title
            } else {
                String::default() // TODO: Remove allocation (somehow?)
            },
        }
    }

    /// Returns the active workspace.
    pub fn get_current_workspace() -> i32 {
        Self::get_data().workspace
    }

    /// Returns the active window.
    pub fn get_current_window() -> String {
        Self::get_data().window
    }
}
