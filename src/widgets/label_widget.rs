use crate::{
    config::Config,
    constants::{ERR_NO_LINES, ERR_STRING_NONE, ERR_TAKE_STDOUT, PROC_TARGET},
    ui,
    utils::aliases::use_aliases,
    widget::{Align, HWidget},
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
    pub revealer: Revealer,
    pub update_anim: RevealerTransitionType,
    pub anim_duration: u32,
    pub config: &'static Config,
}

/// 0.3.2: If `listen` is `true`, call this function and then set the label text-value
///   to that of `BUFFER`.
fn begin_listen(cmd: String, config: &'static Config) {
    task::spawn(async move {
        let mut child = Command::new(PROC_TARGET)
            .args(["-c", &cmd])
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap_or_else(|_| panic!("[ERROR] Cannot start '{cmd}'!"));

        let out = child.stdout.take().expect(ERR_TAKE_STDOUT);

        let mut reader = BufReader::new(out).lines();
        let update_rate = config.get_update_rate();
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
    let mut tooltip = take(&mut label_ref.tooltip);
    let initial_tooltip_len = tooltip.len();
    let tooltip_command = take(&mut label_ref.tooltip_command);
    let mut tick = move || {
        // Remove content after the initial static text-value, assuming there was any static
        // text specified.
        if tooltip.len() > initial_tooltip_len {
            tooltip.drain(initial_tooltip_len..tooltip.len());
        }

        tooltip.push_str(&use_aliases(&tooltip_command));

        let tooltip_markup = label.tooltip_markup().unwrap_or_else(|| GString::from(""));
        if !tooltip_markup.eq(&tooltip) {
            // Markup support here, the user therefore has to deal with any upcoming issues due to
            // the command output, on their own.
            label.set_tooltip_markup(Some(&tooltip));
        }

        glib::Continue(true)
    };

    tick();
    glib::timeout_add_local(Duration::from_millis(1000), tick);
}

/// Starts updating the dynamic label content.
fn start_label_loop(label_ref: &mut LabelWidget) {
    let command = label_ref.command.to_owned();
    if command.is_empty() || label_ref.update_rate <= 3 {
        // Not eligible, cancel.
        return;
    }

    let label = label_ref.label.clone();
    let mut text = take(&mut label_ref.text);
    let initial_text_len = text.len();
    let listen = take(&mut label_ref.listen);
    let update_anim = label_ref.update_anim;
    let revealer = take(&mut label_ref.revealer);
    let anim_speed = take(&mut label_ref.anim_duration);
    let mut tick = move || {
        if !listen {
            // Remove content after the initial static text-value, assuming there was any static
            // text specified.
            // This ensures that the static text-value is preserved, and that the command output is
            // appended after it.
            if text.len() > initial_text_len {
                text.drain(initial_text_len..text.len());
            }

            text.push_str(&use_aliases(&command));

            if !label.text().eq(&text) {
                restart_revealer!(revealer, || label.set_text(&text), update_anim, anim_speed);
            }
        } else {
            update_from_buffer(&label, &revealer, update_anim, anim_speed);
        }

        glib::Continue(true)
    };

    tick();
    glib::timeout_add_local(Duration::from_millis(label_ref.update_rate), tick);
}

/// Updates the labels content with the string from `BUFFER`.
fn update_from_buffer(
    label: &Label,
    revealer: &Revealer,
    update_anim: RevealerTransitionType,
    anim_speed: u32,
) {
    if let Ok(new_content) = BUFFER.lock() {
        let old_content = label.text();
        // eq-check the new content for old_content. Doing the opposite requires a .to_string()
        // call.
        if !new_content.eq(&old_content) {
            // Not the same; set content and redraw.
            restart_revealer!(
                revealer,
                || label.set_text(&new_content),
                update_anim,
                anim_speed
            );
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
    fn add(mut self, name: &str, align: Align, box_holder: Option<&Box>) {
        let is_static = self.command.is_empty() || self.update_rate == 0;
        self.label.set_widget_name(name);
        self.label.set_markup(&self.text);
        self.label.set_tooltip_markup(Some(&self.tooltip));
        self.revealer.set_child(Some(&self.label));
        self.revealer.set_transition_type(self.update_anim);
        ui::add_and_align(&self.revealer, align, box_holder);

        // 0.4.9: If the reveal_anim is unset, None or the label is static, then reveal instantly.
        if self.update_anim == RevealerTransitionType::None || is_static {
            self.revealer.set_reveal_child(true);
        }

        if !is_static {
            if self.listen {
                begin_listen(self.command.to_owned(), self.config);
            }

            self.start_loop();
        }

        log!(format!(
            "Added a new label widget named '{name}', static: {is_static}"
        ));
    }

    fn start_loop(&mut self) {
        // Start loops.
        start_tooltip_loop(self);
        start_label_loop(self);
    }
}
