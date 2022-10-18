use crate::{
    button_widget::ButtonWidget, debug::log, r#loop::update, spacing_widget::SpacingWidget, *,
};
use gtk::traits::*;
use std::{str::FromStr, sync::Mutex};

lazy_static! {
    /// Holds all the dynamic label widgets.
    pub static ref VEC: Mutex<Vec<LabelWidget>> = {
        let v = Vec::new();
        Mutex::new(v)
    };
}

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

    // Prepare all of the widgets.
    create_components(&left, &centered, &right);
    // Make every widget visible.
    window.show_all();
    // Update dynamic content.
    update();
}

/// Gets the widget name for a specific key.
fn get_widget_name(identifiers: Vec<&str>, separator: char, count: usize) -> String {
    // Stores the unique widget name temporarily.
    let mut widget_name = String::default();
    for i in 1..count {
        widget_name.push_str(identifiers[i]);
        // Only add '_' to the end if the remaining amount of items isn't 1.
        if i != count - 1 {
            widget_name.push(separator);
        }
    }

    widget_name
}

/// Creates all of the widgets.
fn create_components(left: &Box, centered: &Box, right: &Box) {
    // Add all of the widgets defined from the config.
    const ALIGNMENT: char = '-';
    const SEPARATOR: char = '_';
    for (key, _) in config::read_config().entries() {
        if !key.contains(ALIGNMENT) || !key.contains(SEPARATOR) {
            continue;
        }

        // Gets the amount of entires in the split key.
        let count = key.split(SEPARATOR).count();

        // Gets the widget identifiers.
        let identifiers = key.split(SEPARATOR).collect::<Vec<&str>>();

        // Identifier example: `left-label_ABC` <= `left-label` is the IDENTIFIER, `ABC` is the NAME.
        let identifier = identifiers[0];

        // Grabs the widget alignment.
        let widget_alignment = key
            .split(ALIGNMENT)
            .nth(0)
            .expect("[ERROR] Invalid widget alignment!\n")
            .to_uppercase();

        // Base keys, text and command being optional.
        let text = config::try_get::<String>(key, "text").0;
        let command = config::try_get::<String>(key, "command").0;
        let alignment = structures::Align::from_str(&widget_alignment)
            .expect("[ERROR] Invalid widget alignment!\n");

        log(format!(
            "Adding widget '{identifier}' with alignment '{widget_alignment}'",
        ));

        let widget_name = get_widget_name(identifiers, SEPARATOR, count);

        // Check for identifiers.
        // Defo. not clean or pretty, will probably fix it later.
        if identifier.contains("label") {
            let label = LabelWidget {
                name: widget_name,
                text,
                command,
                label: Label::new(None),
            };

            label.add(alignment, left, centered, right)
        } else if identifier.contains("button") {
            let button = ButtonWidget {
                name: widget_name,
                command,
                button: Button::with_label(&text),
            };

            button.add(alignment, left, centered, right)
        } else if identifier.contains("spacing") {
            let spacing = SpacingWidget {
                name: widget_name,
                spacing_start: config::try_get::<i32>(key, "spacing_start").1,
                spacing_end: config::try_get::<i32>(key, "spacing_end").1,
            };

            spacing.add(alignment, left, centered, right)
        } else {
            // You are stupid.
            panic!("[ERROR] There are no widgets identified as '{identifier}'!\n")
        }
    }
}
