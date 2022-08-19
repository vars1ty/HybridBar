use crate::{debug::debug_log, proc, ui::Align, ui::GTKWidget, ui::VEC};
use glib::IsA;
use gtk::{traits::*, *};

/// Error message to be displayed if we can't access ui::VEC.
const FAILED_ACCESSING_VEC: &str = "[ERROR] Failed to access VEC!\n";

/// Adds a box.
pub fn add_box(
    draw_left: &Box,
    draw_centered: &Box,
    draw_right: &Box,
    gtk_widget_structure: GTKWidget,
    align: Align,
) {
    // The values and such is all set from `loop.rs`.
    let w_box = gtk_widget_structure
        .spacing
        .as_ref()
        .expect("[ERROR] Failed to access Box!\n");

    debug_log("Adding label");
    add(w_box, draw_left, draw_centered, draw_right, align);
}

/// Adds a button.
pub fn add_button(
    draw_left: &Box,
    draw_centered: &Box,
    draw_right: &Box,
    gtk_widget_structure: GTKWidget,
    align: Align,
) {
    // The values and such is all set from `loop.rs`.
    let button = gtk_widget_structure
        .button
        .as_ref()
        .expect("[ERROR] Failed to access Button!\n");

    button.set_label(&gtk_widget_structure.properties.text);
    let c_command = gtk_widget_structure.properties.command.clone();
    if !c_command.is_empty() {
        button.connect_clicked(move |_| {
            proc::execute(c_command.to_string());
        });
    }

    debug_log("Adding button");
    add(button, draw_left, draw_centered, draw_right, align);
    unsafe {
        VEC.as_mut()
            .expect(FAILED_ACCESSING_VEC)
            .push(gtk_widget_structure);
    }
}

/// Adds a label.
pub fn add_label(
    draw_left: &Box,
    draw_centered: &Box,
    draw_right: &Box,
    gtk_widget_structure: GTKWidget,
    align: Align,
) {
    // The values and such is all set from `loop.rs`.
    let label = gtk_widget_structure
        .label
        .as_ref()
        .expect("[ERROR] Failed to access Label!\n");

    debug_log("Adding label");
    add(label, draw_left, draw_centered, draw_right, align);
    unsafe {
        // If the command is empty, there is no need to add it to the VEC list.
        // Since it won't have to be redrawn.
        if !gtk_widget_structure.properties.command.is_empty() {
            VEC.as_mut()
                .expect(FAILED_ACCESSING_VEC)
                .push(gtk_widget_structure);
        }
    }
}

/// Adds a widget and aligns it automatically.
fn add(
    widget: &impl IsA<Widget>,
    draw_left: &Box,
    draw_centered: &Box,
    draw_right: &Box,
    align: Align,
) {
    match align {
        Align::LEFT => draw_left.add(widget),
        Align::CENTERED => draw_centered.add(widget),
        Align::RIGHT => draw_right.add(widget),
    }
}

/// Creates a standard Box widget with horizontal orientation.
pub fn create_box() -> Box {
    Box::new(gtk::Orientation::Horizontal, 0)
}
