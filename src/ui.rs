use crate::{
    constant_messages::{INVALID_IDENTIFIER, INVALID_WIDGET_ALIGNMENT, INVALID_WIDGET_IDENTIFIER},
    debug::log,
    r#loop::update,
    structures::{GTKWidget, WidgetProperties},
    widget_builder::RenderBoxes,
    *,
};
use gtk::traits::*;
use std::str::FromStr;

/// Static mutable HashMap, because I'm not dealing with lifetime bullshit one more fucking minute.
pub static mut VEC: Option<Vec<GTKWidget>> = None;

/// Key separator.
const ALIGNMENT: char = '-';

/// Identifier separator.
const SEPARATOR: char = '_';

/// Builds all of the widgets.
pub fn build_widgets(window: &gtk::ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let left = widget_builder::create_box();
    let centered = widget_builder::create_box();
    let right = widget_builder::create_box();

    // Add and align all of the box widgets.
    left.set_center_widget(Some(&centered));
    left.pack_end(&right, false, true, 0);
    window.add(&left);

    // Create the Vector.
    unsafe { VEC = Some(Vec::new()) }

    // Create the Render Boxes structure.
    let render_boxes = RenderBoxes {
        draw_left: left,
        draw_centered: centered,
        draw_right: right,
    };

    // Prepare all of the widgets.
    create_components(&render_boxes);
    // Make every widget visible.
    window.show_all();
    // Update dynamic content.
    unsafe {
        update();
    }
}

/// Creates all of the widgets.
fn create_components(render_boxes: &RenderBoxes) {
    // Add all of the widgets defined from the config.
    for (key, _) in config::read_config().entries() {
        if !key.contains(ALIGNMENT) || !key.contains(SEPARATOR) {
            continue;
        }

        // Gets the amount of entires in the split key.
        let count = key.split(SEPARATOR).count();

        // Gets the widget identifier.
        // For example: `left-label_ABC` <= `left-label` is the IDENTIFIER, `ABC` is the NAME.
        let identifier = key
            .split(SEPARATOR)
            .nth(0)
            .expect(INVALID_WIDGET_IDENTIFIER);

        // Grabs the widget alignment.
        let widget_alignment = key
            .split(ALIGNMENT)
            .nth(0)
            .expect(INVALID_WIDGET_ALIGNMENT)
            .to_uppercase();

        // Stores the unique widget name temporarily.
        let mut widget_name = String::default();
        for i in 1..count {
            widget_name.push_str(key.split(SEPARATOR).nth(i).unwrap());
            // Only add '_' to the end if we haven't reached the end.
            if i != count - 1 {
                widget_name.push(SEPARATOR);
            }
        }

        // Create the properties structure.
        let widget_properties = WidgetProperties {
            text: config::try_get::<String>(key, "text").unwrap().0,
            command: config::try_get::<String>(key, "command").unwrap().0,
        };

        // Create the widget structure.
        let mut widget_structure = GTKWidget {
            button: None,
            label: None,
            spacing: None,
            properties: widget_properties,
        };

        // The alignment grabbed.
        // If no valid alignment was found, panic.
        let e_alignment = structures::Align::from_str(&widget_alignment).expect(INVALID_IDENTIFIER);

        // Debug messages.
        log(format!(
            "Adding widget '{identifier}' with alignment '{widget_alignment}'"
        ));

        // Check for identifiers.
        // Defo. not clean or pretty, will probably fix it later.
        if identifier.contains("label") {
            widget_structure.create_label(&widget_name);
            widget_builder::add_label(render_boxes, widget_structure, e_alignment)
        } else if identifier.contains("button") {
            widget_structure.create_button(&widget_name);
            widget_builder::add_button(render_boxes, widget_structure, e_alignment)
        } else if identifier.contains("spacing") {
            widget_structure.create_spacing(
                config::try_get::<i32>(key, "spacing_start").unwrap().1,
                config::try_get::<i32>(key, "spacing_end").unwrap().1,
            );
            widget_builder::add_box(render_boxes, widget_structure, e_alignment)
        }
    }
}
