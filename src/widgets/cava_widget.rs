use crate::{cava, structures::Align, ui, widget::HWidget};
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
    fn add<'a>(self, name: &'a str, align: Align, left: &Box, centered: &Box, right: &Box) {
        self.label.set_widget_name(name);
        ui::add_and_align(&self.label, align, left, centered, right);
        cava::CAVA_INSTANCES
            .lock()
            .expect("[ERROR] Couldn't access ui::CAVA_INSTANCES!")
            .push(self)
            .expect("[ERROR] You can't have more than `8` Cava widgets per Hybrid config!");
    }

    fn update_label_direct(&self, new_content: &str) {
        // Only redraw if the text wasn't the exact same as final_content.
        if !self.label.text().eq(new_content) {
            self.label.set_text(new_content)
        }
    }
}
