use arraystring::{
    typenum::{U100, U200, U25},
    ArrayString,
};

pub type SmallString = ArrayString<U25>;
pub type MediumString = ArrayString<U100>;
pub type LargeString = ArrayString<U200>;
