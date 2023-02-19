use std::str::FromStr;

use crate::{
    cava::HAS_CAVA_STARTED,
    config::{get_custom_variables, with_variables, CONFIG},
    r#loop::update,
    structures::BaseKeys,
    *,
};
use crate::{
    widget::{Align, HWidget},
    widgets::{
        box_widget::BoxWidget, button_widget::ButtonWidget, cava_widget::CavaWidget,
        label_widget::LabelWidget, spacing_widget::SpacingWidget, tray_widget::TrayWidget,
    },
};
use gtk::traits::*;

/// Adds and aligns the specified widget.
pub fn add_and_align(
    widget: &impl IsA<Widget>,
    align: Align,
    left: &Box,
    centered: &Box,
    right: &Box,
    box_holder: Option<&Box>,
) {
    if let Some(r#box) = box_holder {
        r#box.add(widget)
    } else {
        match align {
            Align::LEFT => left.add(widget),
            Align::CENTERED => centered.add(widget),
            Align::RIGHT => right.add(widget),
        }
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
pub fn get_base_keys(root: &JsonValue) -> (String, String, u64, String, String) {
    let custom_variables = &get_custom_variables();
    let text = with_variables(
        root["text"].as_str().unwrap_or_default().to_owned(),
        custom_variables,
    );
    let command = with_variables(
        root["command"].as_str().unwrap_or_default().to_owned(),
        custom_variables,
    );
    let update_rate: u64 = root["update_rate"]
        .as_i32()
        .unwrap_or(100)
        .try_into()
        .unwrap_or_else(|_| panic!("[ERROR] Couldn't convert update_rate to u64! Source: {root}"));
    let tooltip = with_variables(
        root["tooltip"].as_str().unwrap_or_default().to_owned(),
        custom_variables,
    );
    let tooltip_command = with_variables(
        root["tooltip_command"]
            .as_str()
            .unwrap_or_default()
            .to_owned(),
        custom_variables,
    );
    (text, command, update_rate, tooltip, tooltip_command)
}

/// Creates all of the widgets.
fn create_components(left: &Box, centered: &Box, right: &Box) {
    // Add all of the widgets defined from the config.
    if let Ok(cfg) = CONFIG.read() {
        const ALIGNMENT: char = '-';
        const SEPARATOR: &str = "_";
        let relevant = cfg
            .entries()
            .filter(|(key, _)| key.contains(ALIGNMENT) && key.contains(SEPARATOR));

        for (key, json) in relevant {
            // Gets the widget identifiers.
            let identifiers: Vec<_> = key.split(SEPARATOR).collect();

            // Identifier example: `left-label_ABC` <= `left-label` is the IDENTIFIER, `ABC` is the NAME.
            let identifier = identifiers[0];

            // Grabs widget alignment and widget type from the identifier separated by '-'.
            let (widget_alignment, widget_type) = identifier
                .split_once(ALIGNMENT)
                .expect(ERR_INVALID_WIDGET_FORMAT);

            // Formats the widget alignment.
            let widget_alignment = widget_alignment.to_uppercase();

            // Base keys, all being optional.
            let (text, command, update_rate, tooltip, tooltip_command) = get_base_keys(json);
            let base_keys = BaseKeys {
                text,
                command,
                update_rate,
                tooltip,
                tooltip_command,
                alignment: Align::from_str(&widget_alignment)
                    .expect(ERR_INVALID_ALIGNMENT),
            };

            // Gets every element after the widget identifier, then appends '_' in between.
            let widget_name = identifiers[1..].join(SEPARATOR).to_owned();

            if widget_name.is_empty() {
                panic!("{}", ERR_EMPTY_NAME)
            }

            log!(format!(
                "Adding widget '{identifier}' with alignment '{widget_alignment}'!",
            ));

            // Add the widget.
            add_widget(
                json,
                (widget_type, &widget_name),
                base_keys,
                (left, centered, right),
                identifier,
                None,
            )
        }
    } else {
        panic!("{}", ERR_ACCESS_CONFIG)
    }
}

/// Add a new widget of specified identifier.
pub fn add_widget(
    key: &JsonValue,
    widget_pkg: (&str, &str),
    base_keys: BaseKeys,
    left_centered_right: (&Box, &Box, &Box),
    identifier: &str,
    box_holder: Option<&Box>,
) {
    // Extract name and type.
    let (widget_type, widget_name) = widget_pkg;

    // Extract data from the base keys.
    let text = base_keys.text;
    let command = base_keys.command;
    let update_rate = base_keys.update_rate;
    let tooltip = base_keys.tooltip;
    let tooltip_command = base_keys.tooltip_command;
    let alignment = base_keys.alignment;

    // Extract left, centered and right.
    let (left, centered, right) = left_centered_right;

    match widget_type {
        "label" => {
            let label = LabelWidget {
                update_rate,
                tooltip,
                tooltip_command,
                text,
                command,
                label: Label::new(None),
                listen: key["listen"].as_bool().unwrap_or_default(),
            };

            label.add(widget_name, alignment, left, centered, right, box_holder)
        }
        "button" => {
            let button = ButtonWidget {
                tooltip,
                tooltip_command,
                command,
                button: Button::with_label(&text),
            };

            button.add(widget_name, alignment, left, centered, right, box_holder)
        }
        "spacing" => {
            let spacing = SpacingWidget {
                spacing_start: key["spacing_start"].as_i32().unwrap_or_default(),
                spacing_end: key["spacing_end"].as_i32().unwrap_or_default(),
            };

            spacing.add(widget_name, alignment, left, centered, right, box_holder)
        }
        "box" => {
            let box_widget = BoxWidget {
                width: key["width"].as_i32().unwrap_or_default(),
                widgets: key["widgets"].to_owned(),
            };

            box_widget.add(widget_name, alignment, left, centered, right, box_holder)
        }
        "cava" => {
            let cava = CavaWidget {
                label: Label::new(None),
            };

            if let Ok(mut has_cava_started) = HAS_CAVA_STARTED.lock() {
                if !*has_cava_started {
                    cava::update_bars();
                    // Ensure it only calls update_bars once.
                    *has_cava_started = true
                }
            }

            cava.add(widget_name, alignment, left, centered, right, box_holder)
        }
        "tray" => TrayWidget.add(widget_name, alignment, left, centered, right, box_holder),
        _ => {
            panic!("[ERROR] There is no widget type defined as '{identifier}'!\n")
        }
    }
}
