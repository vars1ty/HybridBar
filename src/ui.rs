use crate::{debug::log, r#loop::update, widget_builder::RenderBoxes, *};
use gtk::{traits::*, *};
use std::str::FromStr;

use strum_macros::EnumString;

/// Static mutable HashMap, because I'm not dealing with lifetime bullshit one more fucking minute.
pub static mut VEC: Option<Vec<GTKWidget>> = None;

/// Key separator.
const ALIGNMENT: char = '-';

/// Identifier separator.
const SEPARATOR: char = '_';

// TODO: Possibly rework the structures.

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
    fn create_button(&mut self, name: &str) {
        let button = Button::new();
        button.set_widget_name(name);
        self.button = Some(button);
    }

    /// Creates a label.
    fn create_label(&mut self, name: &str) {
        let label = Label::new(None);
        label.set_widget_name(name);
        self.label = Some(label);
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

        // Gets the amount of entires in the split key.
        let count = key.split(SEPARATOR).count();

        // Gets the widget identifier.
        // For example: `left-label_ABC` <= `left-label` is the IDENTIFIER, `ABC` is the NAME.
        let identifier = key
            .split(SEPARATOR)
            .nth(0)
            .expect("[ERROR] Failed retrieving widget identifier!\n");

        // Grabs the widget alignment.
        let widget_alignment = key
            .split(ALIGNMENT)
            .nth(0)
            .expect("[ERROR] Failed retrieving widget alignment!\n")
            .to_uppercase();

        // Stores the unique widget name temporarily.
        let mut widget_name = String::default();
        for i in 1..count {
            widget_name.push_str(key.split(SEPARATOR).nth(i).unwrap());
            // Only add '_' to the end if we haven't reached the end.
            if i != count - 1 {
                widget_name.push(SEPARATOR);
            }
        }

        // Create the properties structure.
        let widget_properties = WidgetProperties {
            text: config::try_get::<String>(key, "text").unwrap().0,
            command: config::try_get::<String>(key, "command").unwrap().0,
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
        let e_alignment = Align::from_str(&widget_alignment)
            .expect(format!("[ERROR] There is no '{identifier}' identifier!\n").as_str());

        // Debug messages.
        log(format!(
            "Adding widget '{identifier}' with alignment '{widget_alignment}'"
        ));

        // Check for identifiers.
        // Defo. not clean or pretty, will probably fix it later.
        if identifier.contains("label") {
            widget_structure.create_label(&widget_name);
            widget_builder::add_label(render_boxes, widget_structure, e_alignment)
        } else if identifier.contains("button") {
            widget_structure.create_button(&widget_name);
            widget_builder::add_button(render_boxes, widget_structure, e_alignment)
        } else if identifier.contains("spacing") {
            widget_structure.create_spacing(
                config::try_get::<i32>(key, "spacing_start").unwrap().1,
                config::try_get::<i32>(key, "spacing_end").unwrap().1,
            );
            widget_builder::add_box(render_boxes, widget_structure, e_alignment)
        }
    }
}
