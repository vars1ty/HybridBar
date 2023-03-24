use crate::{ui, utils::aliases, widget::Align};
use glib::Cast;
use gtk::{
    traits::{LabelExt, WidgetExt},
    Label, Widget,
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
    ($widget_type:ty, $name:expr, $execute:expr) => {{
        let widgets = WIDGETS.lock().unwrap();
        for widget in widgets.iter() {
            if widget.name != $name {
                continue;
            }

            let widget = widget.widget.downcast_ref::<$widget_type>().unwrap();
            $execute(widget);
            break;
        }

        log!(format!("[WARN] Found no widget named '{}'", $name))
    }};
}

/// Adds a widget into `WIDGETS`.
macro_rules! add_widget {
    ($name:expr, $widget:expr) => {
        let mut widgets = WIDGETS.lock().unwrap();
        // Check so there's no widgets with the same name.
        for widget in widgets.iter() {
            // Ignore case only in this case.
            if widget.name.eq_ignore_ascii_case(&*$name) {
                panic!("[ERROR] [RUNE]: There's already a widget with the same name as '{}', please pick a different name!", $name);
            }
        }

        // No widgets with the same name, continue.
        widgets.push(GTKWidget {
            name: $name,
            widget: gtk::Widget::from($widget),
        });
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
struct GTKWidget {
    pub name: String,
    pub widget: Widget,
}

/// Rune VM.
pub struct RuneVM;

// "Hack"-make `Widget` thread-safe.
unsafe impl Send for GTKWidget {}
unsafe impl Sync for GTKWidget {}

impl Builder {
    /// Adds a new label widget.
    fn add_label(name: &str, content: &str, alignment: &str) {
        let label = Label::new(Some(content));
        label.set_widget_name(name);
        ui::add_and_align(&label, Align::from_str(alignment).unwrap(), None);
        label.show(); // Required otherwise it's inactive.
        add_widget!(name.to_owned(), label);
        log!(format!(
            "Adding a new label widget from loaded script, widget name: '{name}'"
        ));
    }

    /// Changes the text content of a `Label`.
    fn set_label_text(name: &str, content: &str) {
        using_widget!(Label, name, |label: &Label| { label.set_text(content) })
    }

    /// Changes the visibility of a `Label`.
    fn set_label_visible(name: &str, visible: bool) {
        using_widget!(Label, name, |label: &Label| { label.set_visible(visible) })
    }

    /// Checks whether or not the `Label` is visible.
    fn is_label_visible(name: &str) -> bool {
        let mut is_visible = false;
        using_widget!(Label, name, |label: &Label| {
            is_visible = label.is_visible()
        });
        is_visible
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
        module.function(["Builder", "set_label_text"], Builder::set_label_text)?;
        module.function(["Builder", "set_label_visible"], Builder::set_label_visible)?;
        module.function(["Builder", "is_label_visible"], Builder::is_label_visible)?;
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
