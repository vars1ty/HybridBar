use std::process::Command;

const DEFAULT_PROC: &str = "bash";
const DEFAULT_ARG: &str = "-c";

/// Executes a command and returns the output.
pub fn execute(cmd: String) -> String {
    let output = Command::new(DEFAULT_PROC)
        .args([DEFAULT_ARG, &cmd])
        .output()
        .expect(format!("Failed to execute process \"{cmd}\"!\n").as_str());
    let mut result = String::from_utf8_lossy(&output.stdout).to_string();

    // Removes the last character since its an empty line ('\n').
    result.pop();
    result
}
