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
}

/// Widget alignment.
#[derive(EnumString)]
enum Align {
    LEFT,
    CENTERED,
    RIGHT,
}

/// Builds all of the widgets.
pub fn build_widgets(window: &gtk::ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let draw = create_box();
    let draw_centered = create_box();
    let draw_right = create_box();

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
            text: value["text"].to_string(),
            command: value["command"].to_string(),
        };

        // Create the widget structure.
        let mut widget_structure = GTKWidget {
            button: None,
            label: None,
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
            add_label(
                &draw,
                &draw_centered,
                &draw_right,
                widget_structure,
                e_alignment,
            )
        } else if identifier.contains("button") {
            widget_structure.create_button();
            add_button(
                &draw,
                &draw_centered,
                &draw_right,
                widget_structure,
                e_alignment,
            )
        }
    }
}

/// Adds a button.
fn add_button(
    draw: &Box,
    draw_centered: &Box,
    draw_right: &Box,
    gtk_widget_structure: GTKWidget,
    align: Align,
) {
    // The values and such is all set from `loop.rs`.
    let button = gtk_widget_structure
        .button
        .as_ref()
        .expect("[ERROR] Failed to access Button!\n");

    button.set_label(&gtk_widget_structure.properties.text);
    let c_command = gtk_widget_structure.properties.command.clone();
    if !c_command.is_empty() {
        button.connect_clicked(move |_| {
            proc::execute(c_command.to_string());
        });
    }

    debug_log("Adding button");
    match align {
        Align::LEFT => draw.add(button),
        Align::CENTERED => draw_centered.add(button),
        Align::RIGHT => draw_right.add(button),
    }

    unsafe {
        VEC.as_mut()
            .expect("[ERROR] Failed accessing VEC!\n")
            .push(gtk_widget_structure);
    }
}

/// Adds a label.
fn add_label(
    draw: &Box,
    draw_centered: &Box,
    draw_right: &Box,
    gtk_widget_structure: GTKWidget,
    align: Align,
) {
    // The values and such is all set from `loop.rs`.
    let label = gtk_widget_structure
        .label
        .as_ref()
        .expect("[ERROR] Failed to access Label!");
    debug_log("Adding label");
    match align {
        Align::LEFT => draw.add(label),
        Align::CENTERED => draw_centered.add(label),
        Align::RIGHT => draw_right.add(label),
    }

    unsafe {
        // If the command is empty, there is no need to add it to the VEC list.
        // Since it won't have to be redrawn.
        if !gtk_widget_structure.properties.command.is_empty() {
            VEC.as_mut()
                .expect("[ERROR] Failed accessing VEC!\n")
                .push(gtk_widget_structure);
        }
    }
}

/// Creates a standard Box widget with horizontal orientation.
fn create_box() -> Box {
    Box::new(gtk::Orientation::Horizontal, 0)
}
