use crate::{
    constant_messages::{INVALID_IDENTIFIER, INVALID_WIDGET_ALIGNMENT, INVALID_WIDGET_IDENTIFIER},
    debug::log,
    r#loop::update,
    *,
};
use gtk::traits::*;
use std::str::FromStr;

/// Static mutable Vector, because I'm not dealing with lifetime bullshit one more fucking minute.
pub static mut VEC: Option<Vec<LabelWidget>> = None;

/// Key separator.
const ALIGNMENT: char = '-';

/// Identifier separator.
const SEPARATOR: char = '_';

/// Builds all of the widgets.
pub fn build_widgets(window: &gtk::ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let left = Box::new(Orientation::Horizontal, 0);
    let centered = Box::new(Orientation::Horizontal, 0);
    let right = Box::new(Orientation::Horizontal, 0);

    // Add and align all of the box widgets.
    left.set_center_widget(Some(&centered));
    left.pack_end(&right, false, true, 0);
    window.add(&left);

    // Create the Vector.
    unsafe { VEC = Some(Vec::new()) }

    // Prepare all of the widgets.
    create_components(&left, &centered, &right);
    // Make every widget visible.
    window.show_all();
    // Update dynamic content.
    unsafe {
        update();
    }
}

/// Creates all of the widgets.
fn create_components(left: &Box, centered: &Box, right: &Box) {
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
            // Only add '_' to the end if the remaining amount of items isn't 1.
            if i != count - 1 {
                widget_name.push(SEPARATOR);
            }
        }

        // Base keys, text and command being optional.
        let text = config::try_get::<String>(key, "text").unwrap().0;
        let command = config::try_get::<String>(key, "command").unwrap().0;
        let alignment = structures::Align::from_str(&widget_alignment).expect(INVALID_IDENTIFIER);

        log(format!(
            "Adding widget '{identifier}' with alignment '{widget_alignment}'"
        ));

        // Check for identifiers.
        // Defo. not clean or pretty, will probably fix it later.
        if identifier.contains("label") {
            let label = LabelWidget {
                name: widget_name,
                text,
                command,
                label: Label::new(None),
            };

            label.add(alignment, left, centered, right);

            /*
            } else if identifier.contains("button") {
                widget_structure.create_button(&widget_name);
                widget_builder::add_button(&render_boxes, widget_structure, e_alignment)
            } else if identifier.contains("spacing") {
                widget_structure.create_spacing(
                    config::try_get::<i32>(key, "spacing_start").unwrap().1,
                    config::try_get::<i32>(key, "spacing_end").unwrap().1,
                );
                widget_builder::add_box(&render_boxes, widget_structure, e_alignment)
            }
            */
        }
    }
}
