use crate::{NearSignature, PublicKey};
use digest::{typenum::U32, Digest, FixedOutput};
use k256::ecdsa::{SigningKey, VerifyingKey};
use tendermint::crypto::CryptoProvider;

pub trait HostFunctions: CryptoProvider {
	// type CP: CryptoProvider;
	fn sha256(data: &[u8]) -> [u8; 32] {
		let mut hasher = <Self as CryptoProvider>::Sha256::new();
		hasher.update(data);
		let result = hasher.finalize().try_into().unwrap();
		result
	}

	fn verify(signature: NearSignature, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool {
		signature.verify(data, public_key)
	}
}

/// A draft for an imlpementation of the HostFunctionManager for a specific chain (i.e. Polkadot/Near)
/// that uses the [`CryptoProvider`] trait
use core::marker::PhantomData;
use signature::{DigestSigner, DigestVerifier, Signer, Verifier};

struct NearHostFunctionsManager;

#[derive(Debug, Default)]
pub struct NearSha256(sha2::Sha256);

#[derive(Debug)]

pub struct NearSigner<D> {
	inner: SigningKey,
	_d: PhantomData<D>,
}
#[derive(Debug)]
pub struct NearSignatureVerifier<D> {
	inner: VerifyingKey,
	_d: PhantomData<D>,
}

impl<D: Digest + FixedOutput<OutputSize = U32>> NearSignatureVerifier<D> {
	fn from_bytes(public_key: &[u8]) -> Result<Self, ed25519::Error> {
		Ok(Self { inner: VerifyingKey::from_sec1_bytes(public_key)?, _d: PhantomData::default() })
	}
}

impl<D: Digest + FixedOutput<OutputSize = U32>, S: signature::Signature> DigestVerifier<D, S>
	for NearSignatureVerifier<D>
where
	VerifyingKey: DigestVerifier<D, S>,
{
	fn verify_digest(&self, digest: D, signature: &S) -> Result<(), ed25519::Error> {
		self.inner.verify_digest(digest, signature)
	}
}

impl<S: signature::PrehashSignature, D: Digest + FixedOutput<OutputSize = U32>> Verifier<S>
	for NearSignatureVerifier<D>
where
	VerifyingKey: DigestVerifier<D, S>,
{
	fn verify(&self, msg: &[u8], signature: &S) -> Result<(), ed25519::Error> {
		let mut hasher = D::new();
		Digest::update(&mut hasher, msg);
		self.verify_digest(hasher, signature)
	}
}

impl digest::OutputSizeUser for NearSha256 {
	type OutputSize = U32;
}

impl digest::HashMarker for NearSha256 {}

impl digest::Update for NearSha256 {
	fn update(&mut self, data: &[u8]) {
		Digest::update(&mut self.0, data)
	}
}

impl FixedOutput for NearSha256 {
	fn finalize_into(self, out: &mut digest::Output<Self>) {
		*out = self.0.finalize();
	}
}

impl<D: Digest, S: signature::Signature> Signer<S> for NearSigner<D>
where
	SigningKey: DigestSigner<D, S>,
{
	fn try_sign(&self, msg: &[u8]) -> Result<S, ed25519::Error> {
		let mut hasher = D::new();
		Digest::update(&mut hasher, msg);
		self.inner.try_sign_digest(hasher)
	}
}

impl CryptoProvider for NearHostFunctionsManager {
	type Sha256 = NearSha256;

	type EcdsaSecp256k1Signer = NearSigner<Self::Sha256>;
	type EcdsaSecp256k1Verifier = NearSignatureVerifier<Self::Sha256>;
}

// impl NearHostFunctions for NearHostFunctionsManager {
// 	fn sha2_256(preimage: &[u8]) -> [u8; 32] {
// 		let mut hasher = <Self as CryptoProvider>::Sha256::new();
// 		hasher.update(preimage);
// 		let result = hasher.finalize().try_into().unwrap();
// 		result
// 	}
// 	fn ed25519_verify(sig: &[u8], msg: &[u8], pub_key: &[u8]) -> Result<(), ()> {
// 		let verifier =
// 			<<Self as CryptoProvider>::EcdsaSecp256k1Verifier>::from_bytes(pub_key).unwrap();
// 		let signature = k256::ecdsa::Signature::from_der(sig).unwrap();
// 		Ok(verifier.verify(msg, &signature).unwrap())
// 	}

// 	fn secp256k1_verify(_sig: &[u8], _message: &[u8], _public: &[u8]) -> Result<(), ()> {
// 		unimplemented!()
// 	}
// }
