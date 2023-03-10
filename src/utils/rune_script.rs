use crate::utils::aliases;
use rune::{
    termcolor::{ColorChoice, StandardStream},
    Context, ContextError, Diagnostics, Module, Result, Source, Sources, Vm,
};
use std::sync::Arc;

/*
 * This contains unfinished code, use at your own risk!
 */

/// Installs custom functions which can be used by the user.
#[allow(dead_code)]
pub fn hybrid_mod(name: &str) -> Result<Module, ContextError> {
    let mut module = Module::new();
    module.function(&["execute"], execute)?;
    module.function(&["log"], log)?;
    module.function(&["is_feature_active"], is_feature_active)?;
    module.function(&["use_aliases"], use_aliases)?;
    module.constant(["SCRIPT_NAME"], name)?;
    Ok(module)
}

/// Creates a new Virtual Machine for the given source.
#[allow(dead_code)]
pub fn create_vm(name: &str, source: &str) -> rune::Result<Vm> {
    let m = hybrid_mod(name)?;
    let mut context = Context::with_default_modules()?;
    context.install(&m)?;
    let runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    sources.insert(Source::new(name, source));

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
