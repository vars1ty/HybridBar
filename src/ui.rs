use crate::{
    box_widget::BoxWidget, button_widget::ButtonWidget, cava_widget::CavaWidget, r#loop::update,
    spacing_widget::SpacingWidget, structures::Align, *,
};
use gtk::traits::*;
use std::{str::FromStr, sync::Mutex};

lazy_static! {
    /// Holds all the dynamic label widgets.
    pub static ref VEC: Mutex<Vec<LabelWidget>> = {
        let v = Vec::new();
        Mutex::new(v)
    };

    /// All active cava label instances.
    pub static ref CAVA_INSTANCES: Mutex<Vec<CavaWidget>> = {
        let v = Vec::new();
        Mutex::new(v)
    };
}

/// Adds and aligns the specified widget.
pub fn add_and_align(
    widget: &impl IsA<Widget>,
    align: Align,
    left: &Box,
    centered: &Box,
    right: &Box,
) {
    match align {
        Align::LEFT => left.add(widget),
        Align::CENTERED => centered.add(widget),
        Align::RIGHT => right.add(widget),
    }
}

/// Builds all of the widgets.
pub fn build_widgets(window: &ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let root = Box::new(Orientation::Horizontal, 0);
    let left = Box::new(Orientation::Horizontal, 0);
    let centered = Box::new(Orientation::Horizontal, 0);
    let right = Box::new(Orientation::Horizontal, 0);

    // 0.2.5: Root expands across the entire bar, previously "left" would do this but it isn't
    //   ideal when customizing, since borders would draw on the entire bar rather than just on the
    //   left portion of the bar.
    root.set_widget_name("root");

    // 0.2.5: Allow for customizing left, centered and right.
    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");

    // Add and align both centered and right.
    root.set_center_widget(Some(&centered));
    root.pack_end(&right, false, true, 0);

    // Add only left because centered and right are implicitly added above.
    root.add(&left);

    // Add root to the main canvas before finally adding all the widgets and drawing it.
    window.add(&root);

    // Prepare all of the widgets.
    create_components(&left, &centered, &right);
    // Make every widget visible.
    window.show_all();
    // Update dynamic content.
    update();
}

/// Creates all of the widgets.
fn create_components(left: &Box, centered: &Box, right: &Box) {
    // Add all of the widgets defined from the config.
    const ALIGNMENT: char = '-';
    const SEPARATOR: char = '_';
    let mut has_started_cava = false;
    for (key, _) in config::read_config().entries() {
        if !key.contains(ALIGNMENT) || !key.contains(SEPARATOR) {
            continue;
        }

        // Gets the widget identifiers.
        let identifiers = key.split(SEPARATOR).collect::<Vec<&str>>();

        // Identifier example: `left-label_ABC` <= `left-label` is the IDENTIFIER, `ABC` is the NAME.
        let identifier = identifiers[0];

        // Grabs the widget alignment.
        let widget_alignment = key
            .split(ALIGNMENT)
            .next()
            .expect("[ERROR] Invalid widget alignment!\n")
            .to_uppercase();

        // Base keys, text and command being optional.
        let text = config::try_get(key, "text", true).0;
        let command = config::try_get(key, "command", true).0;
        let tooltip = config::try_get(key, "tooltip", true).0;
        let alignment = structures::Align::from_str(&widget_alignment)
            .expect("[ERROR] Invalid widget alignment!\n");

        log!(format!(
            "Adding widget '{identifier}' with alignment '{widget_alignment}'",
        ));

        // Gets every element after the widget identifier, then appends '_' in between.
        let widget_name = identifiers[1..].join("_").to_string();

        // Check for identifiers.
        // Defo. not clean or pretty, will probably fix it later.
        if identifier.contains("label") {
            let label = LabelWidget {
                name: widget_name,
                tooltip,
                text,
                command,
                label: Label::new(None),
            };

            label.add(alignment, left, centered, right)
        } else if identifier.contains("button") {
            let button = ButtonWidget {
                name: widget_name,
                tooltip,
                command,
                button: Button::with_label(&text),
            };

            button.add(alignment, left, centered, right)
        } else if identifier.contains("spacing") {
            let spacing = SpacingWidget {
                name: widget_name,
                spacing_start: config::try_get(key, "spacing_start", false).1,
                spacing_end: config::try_get(key, "spacing_end", false).1,
            };

            spacing.add(alignment, left, centered, right)
        } else if identifier.contains("box") {
            let box_widget = BoxWidget {
                name: widget_name,
                width: config::try_get(key, "width", false).1,
            };

            box_widget.add(alignment, left, centered, right)
        } else if identifier.contains("cava") {
            let cava = CavaWidget {
                name: widget_name,
                label: Label::new(None),
            };

            if !has_started_cava {
                // Ensure it only calls update_bars once.
                r#loop::update_bars();
                has_started_cava = true;
            }
            log!("CAVA BETA WIDGET ACTIVE");
            cava.add(alignment, left, centered, right)
        } else {
            // You are stupid.
            panic!("[ERROR] There are no widgets identified as '{identifier}'!\n")
        }
    }
}
