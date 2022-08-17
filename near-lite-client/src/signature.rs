use near_primitives_wasm_friendly::{PublicKey, Signature};
use sp_core::ed25519::Public as Ed25519Public;
use sp_io::crypto::ed25519_verify;

pub trait SignatureVerification {
	fn verify(&self, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool;
}

impl SignatureVerification for Signature {
	fn verify(&self, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool {
		match self {
			Self::Ed25519(signature) =>
				ed25519_verify(signature, data.as_ref(), &Ed25519Public::from(&public_key)),
		}
	}
}

#[cfg(test)]
pub struct DummySignature;

#[cfg(test)]
impl SignatureVerification for DummySignature {
	fn verify(&self, _data: impl AsRef<[u8]>, _public_key: PublicKey) -> bool {
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dummy_verificator() {
		let signature = DummySignature {};
		signature.verify(b"data", PublicKey([0; 32]));
	}
}
