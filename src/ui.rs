use crate::config;
use crate::proc;
use crate::r#loop::*;
use gtk::{traits::*, *};

/// Static mutable HashMap, because I'm not dealing with lifetime bullshit one more fucking minute.
pub static mut VEC: Option<Vec<GTKWidget>> = None;

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
enum Align {
    LEFT,
    CENTER,
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
        if !key.contains(SEPARATOR) {
            continue;
        }

        // Identifiers are the first part of the JSON Key, for example:
        // label_hello <= "label" being the identifier, "hello" being the name.
        let identifier = key
            .split(SEPARATOR)
            .nth(0)
            .expect("[ERROR] Failed splitting key!\n");

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

        // Check for identifiers.
        match identifier {
            "label" => {
                widget_structure.create_label();
                add_label(&draw, &draw_centered, &draw_right, widget_structure, Align::LEFT)
            }
            "centered-label" => {
                widget_structure.create_label();
                add_label(&draw, &draw_centered, &draw_right, widget_structure, Align::CENTER);
            }
            "right-label" => {
                widget_structure.create_label();
                add_label(&draw, &draw_centered, &draw_right, widget_structure, Align::RIGHT)
            }
            "button" => {
                widget_structure.create_button();
                add_button(&draw, &draw_centered, &draw_right, widget_structure, Align::LEFT);
                
            }
            "centered-button" => {
                widget_structure.create_button();
                add_button(&draw, &draw_centered, &draw_right, widget_structure, Align::CENTER);
                
            }
            "right-button" => {
                widget_structure.create_button();
                add_button(&draw, &draw_centered, &draw_right, widget_structure, Align::RIGHT);
                
            }
            _ => panic!(
                "[ERROR] Invalid identifier! You can only use [ label / centered-label / right-label / button / centered-button / right-button ]\n"
            ),
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
        .expect("[ERROR] Failed to access Button!");

    button.set_label(&gtk_widget_structure.properties.text);
    let c_command = gtk_widget_structure.properties.command.clone();
    if !c_command.is_empty() {
    button.connect_clicked(move |_| {
        proc::execute(c_command.to_string());
    });
    }

    match align {
        Align::LEFT => draw.add(button),
        Align::CENTER => draw_centered.add(button),
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
    match align {
        Align::LEFT => draw.add(label),
        Align::CENTER => draw_centered.add(label),
        Align::RIGHT => draw_right.add(label),
    }

    unsafe {
        VEC.as_mut()
            .expect("[ERROR] Failed accessing VEC!\n")
            .push(gtk_widget_structure);
    }
}

/// Creates a standard Box widget with horizontal orientation.
fn create_box() -> Box {
    Box::new(gtk::Orientation::Horizontal, 0)
}
