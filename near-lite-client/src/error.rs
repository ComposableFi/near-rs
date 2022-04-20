use crate::types::ConversionError;
use borsh::maybestd::io::Error as BorshError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NearLiteClientError {
    #[error("Borsh error {0}")]
    Borsh(BorshError),

    #[error("Conversion error {0}")]
    Conversion(ConversionError),
}

// Had to implement this variant manually due to some traits missing on the
// Borsh side to be fully compatible w/ `thiserror`
impl From<BorshError> for NearLiteClientError {
    fn from(err: BorshError) -> Self {
        Self::Borsh(err)
    }
}

// Had to implement this variant manually due to some traits missing on the
// Borsh side to be fully compatible w/ `thiserror`
impl From<ConversionError> for NearLiteClientError {
    fn from(err: ConversionError) -> Self {
        Self::Conversion(err)
    }
}
