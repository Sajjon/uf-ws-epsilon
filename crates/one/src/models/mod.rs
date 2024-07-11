#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct One(bool);

uniffi::custom_newtype!(One, bool);

#[uniffi::export]
pub fn new_one(value: bool) -> One {
    One(value)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Debug)]
pub struct OneObject {
    one: One,
}

#[uniffi::export]
impl OneObject {
    #[uniffi::constructor]
    pub fn new(one: One) -> Self {
        Self { one }
    }
}
