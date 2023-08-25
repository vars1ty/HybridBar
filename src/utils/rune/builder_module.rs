use crate::{widget::Align, UI};
use gtk::{traits::*, Button, Label, Widget};
use gtk::prelude::Cast;
use rune::Any;
use smallvec::SmallVec;
use std::sync::Mutex;

/// Looks up a runtime-scripted widget, then passes it to `execute` for you to use as a reference.
/// Panics if no widget with the specified name was found.
macro_rules! using_widget {
    // Single widget-type variant.
    ($widget_type:ty, $name:expr, $execute:expr) => {{
        let widgets = WIDGETS.lock().unwrap();
        for widget in widgets.iter() {
            let widget = &widget.0;
            if !widget.widget_name().eq_ignore_ascii_case($name) {
                continue;
            }

            let widget = widget.downcast_ref::<$widget_type>().unwrap();
            $execute(widget);
            return;
        }

        // No widgets were found with the specified name, panic.
        panic!(
            "[ERROR] [RUNE]: Found no widgets named '{}', please check if the name is correct!",
            $name
        );
    }};

    // Any type variant
    ($name:expr, $execute:expr) => {
        let widgets = WIDGETS.lock().unwrap();
        let widgets = widgets.iter();
        let mut found_any = false;
        for widget in widgets {
            let widget = &widget.0;
            if widget.widget_name().eq_ignore_ascii_case($name) {
                found_any = true;
                $execute(widget);
            }
        }

        // No widgets were found with the specified name, panic.
        if !found_any {
            panic!(
                "[ERROR] [RUNE]: Found no widgets named '{}', please check if the name is correct!",
                $name
            );
        }
    };
}

/// Adds a widget into `WIDGETS`.
/// Panics if there's already a widget present with the same name.
macro_rules! add_widget {
    ($name:expr, $widget:expr) => {
        let mut widgets = WIDGETS.lock().unwrap();
        // Check so there's no widgets with the same name.
        for widget in widgets.iter() {
            // Ignore case only in this case.
            if widget.0.widget_name().eq_ignore_ascii_case(&*$name) {
                panic!("[ERROR] [RUNE]: There's already a widget with the same name as '{}', please pick a different name!", $name);
            }
        }

        // No widgets with the same name, continue.
        widgets.push(GTKWidget(gtk::Widget::from($widget)));
    };
}

lazy_static! {
    static ref WIDGETS: Mutex<SmallVec<[GTKWidget; 6]>> = Mutex::new(SmallVec::new());
}

/// Wrapper around `Widget`.
struct GTKWidget(Widget);

// "Hack"-make `GTKWidget` thread-safe so that `Widget` can be used across "threads".
unsafe impl Send for GTKWidget {}

/// The Hybrid module, providing functionalities such as `Builder::rename_widget(name, new_name)`.
#[derive(Any)]
pub struct Builder;

impl Builder {
    /// Adds a new label widget.
    pub fn add_label(ui: &UI, name: &str, content: &str, alignment: &str) {
        let label = Label::new(Some(content));
        label.set_widget_name(name);
        ui.add_and_align(&label, Align::from_str(alignment).unwrap(), None);
        label.show();
        add_widget!(name.to_owned(), label);
        log!(format!(
            "Adding a new label widget from loaded script, widget name: '{name}'"
        ));
    }

    /// Adds a new button widget.
    pub fn add_button(ui: &UI, name: &str, content: &str, alignment: &str) {
        let button = Button::with_label(content);
        button.set_widget_name(name);
        ui.add_and_align(&button, Align::from_str(alignment).unwrap(), None);
        button.show();
        add_widget!(name.to_owned(), button);
        log!(format!(
            "Adding a new button widget from loaded script, widget name: '{name}'"
        ));
    }

    /// Changes the text content of a `Label`.
    pub fn set_label_text(name: &str, content: &str) {
        using_widget!(Label, name, |label: &Label| label.set_text(content))
    }

    /// Changes the text content of a `Button`.
    pub fn set_button_text(name: &str, content: &str) {
        using_widget!(Button, name, |button: &Button| button.set_label(content))
    }

    /// Changes the shell-command of a `Button`.
    pub fn set_button_command(name: &str, command: &'static str) {
        using_widget!(Button, name, |button: &Button| button.connect_clicked(
            |_| {
                execute!(command);
            }
        ))
    }

    /// Changes the visibility of a `Widget` with the specified name.
    pub fn set_visible(name: &str, visible: bool) {
        using_widget!(name, |widget: &Widget| widget.set_visible(visible));
    }

    /// Checks whether or not the `Widget` with the specified name is visible.
    pub fn is_visible(name: &str) -> bool {
        let mut is_visible = false;
        using_widget!(name, |widget: &Widget| is_visible = widget.is_visible());
        is_visible
    }

    /// Renames a `Widget` to a new name.
    pub fn rename_widget(name: &str, new_name: &str) {
        using_widget!(name, |widget: &Widget| {
            widget.set_widget_name(new_name);
            log!(format!("[RUNE]: Widget '{name}' renamed to '{new_name}'!"));
        });
    }

    /// Changes the tooltip content of a `Widget`.
    pub fn set_tooltip(name: &str, content: &str, markup: bool) {
        let content = if content.is_empty() {
            None
        } else {
            Some(content)
        };
        using_widget!(name, |widget: &Widget| if markup {
            widget.set_tooltip_markup(content);
        } else {
            widget.set_tooltip_text(content)
        });
    }

    /// Changes the opacity of a `Widget`.
    pub fn set_opacity(name: &str, opacity: f64) {
        using_widget!(name, |widget: &Widget| widget.set_opacity(opacity));
    }
}
