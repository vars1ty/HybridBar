use std::str::FromStr;

use crate::{debug::debug_log, r#loop::update, *};
use gtk::{traits::*, *};

use strum_macros::EnumString;

/// Static mutable HashMap, because I'm not dealing with lifetime bullshit one more fucking minute.
pub static mut VEC: Option<Vec<GTKWidget>> = None;

/// Alignment separator.
const ALIGNMENT: char = '-';

/// Identifier separator.
const SEPARATOR: char = '_';

/// GTK Widget structure.
pub struct GTKWidget {
    pub button: Option<Button>,
    pub label: Option<Label>,
    pub spacing: Option<Box>,
    pub properties: WidgetProperties,
}

/// Widget properties structure.
pub struct WidgetProperties {
    pub text: String,
    pub command: String,
}

/// Easy mutable implementions for GTKWidget.
impl GTKWidget {
    /// Creates a button.
    fn create_button(&mut self) {
        self.button = Some(Button::new());
    }

    /// Creates a label.
    fn create_label(&mut self) {
        self.label = Some(Label::new(None));
    }

    /// Creates a box.
    fn create_spacing(&mut self, spacing_start: i32, spacing_end: i32) {
        let w_box = widget_builder::create_box();
        w_box.set_margin_start(spacing_start);
        w_box.set_margin_end(spacing_end);
        self.spacing = Some(w_box);
    }
}

/// Widget alignment.
#[derive(EnumString)]
pub enum Align {
    LEFT,
    CENTERED,
    RIGHT,
}

/// Builds all of the widgets.
pub fn build_widgets(window: &gtk::ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let draw = widget_builder::create_box();
    let draw_centered = widget_builder::create_box();
    let draw_right = widget_builder::create_box();

    // Add and align all of the box widgets.
    draw.set_center_widget(Some(&draw_centered));
    draw.pack_end(&draw_right, false, true, 0);
    window.add(&draw);

    // Create the Vector.
    unsafe { VEC = Some(Vec::new()) }

    // Prepare all of the widgets.
    create_components(&draw, &draw_right, &draw_centered);
    // Make every widget visible.
    window.show_all();
    // Update dynamic content.
    update();
}

/// Creates all of the widgets.
fn create_components(draw: &Box, draw_right: &Box, draw_centered: &Box) {
    // Add all of the widgets defined from the config.
    for (key, value) in config::read_config().entries() {
        if !key.contains(ALIGNMENT) || !key.contains(SEPARATOR) {
            continue;
        }

        // Identifiers are the first part of the JSON Key.
        // For example: label_hello <= "label" being the identifier, "hello" being the name.
        let identifier = key
            .split(SEPARATOR)
            .nth(0)
            .expect("[ERROR] Failed splitting key!\n");

        // Get the alignment of the widget.
        // For example: left-label_abc <= "left" being the alignment.
        let alignment = key
            .split('-')
            .nth(0)
            .expect("[ERROR] Failed splitting alignment!\n")
            .to_uppercase();

        // Create the properties structure.
        let widget_properties = WidgetProperties {
            text: config::try_get_string(key, "text"),
            command: config::try_get_string(key, "command"),
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
        let e_alignment = Align::from_str(&alignment)
            .expect(format!("[ERROR] There is no '{identifier}' identifier!\n").as_str());

        // Debug messages.
        debug_log(format!(
            "Adding widget '{identifier}' with alignment '{alignment}'"
        ));

        // Check for identifiers.
        // Defo. not clean or pretty, will probably fix it later.
        if identifier.contains("label") {
            widget_structure.create_label();
            widget_builder::add_label(
                &draw,
                &draw_centered,
                &draw_right,
                widget_structure,
                e_alignment,
            )
        } else if identifier.contains("button") {
            widget_structure.create_button();
            widget_builder::add_button(
                &draw,
                &draw_centered,
                &draw_right,
                widget_structure,
                e_alignment,
            )
        } else if identifier.contains("spacing") {
            widget_structure.create_spacing(
                config::try_get_i32(key, "spacing_start"),
                config::try_get_i32(key, "spacing_end"),
            );
            widget_builder::add_box(
                &draw,
                &draw_centered,
                &draw_right,
                widget_structure,
                e_alignment,
            )
        }
    }
}
