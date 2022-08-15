use near_crypto::{Signature, PublicKey};
use sp_core::ed25519::Public as Ed25519Public;
use sp_io::crypto::ed25519_verify;

pub trait SignatureVerification {
    fn verify(&self, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool;
}

impl SignatureVerification for Signature {
    fn verify(&self, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool {
        match (self, public_key) {
            (Self::ED25519(signature), PublicKey::ED25519(pulic_key)) => {
                ed25519_verify(&sp_core::ed25519::Signature::from_raw(signature.to_bytes()), data.as_ref(), &Ed25519Public(pulic_key.0.clone()))
            }
            _ => unimplemented!()
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
        signature.verify(b"data", PublicKey::ED25519([0; 32].into()));
    }
}
