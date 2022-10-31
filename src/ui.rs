use crate::{
    box_widget::BoxWidget, button_widget::ButtonWidget, cava_widget::CavaWidget,
    cmd_widget::CmdWidget, r#loop::update, spacing_widget::SpacingWidget, structures::Align, *,
};
use gtk::traits::*;
use heapless::Vec;
use std::{str::FromStr, sync::Mutex};
use uuid::Uuid;

lazy_static! {
    /// Holds all the dynamic label widgets.
    pub static ref VEC: Mutex<Vec<LabelWidget, 1024>> = Mutex::new(Vec::new());

    /// All active cava label instances.
    // This will be moved to `cava.rs` soon.
    pub static ref CAVA_INSTANCES: Mutex<Vec<CavaWidget, 8>> = Mutex::new(Vec::new());
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

/// Gets the values for `text`, `command` and `tooltip`.
/// If one is left unspecified, the value is `"", 0`, a.k.a default.
fn get_base_keys(root: &str) -> (String, String, String) {
    let text = config::get_or_default(root, "text", true, true).0;
    let command = config::get_or_default(root, "command", true, true).0;
    let tooltip = config::get_or_default(root, "tooltip", true, true).0;
    (text, command, tooltip)
}

/// Creates all of the widgets.
fn create_components(left: &Box, centered: &Box, right: &Box) {
    // Add all of the widgets defined from the config.
    const ALIGNMENT: char = '-';
    const SEPARATOR: &str = "_";
    let mut has_started_cava = false;
    for (key, _) in config::read_config().entries() {
        if !key.contains(ALIGNMENT) || !key.contains(SEPARATOR) {
            continue;
        }

        // Gets the widget identifiers.
        let identifiers = key.split(SEPARATOR).collect::<Vec<&str, 8>>();

        // Identifier example: `left-label_ABC` <= `left-label` is the IDENTIFIER, `ABC` is the NAME.
        let identifier = identifiers[0];

        // Grabs widget alignment and widget type from the identifier separated by '-'.
        let (widget_alignment, widget_type) = identifier
            .split_once(ALIGNMENT)
            .expect("[ERROR] Widget should be named as [alignment]-[widget_type]_[name]\n");

        // Formats the widget alignment.
        let f_widget_alignment = widget_alignment.to_uppercase();

        // Base keys, all being optional.
        let base_keys = get_base_keys(key);
        let text = base_keys.0;
        let command = base_keys.1;
        let tooltip = base_keys.2;
        let alignment = structures::Align::from_str(&f_widget_alignment)
            .expect("[ERROR] Invalid widget alignment!\n");

        log!(format!(
            "Adding widget '{identifier}' with alignment '{f_widget_alignment}'",
        ));

        // Gets every element after the widget identifier, then appends '_' in between.
        let mut widget_name = identifiers[1..].join(SEPARATOR).to_string();

        if widget_name.is_empty() {
            log!("Found an empty widget name (probably discarded), replacing with a random UUID");
            widget_name = Uuid::new_v4().to_string()
        }

        // Add all of the widgets.
        add_widgets(
            key,
            (widget_type, widget_name),
            (text, command, tooltip),
            alignment,
            (left, centered, right),
            identifier,
            &mut has_started_cava,
        )
    }
}

/// Adds all of the widgets.
// This uses tuples for several parameters to get around the "max parameters" limitation.
// Plus, it looks nicer.
fn add_widgets(
    key: &str,
    widget_pkg: (&str, String),
    text_command_tooltip: (String, String, String),
    alignment: Align,
    left_centered_right: (&Box, &Box, &Box),
    identifier: &str,
    has_started_cava: &mut bool,
) {
    // Extract name and type.
    let widget_type = widget_pkg.0;
    let widget_name = widget_pkg.1;

    // Extract text, command and tooltip.
    let text = text_command_tooltip.0;
    let command = text_command_tooltip.1;
    let tooltip = text_command_tooltip.2;

    // Extract left, centered and right.
    let left = left_centered_right.0;
    let centered = left_centered_right.1;
    let right = left_centered_right.2;

    match widget_type {
        "label" => {
            let label = LabelWidget {
                tooltip,
                text,
                command,
                label: Label::new(None),
                listen: config::get_or_default(key, "listen", true, false).0 == "true",
            };

            label.add(widget_name, alignment, left, centered, right)
        }
        "button" => {
            let button = ButtonWidget {
                tooltip,
                command,
                button: Button::with_label(&text),
            };

            button.add(widget_name, alignment, left, centered, right)
        }
        "spacing" => {
            let spacing = SpacingWidget {
                spacing_start: config::get_or_default(key, "spacing_start", false, false).1,
                spacing_end: config::get_or_default(key, "spacing_end", false, false).1,
            };

            spacing.add(widget_name, alignment, left, centered, right)
        }
        "box" => {
            let box_widget = BoxWidget {
                width: config::get_or_default(key, "width", false, false).1,
            };

            box_widget.add(widget_name, alignment, left, centered, right)
        }
        "cava" => {
            let cava = CavaWidget {
                label: Label::new(None),
            };

            if !*has_started_cava {
                cava::update_bars();
                // Ensure it only calls update_bars once.
                *has_started_cava = true;
            }

            cava.add(widget_name, alignment, left, centered, right)
        }
        "cmd" => {
            let cmd = CmdWidget {};

            cmd.add(widget_name, alignment, left, centered, right)
        }
        _ => {
            panic!("[ERROR] There are no widgets identified as '{identifier}'!\n")
        }
    }
}
