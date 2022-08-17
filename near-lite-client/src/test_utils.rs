use near_merkle_proofs::HostFunctions;

#[cfg(test)]
pub struct MockedHostFunctions;

#[cfg(test)]
impl HostFunctions for MockedHostFunctions {
	fn sha256(data: &[u8]) -> [u8; 32] {
		use sha2::Digest;
		sha2::Sha256::digest(data).try_into().unwrap()
	}
}
