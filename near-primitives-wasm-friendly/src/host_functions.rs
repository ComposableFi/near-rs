use crate::{PublicKey, Signature};

pub trait HostFunctions {
	fn sha256(data: &[u8]) -> [u8; 32];

	fn verify(signature: Signature, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool;
}
