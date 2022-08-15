use near_primitives_wasm_friendly::ConversionError;
use borsh::maybestd::io::Error as BorshError;

#[derive(Debug)]
pub enum NearLiteClientError {
    Borsh(BorshError),
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
