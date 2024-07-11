use std::sync::Arc;

use one::OneObject;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Two(bool);

uniffi::custom_newtype!(Two, bool);

#[uniffi::export]
pub fn new_two_default() -> Two {
    Two::default()
}

#[uniffi::export]
pub fn new_two(value: bool) -> Two {
    Two(value)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Debug)]
pub struct EpsilonObject {
    one_object: Arc<OneObject>,
    two: Two,
}

#[uniffi::export]
impl EpsilonObject {
    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[uniffi::constructor]
    pub fn new(one_object: Arc<OneObject>, two: Two) -> Self {
        Self { one_object, two }
    }
}
