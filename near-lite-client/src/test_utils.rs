use near_primitives_wasm::{
	host_functions::{NearSha256, NearSignatureVerifier, NearSigner},
	HostFunctions,
};
use tendermint::crypto::CryptoProvider;

#[cfg(test)]
pub struct MockedHostFunctions;

impl CryptoProvider for MockedHostFunctions {
	type Sha256 = NearSha256;

	type EcdsaSecp256k1Signer = NearSigner<Self::Sha256>;
	type EcdsaSecp256k1Verifier = NearSignatureVerifier<Self::Sha256>;
}

#[cfg(any(test))]
impl HostFunctions for MockedHostFunctions {
	fn sha256(data: &[u8]) -> [u8; 32] {
		use sha2::Digest;
		sha2::Sha256::digest(data).try_into().unwrap()
	}

	fn verify(
		signature: near_primitives_wasm::NearSignature,
		data: impl AsRef<[u8]>,
		public_key: near_primitives_wasm::PublicKey,
	) -> bool {
		todo!()
	}
}
