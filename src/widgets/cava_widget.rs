use crate::{
    constants::ERR_ACCESS_CAVA_INSTANCES,
    ui,
    utils::cava,
    widget::{Align, HWidget},
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
    fn add(
        self,
        name: &str,
        align: Align,
        left: &Box,
        centered: &Box,
        right: &Box,
        box_holder: Option<&Box>,
    ) {
        self.label.set_widget_name(name);
        ui::add_and_align(&self.label, align, left, centered, right, box_holder);
        cava::CAVA_INSTANCES
            .lock()
            .expect(ERR_ACCESS_CAVA_INSTANCES)
            .push(self);
    }

    fn update_label_direct(&self, new_content: &str) {
        // Only redraw if the text wasn't the exact same as final_content.
        if self.label.text() != new_content {
            self.label.set_text(new_content)
        }
    }
}
