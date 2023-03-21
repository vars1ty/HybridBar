use crate::{ui, utils::aliases, widget::Align};
use gtk::{traits::WidgetExt, Label};
use rune::{
    termcolor::{ColorChoice, StandardStream},
    Context, ContextError, Diagnostics, Module, Result, Source, Sources, Vm,
};
use std::sync::Arc;

/*
 * This contains unfinished code, use at your own risk!
 */

pub struct RuneVM;

impl RuneVM {
    /// Installs custom functions which can be used by the user.
    #[allow(dead_code)]
    pub fn hybrid_mod() -> Result<Module, ContextError> {
        let mut module = Module::new();
        module.function(["execute"], Self::execute)?;
        module.function(["log"], Self::log)?;
        module.function(["is_feature_active"], Self::is_feature_active)?;
        module.function(["use_aliases"], Self::use_aliases)?;
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
        log!(msg)
    }

    /// Checks if the specified feature is active.
    fn is_feature_active(tag: &str) -> bool {
        is_feature_active!(tag)
    }

    /// Checks for aliases in `content`, then replaces them with their real values.
    fn use_aliases(content: &str) -> String {
        aliases::use_aliases(content)
    }

    /// Adds a new label widget.
    fn add_label(name: &str, content: &str, alignment: &str) {
        let label = Label::new(Some(content));
        label.set_widget_name(name);
        ui::add_and_align(&label, Align::from_str(alignment).unwrap(), None);
        label.show(); // Required otherwise it's inactive.
        log!(&format!(
            "Adding a new label widget from loaded script, widget name: '{name}'"
        ));
    }
}
