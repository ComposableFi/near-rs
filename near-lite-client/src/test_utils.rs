use near_primitives_wasm_friendly::HostFunctions;

#[cfg(test)]
pub struct MockedHostFunctions;

#[cfg(any(test))]
impl HostFunctions for MockedHostFunctions {
	fn sha256(data: &[u8]) -> [u8; 32] {
		use sha2::Digest;
		sha2::Sha256::digest(data).try_into().unwrap()
	}

	fn verify(
		signature: near_primitives_wasm_friendly::Signature,
		data: impl AsRef<[u8]>,
		public_key: near_primitives_wasm_friendly::PublicKey,
	) -> bool {
		todo!()
	}
}
