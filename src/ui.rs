use crate::config;
use crate::r#loop::update;
use gtk::{traits::*, Box, Label};
use std::collections::HashMap;

/// Static mutable HashMap, because I'm not dealing with lifetime bullshit one more fucking minute.
pub static mut MAP: Option<HashMap<Label, DynamicWidget>> = None;

/// Identifier separator.
const SEPARATOR: char = '_';

/// Dynamic Widget structure.
pub struct DynamicWidget {
    pub text: String,
    pub command: String,
}

/// Widget alignment.
enum Align {
    LEFT,
    CENTER,
    RIGHT,
}

/// Builds all of the widgets.
pub fn build_widgets(window: &gtk::ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let draw = create_box();
    let draw_centered = create_box();
    let draw_right = create_box();

    // Add and align all of the box widgets.
    draw.set_center_widget(Some(&draw_centered));
    draw.pack_end(&draw_right, false, true, 0);
    window.add(&draw);

    // Create the HashMap.
    unsafe {
        MAP = Some(HashMap::new());
    }

    // Prepare all of the widgets.
    pre_create(&draw, &draw_right, &draw_centered);
    // Make every widget visible
    window.show_all();
    // Update dynamic content.
    update();
}

/// Prepares everything for widgets.
fn pre_create(draw: &Box, draw_right: &Box, draw_centered: &Box) {
    // Add all of the widgets defined from the config.
    for (key, value) in config::read_config().entries() {
        if !key.contains(SEPARATOR) {
            continue;
        }

        // Identifiers are the first part of the JSON Key, for example:
        // label_hello <= "label" being the identifier, "hello" being the name.
        let identifier = key
            .split(SEPARATOR)
            .nth(0)
            .expect("[ERROR] Failed splitting key!\n");

        // Create the structure.
        let structure = DynamicWidget {
            text: value["text"].to_string(),
            command: value["command"].to_string(),
        };

        // Check for identifiers.
        match identifier {
            "centered-label" => {
                create_labels(&draw, &draw_centered, &draw_right, structure, Align::CENTER)
            }
            "label" => create_labels(&draw, &draw_centered, &draw_right, structure, Align::LEFT),
            "right-label" => {
                create_labels(&draw, &draw_centered, &draw_right, structure, Align::RIGHT)
            }
            _ => panic!(
                "[ERROR] Invalid identifier! You can only use [ centered-label / label / right-label ]\n"
            ),
        }
    }
}

/// Creates all of the labels.
fn create_labels(
    draw: &Box,
    draw_centered: &Box,
    draw_right: &Box,
    dynamic_widget: DynamicWidget,
    align: Align,
) {
    // The values and such is all set from `loop.rs`.
    let label = Label::new(None);
    match align {
        Align::LEFT => draw.add(&label),
        Align::CENTER => draw_centered.add(&label),
        Align::RIGHT => draw_right.add(&label),
    }

    unsafe {
        MAP.as_mut()
            .expect("[ERROR] Failed accessing MAP!\n")
            .insert(label, dynamic_widget);
    }
}

/// Creates a standard Box widget with horizontal orientation.
fn create_box() -> Box {
    Box::new(gtk::Orientation::Horizontal, 0)
}
