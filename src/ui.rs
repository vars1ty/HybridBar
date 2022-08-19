use std::str::FromStr;

use crate::{debug::debug_log, r#loop::update, widget_builder::RenderBoxes, *};
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
    let left = widget_builder::create_box();
    let centered = widget_builder::create_box();
    let right = widget_builder::create_box();

    // Add and align all of the box widgets.
    left.set_center_widget(Some(&centered));
    left.pack_end(&right, false, true, 0);
    window.add(&left);

    // Create the Vector.
    unsafe { VEC = Some(Vec::new()) }

    // Create the Render Boxes structure.
    let render_boxes = RenderBoxes {
        draw_left: left,
        draw_centered: centered,
        draw_right: right,
    };

    // Prepare all of the widgets.
    create_components(&render_boxes);
    // Make every widget visible.
    window.show_all();
    // Update dynamic content.
    update();
}

/// Creates all of the widgets.
fn create_components(render_boxes: &RenderBoxes) {
    // Add all of the widgets defined from the config.
    for (key, _) in config::read_config().entries() {
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
            widget_builder::add_label(render_boxes, widget_structure, e_alignment)
        } else if identifier.contains("button") {
            widget_structure.create_button();
            widget_builder::add_button(render_boxes, widget_structure, e_alignment)
        } else if identifier.contains("spacing") {
            widget_structure.create_spacing(
                config::try_get_i32(key, "spacing_start"),
                config::try_get_i32(key, "spacing_end"),
            );
            widget_builder::add_box(render_boxes, widget_structure, e_alignment)
        }
    }
}
