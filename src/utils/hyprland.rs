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
        window: get_active_window_title(),
    }
}

/// Gets and returns the active window title
fn get_active_window_title() -> String {
    match Client::get_active().unwrap() {
        Some(window) => window.title,
        None => String::default(),
    }
}