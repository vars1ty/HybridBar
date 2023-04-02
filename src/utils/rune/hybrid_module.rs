use crate::utils::aliases;

/// The Hybrid module, providing functionalities such as `Hybrid::execute(cmd)`.
pub struct HybridModule;

impl HybridModule {
    /// Executes a shell-command.
    pub fn execute(cmd: &str) -> String {
        execute!(cmd)
    }

    /// Prints a Hybrid verbose message.
    pub fn log(msg: &str) {
        log!(format!("[RUNE]: {msg}"))
    }

    /// Checks if the specified feature is active.
    pub fn is_feature_active(tag: &str) -> bool {
        is_feature_active!(tag)
    }

    /// Checks for aliases in `content`, then replaces them with their real values.
    pub fn use_aliases(content: &str) -> String {
        aliases::use_aliases(content)
    }
}
