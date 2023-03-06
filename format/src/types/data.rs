#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct Data(Vec<u8>);

impl From<&[u8]> for Data {
    fn from(buf: &[u8]) -> Data {
        Data(buf.to_owned())
    }
}
