#[macro_export]
/// Executes a bash command and outputs it to `result`.
macro_rules! execute {
    ($cmd:expr, $result:ident) => {
        if $cmd.is_empty() {
            drop(String::default());
        }

        let mut $result = String::from_utf8_lossy(
            &std::process::Command::new("bash")
                .args(["-c", $cmd])
                .output()
                .unwrap()
                .stdout,
        )
        .to_string();

        $result.pop();
        drop(&$result);
    };
}
