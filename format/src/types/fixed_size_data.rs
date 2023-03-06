

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
pub struct FixedSizeBytes<const N: usize>(Box<[u8; N]>);

impl<const N: usize> TryFrom<&[u8]> for FixedSizeBytes<N> {
    type Error = Error;

    fn try_from(buf: &[u8]) -> Result<FixedSizeBytes<N>> {
        let buf: [u8; N] = buf.try_into().map_err(Error::ArrayFromSlice);

        Ok(FixedSizeBytes(Box::new(buf)))
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for FixedSizeBytes<N> {
    type Error = Error;
    
    fn try_from(buf: Vec<u8>) -> Result<FixedSizeBytes<N>> {
        let buf: Box<[u8; N]> = buf.try_into().map_err(|| UnexpectedLength {
            expected: N,
            got: buf.len(), 
        })?;
        
        Ok(FixedSizeBytes(buf))
    }
}
