use crate::{
    constants::ERR_EMPTY_NAME,
    structures::BaseKeys,
    ui,
    widget::{Align, HWidget},
};
use gtk::{traits::*, *};
use json::JsonValue;

/// Creates a new basic box widget.
pub struct BoxWidget {
    pub width: i32,
    pub widgets: JsonValue,
}

/// Builds the child widgets.
fn build_child_widgets(
    widgets: JsonValue,
    left: &Box,
    centered: &Box,
    right: &Box,
    box_holder: &Box,
) {
    const SEPARATOR: &str = "_";
    let relevant = widgets.entries().filter(|(key, _)| key.contains(SEPARATOR));

    for (key, json) in relevant {
        // Gets the widget identifiers.
        let identifiers: Vec<_> = key.split(SEPARATOR).collect();

        // Type example: `label_ABC` <= `label` is the IDENTIFIER, `ABC` is the NAME.
        let widget_type = identifiers[0];

        // Base keys.
        let (text, command, update_rate, tooltip, tooltip_command) = ui::get_base_keys(json);
        let base_keys = BaseKeys {
            text,
            command,
            update_rate,
            tooltip,
            tooltip_command,
            alignment: Align::Left, // <= Doesn't matter as it won't be used.
        };

        let widget_name = identifiers[1..].join(SEPARATOR);
        if widget_name.is_empty() {
            panic!("{}", ERR_EMPTY_NAME)
        }

        log!(format!(
            "Adding child widget '{widget_name}', type '{widget_type}' into '{}'!",
            box_holder.widget_name()
        ));

        // Add the widget.
        ui::add_widget(
            json,
            (widget_type, &widget_name),
            base_keys,
            (left, centered, right),
            widget_type,
            Some(box_holder),
        )
    }
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for BoxWidget {
    fn add(
        self,
        name: &str,
        align: Align,
        left: &Box,
        centered: &Box,
        right: &Box,
        box_holder: Option<&Box>,
    ) {
        let widget = Box::new(Orientation::Horizontal, 0);
        widget.set_widget_name(name);
        widget.set_width_request(self.width);

        // 0.4.3: Experimental: Allow for widgets enclosed into boxes.
        if !self.widgets.is_null() && experimental!() {
            build_child_widgets(self.widgets, left, centered, right, &widget)
        }

        ui::add_and_align(
            &widget,
            align,
            left,
            centered,
            right,
            if experimental!() { box_holder } else { None },
        );
        log!("Added a new box widget");
    }
}
