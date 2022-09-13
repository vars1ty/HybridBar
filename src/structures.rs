use gtk::{traits::WidgetExt, *};
use strum::EnumString;

use crate::widget_builder;

// This file holds all the structures we'll be using.
/// GTK Widget structure.
pub struct GTKWidget {
    pub button: Option<Button>,
    pub label: Option<Label>,
    pub spacing: Option<Box>,
    pub properties: WidgetProperties,
}

/// Easy mutable implementions for GTKWidget.
impl GTKWidget {
    /// Creates a button.
    pub fn create_button(&mut self, name: &str) {
        let button = Button::new();
        button.set_widget_name(name);
        self.button = Some(button);
    }

    /// Creates a label.
    pub fn create_label(&mut self, name: &str) {
        let label = Label::new(None);
        label.set_widget_name(name);
        self.label = Some(label);
    }

    /// Creates a box.
    pub fn create_spacing(&mut self, spacing_start: i32, spacing_end: i32) {
        let w_box = widget_builder::create_box();
        w_box.set_margin_start(spacing_start);
        w_box.set_margin_end(spacing_end);
        self.spacing = Some(w_box);
    }
}

/// Widget properties structure.
pub struct WidgetProperties {
    pub text: String,
    pub command: String,
}

/// Widget alignment.
#[derive(EnumString)]
pub enum Align {
    LEFT,
    CENTERED,
    RIGHT,
}
