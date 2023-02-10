use crate::{
    aliases::use_aliases,
    config,
    constants::{ERR_NO_LINES, ERR_STRING_NONE, ERR_TAKE_STDOUT, PROC_TARGET},
    structures::Align,
    ui,
    widget::HWidget,
};
use gtk::{glib::GString, traits::*, *};
use std::{mem::take, process::Stdio, sync::Mutex, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

lazy_static! {
    /// Current text buffer from `stdout`.
    static ref BUFFER: Mutex<String> = Mutex::new(String::default());
}

/// Creates a new label widget.
#[derive(Debug)]
pub struct LabelWidget {
    pub tooltip: String,
    pub tooltip_command: String,
    pub text: String,
    pub command: String,
    pub update_rate: u64,
    pub label: Label,
    pub listen: bool,
}

/// 0.3.2: If `listen` is `true`, call this function and then set the label text-value
///   to that of `BUFFER`.
fn begin_listen(cmd: String) {
    task::spawn(async move {
        let mut child = Command::new(PROC_TARGET)
            .args(["-c", &cmd])
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap_or_else(|_| panic!("[ERROR] Cannot start '{cmd}'!"));

        let out = child.stdout.take().expect(ERR_TAKE_STDOUT);

        let mut reader = BufReader::new(out).lines();
        let update_rate = config::get_update_rate();
        loop {
            *BUFFER.lock().unwrap() = reader
                .next_line()
                .await
                .expect(ERR_NO_LINES)
                .expect(ERR_STRING_NONE);

            tokio::time::sleep(Duration::from_millis(update_rate)).await;
        }
    });
}

/// Starts updating the dynamic tooltip, if any.
fn start_tooltip_loop(label_ref: &mut LabelWidget) {
    if label_ref.tooltip_command.is_empty() {
        // Not eligible, cancel.
        return;
    }

    let label = label_ref.label.clone();
    let tooltip = take(&mut label_ref.tooltip);
    let tooltip_command = take(&mut label_ref.tooltip_command);
    let tick = move || {
        let mut new_tooltip = String::default();
        new_tooltip.push_str(&tooltip);
        new_tooltip.push_str(&use_aliases(&tooltip_command));

        let tooltip_markup = label.tooltip_markup().unwrap_or(GString::from(""));
        if !tooltip_markup.eq(&new_tooltip) {
            // Markup support here, the user therefore has to deal with any upcoming issues due to
            // the command output, on their own.
            label.set_tooltip_markup(Some(&new_tooltip));
        }

        glib::Continue(true)
    };

    tick();
    glib::timeout_add_local(Duration::from_millis(1000), tick);
}

/// Starts updating the dynamic label content.
fn start_label_loop(label: Label, text: String, command: String, update_rate: u64, listen: bool) {
    if command.is_empty() || update_rate == 0 {
        // Not eligible, cancel.
        return;
    }

    let tick = move || {
        if !listen {
            let mut new_text = String::default();
            new_text.push_str(&text);
            new_text.push_str(&use_aliases(&command));

            if !label.text().eq(&new_text) {
                // Not the same as new_text; redraw.
                label.set_text(&new_text);
            }
        } else {
            update_from_buffer(&label);
        }

        glib::Continue(true)
    };

    tick();
    glib::timeout_add_local(Duration::from_millis(update_rate), tick);
}

/// Updates the labels content with the string from `BUFFER`.
fn update_from_buffer(label: &Label) {
    if let Ok(new_content) = BUFFER.lock() {
        let old_content = label.text();
        // eq-check the new content for old_content. Doing the opposite requires a .to_string()
        // call.
        if !new_content.eq(&old_content) {
            // Not the same; set content and redraw.
            label.set_text(&new_content);
        }
    } else {
        log!(format!(
            "[WARN] Failed retrieving content from BUFFER for label '{}'!",
            label.widget_name()
        ))
    }
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for LabelWidget {
    fn add(
        mut self,
        name: &str,
        align: Align,
        left: &Box,
        centered: &Box,
        right: &Box,
        box_holder: Option<&Box>,
    ) {
        let is_static = self.command.is_empty() || self.update_rate == 0;
        self.label.set_widget_name(name);
        self.label.set_tooltip_markup(Some(&self.tooltip));
        ui::add_and_align(&self.label, align, left, centered, right, box_holder);

        if self.listen {
            begin_listen(self.command.to_owned());
        }

        self.start_loop();

        if is_static {
            self.label.set_markup(&self.text);
        }

        log!(format!(
            "Added a new label widget named '{name}', static: {is_static}"
        ));
    }

    fn start_loop(&mut self) {
        // Start loops.
        start_tooltip_loop(self);
        start_label_loop(
            self.label.to_owned(),
            self.text.to_owned(),
            self.command.to_owned(),
            self.update_rate,
            self.listen,
        );
    }
}
