use crate::{
    constant_messages::{
        CANNOT_ACCESS_BOX, CANNOT_ACCESS_BUTTON, CANNOT_ACCESS_LABEL, CANNOT_ACCESS_VEC,
    },
    debug::log,
    proc,
    structures::{self, GTKWidget},
    ui::VEC,
};
use glib::IsA;
use gtk::{traits::*, *};

/// Boxes used to render content.
pub struct RenderBoxes {
    pub draw_left: Box,
    pub draw_centered: Box,
    pub draw_right: Box,
}

/// Adds a box.
pub fn add_box(
    render_boxes: &RenderBoxes,
    gtk_widget_structure: GTKWidget,
    align: structures::Align,
) {
    // The values and such is all set from `loop.rs`.
    let w_box = gtk_widget_structure
        .spacing
        .as_ref()
        .expect(CANNOT_ACCESS_BOX);

    log("Adding label");
    add(w_box, render_boxes, align);
}

/// Adds a button.
pub fn add_button(
    render_boxes: &RenderBoxes,
    gtk_widget_structure: GTKWidget,
    align: structures::Align,
) {
    // The values and such is all set from `loop.rs`.
    let button = gtk_widget_structure
        .button
        .as_ref()
        .expect(CANNOT_ACCESS_BUTTON);

    button.set_label(&gtk_widget_structure.properties.text);
    let c_command = gtk_widget_structure.properties.command.clone();
    if !c_command.is_empty() {
        button.connect_clicked(move |_| {
            log("Button.connect_clicked()");
            proc::execute(c_command.clone());
        });
    }

    log("Adding button");
    add(button, render_boxes, align);
    // Buttons don't need to exist inside of the Vector list, since there's nothing to redraw nor
    // update.
}

/// Adds a label.
pub fn add_label(
    render_boxes: &RenderBoxes,
    gtk_widget_structure: GTKWidget,
    align: structures::Align,
) {
    // The values and such is all set from `loop.rs`.
    let label = gtk_widget_structure
        .label
        .as_ref()
        .expect(CANNOT_ACCESS_LABEL);

    log("Adding label");
    label.set_text(&gtk_widget_structure.properties.text);
    add(label, render_boxes, align);
    unsafe {
        // If the command is empty, there is no need to add it to the VEC list.
        // Since it won't have to be redrawn.
        if !gtk_widget_structure.properties.command.is_empty() {
            VEC.as_mut()
                .expect(CANNOT_ACCESS_VEC)
                .push(gtk_widget_structure);
        }
    }
}

/// Adds a widget and aligns it automatically.
fn add(widget: &impl IsA<Widget>, render_boxes: &RenderBoxes, align: structures::Align) {
    match align {
        structures::Align::LEFT => render_boxes.draw_left.add(widget),
        structures::Align::CENTERED => render_boxes.draw_centered.add(widget),
        structures::Align::RIGHT => render_boxes.draw_right.add(widget),
    }
}

/// Creates a standard Box widget with horizontal orientation.
pub fn create_box() -> Box {
    Box::new(Orientation::Horizontal, 0)
}
