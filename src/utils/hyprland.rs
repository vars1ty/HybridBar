use hyprland::{
    data::{Client, Workspace},
    shared::{HyprDataActive, HyprDataActiveOptional},
};

/// Hyprland data.
pub struct HyprlandData {
    pub workspace: i32,
    pub window: String,
}

/// Gets the workspace and active window.
pub fn get_data() -> HyprlandData {
    HyprlandData {
        workspace: Workspace::get_active().unwrap().id,
        window: Client::get_active().unwrap().unwrap().title,
    }
}
