use near_primitives_wasm_friendly::PublicKey;

pub trait HostFunctions {
	fn sha256(data: &[u8]) -> [u8; 32];

	fn verify(&self, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool;
}
