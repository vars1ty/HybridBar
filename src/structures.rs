use strum::EnumString;

/// Widget alignment.
#[derive(EnumString)]
pub enum Align {
    LEFT,
    CENTERED,
    RIGHT,
}
