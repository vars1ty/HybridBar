use crate::{
    structures::Align,
    ui::{self, VEC},
    widget::HWidget, r#loop::get_update_rate,
};
use gtk::{traits::*, *};
use std::{fmt::Display, process::Stdio, sync::RwLock, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

lazy_static! {
    /// Current text buffer from `stdout`.
    static ref BUFFER: RwLock<String> = RwLock::new(String::default());
}

/// Creates a new label widget.
#[derive(Debug)]
pub struct LabelWidget {
    pub tooltip: String,
    pub text: String,
    pub command: String,
    pub label: Label,
    pub listen: bool,
}

// For VEC to work.
unsafe impl Send for LabelWidget {}
unsafe impl Sync for LabelWidget {}

/// 0.3.2: If `listen` is `true`, call this function and then externally set the label text-value
///   to that of `BUFFER`.
fn begin_listen(cmd: String) {
    task::spawn(async move {
        let mut child = Command::new("bash")
            .args(["-c", &cmd])
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap_or_else(|_| panic!("[ERROR] Cannot start '{cmd}'\n"));

        let out = child
            .stdout
            .take()
            .expect("[ERROR] Cannot take stdout from child!\n");

        let mut reader = BufReader::new(out).lines();
        let update_rate = get_update_rate();
        loop {
            *BUFFER.write().unwrap() = reader
                .next_line()
                .await
                .expect("[ERROR] There are no more lines available!\n")
                .expect("[ERROR] The string value is None!\n");

            tokio::time::sleep(Duration::from_millis(update_rate)).await;
        }
    });
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for LabelWidget {
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box) {
        let is_static = self.command.is_empty();
        self.label.set_widget_name(&name);
        // 0.2.7: Support for tooltips
        self.label.set_tooltip_markup(Some(&self.tooltip));
        ui::add_and_align(&self.label, align, left, centered, right);

        if self.listen {
            begin_listen(self.command.clone());
        }

        // 0.3.2: Don't add widgets that don't have a command set to the vector, as it won't be
        //   updated due to being static.
        if !is_static {
            VEC.lock()
                .expect("[ERROR] Cannot access ui::VEC!\n")
                .push(self)
                .expect("[ERROR] You cannot have more than `1024` Labels!\n");
        } else {
            self.label.set_markup(&self.text);
        }

        log!(format!(
            "Added a new label widget named '{name}', is static: {}",
            is_static
        ));
    }

    fn update_label_reg(&self, new_content: &(impl Display + Clone)) {
        let ts = new_content.to_string();
        if self.label.text().eq(&ts) {
            // Exact same content, return and don't cause a redraw.
            return;
        }

        log!(format!(
            "[{}] -> Label update received (from => \"{}\", to => \"{}\")",
            self.label.widget_name(),
            self.label.text(),
            ts
        ));

        // 0.2.7: Support for markup as long as the command is empty.
        // It doesn't support markup with commands because some strings may cause GTK to mistreat
        // it, which I may fix in the future.
        if self.command.is_empty() {
            self.label.set_markup(&ts);
        } else {
            self.label.set_text(&ts);
        }
    }

    fn update_label_internal(&self) {
        let new_content = BUFFER
            .read()
            .expect("[ERROR] Failed retrieving content from BUFFER!\n");
        let old_content = self.label.text();
        // eq-check the new content for old_content. Doing the opposite requires a .to_string()
        // call.
        if !new_content.eq(&old_content) {
            // Not the same; set content and redraw.
            self.label.set_text(&new_content);
        }
    }
}
