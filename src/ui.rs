use crate::{
    box_widget::BoxWidget, button_widget::ButtonWidget, cava_widget::CavaWidget,
    cmd_widget::CmdWidget, r#loop::update, spacing_widget::SpacingWidget, structures::Align, *,
};
use gtk::traits::*;
use heapless::Vec;
use std::{str::FromStr, sync::Mutex};

lazy_static! {
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

/// Gets the base key values.
fn get_base_keys(root: &str) -> (String, String, u64, String, String) {
    let text = config::try_get(root, "text", true, true)
        .string
        .unwrap_or_default();
    let command = config::try_get(root, "command", true, true)
        .string
        .unwrap_or_default();
    let update_rate: u64 = config::try_get(root, "update_rate", false, false)
        .number
        .unwrap_or(100)
        .try_into()
        .unwrap_or_else(|_| {
            panic!("[ERROR] Couldn't convert update_rate to u64! Source: {root}\n")
        });
    let tooltip = config::try_get(root, "tooltip", true, true)
        .string
        .unwrap_or_default();
    let tooltip_command = config::try_get(root, "tooltip_command", true, true)
        .string
        .unwrap_or_default();
    (text, command, update_rate, tooltip, tooltip_command)
}

/// Creates all of the widgets.
fn create_components(left: &Box, centered: &Box, right: &Box) {
    // Add all of the widgets defined from the config.
    const ALIGNMENT: char = '-';
    const SEPARATOR: &str = "_";
    let mut has_started_cava = false;
    for (key, _) in config::CONFIG.read().unwrap().entries() {
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
        let update_rate = base_keys.2;
        let tooltip = base_keys.3;
        let tooltip_command = base_keys.4;
        let alignment = structures::Align::from_str(&f_widget_alignment)
            .expect("[ERROR] Invalid widget alignment!\n");

        // Gets every element after the widget identifier, then appends '_' in between.
        let widget_name = identifiers[1..].join(SEPARATOR).to_string();

        if widget_name.is_empty() {
            // JSON doesn't play nicely with duplicate keys, will probably end up making a custom
            // format + parser later on.
            // Closes issue #14.
            panic!("[ERROR] Found an empty widget name, this is not currently supported!\n")
        }

        log!(format!(
            "Adding widget '{identifier}' with alignment '{f_widget_alignment}'",
        ));

        // Add the widget.
        // TODO: Redo this through a structure or similar.
        add_widget(
            key,
            (widget_type, widget_name),
            (text, command, update_rate, tooltip, tooltip_command),
            alignment,
            (left, centered, right),
            identifier,
            &mut has_started_cava,
        )
    }
}

/// Add a new widget of specified identifier.
// This uses tuples for several parameters to get around the "max parameters" limitation.
// Plus, it looks nicer.
fn add_widget(
    key: &str,
    widget_pkg: (&str, String),
    base_keys: (String, String, u64, String, String),
    alignment: Align,
    left_centered_right: (&Box, &Box, &Box),
    identifier: &str,
    has_started_cava: &mut bool,
) {
    // Extract name and type.
    let widget_type = widget_pkg.0;
    let widget_name = widget_pkg.1;

    // Extract text, command, tooltip and tooltip_command.
    let text = base_keys.0;
    let command = base_keys.1;
    let update_rate = base_keys.2;
    let tooltip = base_keys.3;
    let tooltip_command = base_keys.4;

    // Extract left, centered and right.
    let left = left_centered_right.0;
    let centered = left_centered_right.1;
    let right = left_centered_right.2;

    match widget_type {
        "label" => {
            let label = LabelWidget {
                update_rate,
                tooltip,
                tooltip_command,
                text,
                command,
                label: Label::new(None),
                listen: config::try_get(key, "listen", true, false)
                    .string
                    .unwrap_or_default()
                    == "true",
            };

            label.add(widget_name, alignment, left, centered, right)
        }
        "button" => {
            let button = ButtonWidget {
                tooltip,
                tooltip_command,
                command,
                button: Button::with_label(&text),
            };

            button.add(widget_name, alignment, left, centered, right)
        }
        "spacing" => {
            let spacing = SpacingWidget {
                spacing_start: config::try_get(key, "spacing_start", false, false)
                    .number
                    .unwrap_or_default(),
                spacing_end: config::try_get(key, "spacing_end", false, false)
                    .number
                    .unwrap_or_default(),
            };

            spacing.add(widget_name, alignment, left, centered, right)
        }
        "box" => {
            let box_widget = BoxWidget {
                width: config::try_get(key, "width", false, false)
                    .number
                    .unwrap_or_default(),
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
