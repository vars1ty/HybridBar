use crate::utils::{
    hyprland::HyprlandData,
    rune::{builder_module::Builder, hybrid_module::HybridModule},
};
use rune::{
    termcolor::{ColorChoice, StandardStream},
    Context, ContextError, Diagnostics, Module, Result, Source, Sources, Vm,
};
use std::sync::Arc;

/*
 * This contains unfinished code, use at your own risk!
*/

/// Rune VM.
pub struct RuneVM;

impl RuneVM {
    /// Installs custom functions which can be used by the user.
    #[allow(dead_code)]
    pub fn hybrid_mod() -> Result<Module, ContextError> {
        let mut module = Module::new();

        // Base core functions.
        module.function(["Hybrid", "execute"], HybridModule::execute)?;
        module.function(["Hybrid", "log"], HybridModule::log)?;
        module.function(["Hybrid", "use_aliases"], HybridModule::use_aliases)?;

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
        module.function(["Builder", "set_tooltip"], Builder::set_tooltip)?;
        module.function(["Builder", "set_opacity"], Builder::set_opacity)?;

        // Feature-related functions.
        module.function(
            ["Hyprland", "get_current_workspace"],
            HyprlandData::get_current_workspace,
        )?;
        module.function(
            ["Hyprland", "get_current_window"],
            HyprlandData::get_current_window,
        )?;
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
}
