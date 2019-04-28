pub trait EnumValues: Sized {
    fn values() -> Vec<Self>;
}
