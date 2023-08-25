use gtk::prelude::RevealerExt;
use gtk::RevealerTransitionType;

/// UI Utilities.
pub struct UIUtils;

impl UIUtils {
    /// Restarts the given revealer, with the specified `animation` and a custom speed.
    pub fn restart_revealer<F: FnOnce()>(
        revealer: &gtk::Revealer,
        animation: RevealerTransitionType,
        speed: u32,
        after_closure: F,
    ) {
        if animation == RevealerTransitionType::None {
            // No transition, skip full restart and instead just call directly.
            after_closure();
        } else {
            revealer.set_transition_duration(0);
            revealer.set_reveal_child(false);
            revealer.set_transition_type(RevealerTransitionType::None);
            after_closure();
            revealer.set_transition_duration(speed);
            revealer.set_transition_type(animation);
            revealer.set_reveal_child(true);
        }
    }
}
