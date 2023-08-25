use crate::{
    utils::cava,
    widget::{Align, HWidget},
    UI,
};
use gtk::{traits::*, *};

/// Creates a new label widget.
#[derive(Debug)]
pub struct CavaWidget {
    pub label: Label,
}

unsafe impl Send for CavaWidget {}
unsafe impl Sync for CavaWidget {}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for CavaWidget {
    fn add(self, ui: &UI, name: &str, align: Align, box_holder: Option<&Box>) {
        self.label.set_widget_name(name);
        ui.add_and_align(&self.label, align, box_holder);
        cava::CAVA_INSTANCES.write().push(self);
    }

    fn update_label_direct(&self, new_content: &str) {
        // Only redraw if the text wasn't the exact same as final_content.
        self.label.set_redraw_on_allocate(false);
        if self.label.text() != new_content {
            self.label.set_text(new_content)
        }
    }
}
