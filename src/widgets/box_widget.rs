use crate::{
    config::Config,
    constants::ERR_EMPTY_NAME,
    structures::BaseKeys,
    widget::{Align, HWidget},
    UI,
};
use gtk::{traits::*, *};
use json::JsonValue;
use smallvec::SmallVec;

/// Creates a new basic box widget.
pub struct BoxWidget {
    pub width: i32,
    pub widgets: JsonValue,
}

/// Builds the child widgets.
fn build_child_widgets(ui: &UI, widgets: JsonValue, box_holder: &Box, config: &'static Config) {
    const SEPARATOR: &str = "_";
    let relevant = widgets.entries().filter(|(key, _)| key.contains(SEPARATOR));

    for (key, json) in relevant {
        // Gets the widget identifiers.
        let identifiers: SmallVec<[&str; 4]> = key.split(SEPARATOR).collect();

        // Type example: `label_ABC` <= `label` is the IDENTIFIER, `ABC` is the NAME.
        let widget_type = identifiers[0];

        // Base keys.
        let (text, command, update_rate, tooltip, tooltip_command) = ui.get_base_keys(json, config);
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
        ui.add_widget(
            json,
            (widget_type, &widget_name),
            base_keys,
            widget_type,
            Some(box_holder),
            config,
        )
    }
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for BoxWidget {
    fn add(self, ui: &UI, name: &str, align: Align, box_holder: Option<&Box>) {
        let widget = Box::new(Orientation::Horizontal, 0);
        widget.set_widget_name(name);
        widget.set_width_request(self.width);

        // 0.4.3: Experimental: Allow for widgets enclosed into boxes.
        // 0.4.7: Stabilize Box Child-Widgets.
        if !self.widgets.is_null() {
            build_child_widgets(ui, self.widgets, &widget, ui.get_config())
        }

        ui.add_and_align(&widget, align, box_holder);
        log!("Added a new box widget");
    }
}
