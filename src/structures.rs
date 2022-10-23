use strum::EnumString;

/// Widget alignment.
// Allow for non-uppercase enum namings here.
#[allow(clippy::upper_case_acronyms)]
#[derive(EnumString)]
pub enum Align {
    LEFT,
    CENTERED,
    RIGHT,
}
