use cmd_lib::run_fun;

/// Executes a command and returns the output.
pub fn execute_string(cmd: String) -> String {
    run_fun!(/usr/bin/bash -c "$cmd").expect("[ERROR] Failed executing bash command!\n")
}

/// Eecutes a command and returns the output.
pub fn execute_str(cmd: &str) -> &str {
    &run_fun!(/usr/bin/bash -c "$cmd").expect("[ERROR] Failed executing bash command!\n")
}
