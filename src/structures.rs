use strum::EnumString;

/*
/// Easy mutable implementions for GTKWidget.
impl GTKWidget {
    /// Creates a button.
    pub fn create_button(&mut self, name: &str) {
        let button = Button::new();
        button.set_widget_name(name);
        self.button = Some(button);
    }

    /// Creates a box.
    pub fn create_spacing(&mut self, spacing_start: i32, spacing_end: i32) {
        let w_box = widget_builder::create_box();
        w_box.set_margin_start(spacing_start);
        w_box.set_margin_end(spacing_end);
        self.spacing = Some(w_box);
    }
}
*/

/// Widget alignment.
#[derive(EnumString)]
pub enum Align {
    LEFT,
    CENTERED,
    RIGHT,
}
