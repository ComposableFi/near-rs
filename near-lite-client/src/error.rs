use alloc::string::String;
use borsh::maybestd::io::Error as BorshError;
use near_primitives_wasm::ConversionError;

#[derive(Debug)]
pub enum NearLiteClientError {
	Borsh(BorshError),
	Conversion(ConversionError),
	ProofVerificationError(String),
	InvalidLiteBlock(String),
	SignatureVerification(String),
	TransactionValidation(String),
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

impl From<String> for NearLiteClientError {
	fn from(err: String) -> Self {
		Self::ProofVerificationError(err)
	}
}
