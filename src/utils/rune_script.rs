use crate::{ui, utils::aliases, widget::Align};
use glib::Cast;
use gtk::{
    traits::{ButtonExt, LabelExt, WidgetExt},
    Button, Label, Widget,
};
use rune::{
    termcolor::{ColorChoice, StandardStream},
    Any, Context, ContextError, Diagnostics, Module, Result, Source, Sources, Vm,
};
use smallvec::SmallVec;
use std::sync::{Arc, Mutex};

/*
 * This contains unfinished code, use at your own risk!
*/

/// Looks up a runtime-scripted widget, then passes it to `execute` for you to use as a reference.
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

/// Widget Builder which hold certain exposed functions for the user, alongside some internal
/// tools.
#[derive(Any)]
struct Builder;

/// Wrapper around `Widget`.
struct GTKWidget(Widget);

/// Rune VM.
pub struct RuneVM;

// "Hack"-make `GTKWidget` thread-safe so that `Widget` can be used across "threads".
unsafe impl Send for GTKWidget {}

impl Builder {
    /// Adds a new label widget.
    fn add_label(name: &str, content: &str, alignment: &str) {
        let label = Label::new(Some(content));
        label.set_widget_name(name);
        ui::add_and_align(&label, Align::from_str(alignment).unwrap(), None);
        label.show();
        add_widget!(name.to_owned(), label);
        log!(format!(
            "Adding a new label widget from loaded script, widget name: '{name}'"
        ));
    }

    /// Adds a new button widget.
    fn add_button(name: &str, content: &str, alignment: &str) {
        let button = Button::with_label(content);
        button.set_widget_name(name);
        ui::add_and_align(&button, Align::from_str(alignment).unwrap(), None);
        button.show();
        add_widget!(name.to_owned(), button);
        log!(format!(
            "Adding a new button widget from loaded script, widget name: '{name}'"
        ));
    }

    /// Changes the text content of a `Label`.
    fn set_label_text(name: &str, content: &str) {
        using_widget!(Label, name, |label: &Label| label.set_text(content))
    }

    /// Changes the text content of a `Button`.
    fn set_button_text(name: &str, content: &str) {
        using_widget!(Button, name, |button: &Button| button.set_label(content))
    }

    /// Changes the shell-command of a `Button`.
    fn set_button_command(name: &str, command: &'static str) {
        using_widget!(Button, name, |button: &Button| button.connect_clicked(
            |_| {
                execute!(command);
            }
        ))
    }

    /// Changes the visibility of a `Widget` with the specified name.
    /// Panics if no widget with the specified name was found.
    fn set_visible(name: &str, visible: bool) {
        using_widget!(name, |widget: &Widget| widget.set_visible(visible));
    }

    /// Checks whether or not the `Widget` with the specified name is visible.
    /// Panics if no widget with the specified name was found.
    fn is_visible(name: &str) -> bool {
        let mut is_visible = false;
        using_widget!(name, |widget: &Widget| is_visible = widget.is_visible());
        is_visible
    }

    /// Renames a `Widget` to a new name.
    /// Panics if no widget with the specified name was found.
    fn rename_widget(name: &str, new_name: &str) {
        using_widget!(name, |widget: &Widget| {
            widget.set_widget_name(new_name);
            log!(format!("[RUNE]: Widget '{name}' renamed to '{new_name}'!"));
        });
    }
}

impl RuneVM {
    /// Installs custom functions which can be used by the user.
    #[allow(dead_code)]
    pub fn hybrid_mod() -> Result<Module, ContextError> {
        let mut module = Module::new();

        // Base core functions.
        module.function(["execute"], Self::execute)?;
        module.function(["log"], Self::log)?;
        module.function(["is_feature_active"], Self::is_feature_active)?;
        module.function(["use_aliases"], Self::use_aliases)?;

        // Widget-related functions.
        module.function(["Builder", "add_label"], Builder::add_label)?;
        module.function(["Builder", "add_button"], Builder::add_button)?;
        module.function(["Builder", "set_label_text"], Builder::set_label_text)?;
        module.function(["Builder", "set_button_text"], Builder::set_button_text)?;
        module.function(
            ["Builder", "set_button_command"],
            Builder::set_button_command,
        )?;
        module.function(["Builder", "set_visible"], Builder::set_visible)?;
        module.function(["Builder", "is_visible"], Builder::is_visible)?;
        module.function(["Builder", "rename_widget"], Builder::rename_widget)?;
        Ok(module)
    }

    /// Creates a new Virtual Machine for the given source.
    #[allow(dead_code)]
    pub fn create_vm(source: &str) -> rune::Result<Vm> {
        let m = Self::hybrid_mod()?;
        let mut context = Context::with_default_modules()?;
        context.install(&m)?;
        let runtime = Arc::new(context.runtime());

        let mut sources = Sources::new();
        sources.insert(Source::new("main", source));

        let mut diagnostics = Diagnostics::new();

        let result = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diagnostics.emit(&mut writer, &sources)?;
        }

        let unit = result?;
        Ok(Vm::new(runtime, Arc::new(unit)))
    }

    /// Executes a shell-command.
    fn execute(cmd: &str) -> String {
        execute!(cmd)
    }

    /// Prints a Hybrid verbose message.
    fn log(msg: &str) {
        log!(format!("[RUNE]: {msg}"))
    }

    /// Checks if the specified feature is active.
    fn is_feature_active(tag: &str) -> bool {
        is_feature_active!(tag)
    }

    /// Checks for aliases in `content`, then replaces them with their real values.
    fn use_aliases(content: &str) -> String {
        aliases::use_aliases(content)
    }
}
