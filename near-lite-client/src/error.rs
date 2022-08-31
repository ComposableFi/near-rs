use alloc::string::String;
use borsh::maybestd::io::Error as BorshError;
use derive_more::{Display, From};
use near_primitives_wasm::ConversionError;

#[derive(Debug, Display, From)]
pub enum NearLiteClientError {
	#[display(fmt = "borsh serialization error: {_0}")]
	#[from]
	Borsh(BorshError),
	#[display(fmt = "conversion error: {_0}")]
	#[from]
	Conversion(ConversionError),
	#[display(fmt = "invalid epoch id: {_0}")]
	#[from]
	ProofVerificationError(String),
	#[display(fmt = "invalid signature: {_0}")]
	InvalidLiteBlock(String),
	#[display(fmt = "invalid timestamp: {_0}")]
	SignatureVerification(String),
	#[display(fmt = "invalid timestamp: {_0}")]
	TransactionValidation(String),
}
