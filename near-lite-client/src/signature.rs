use crate::types::Signature;

pub trait SignatureVerification {
    fn verify(&self, data: impl AsRef<[u8]>, public_keys: impl AsRef<[Signature]>) -> bool;
}

impl SignatureVerification for Signature {
    fn verify(&self, data: impl AsRef<[u8]>, public_keys: impl AsRef<[Signature]>) -> bool {
        // TODO: do a proper implementation here!
        true
    }
}
#[cfg(test)]
pub struct DummySignature {}

#[cfg(test)]
impl SignatureVerification for DummySignature {
    fn verify(&self, _data: impl AsRef<[u8]>, _public_keys: impl AsRef<[Signature]>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_verificator() {
        let signature = DummySignature {};
        signature.verify(b"data", vec![[0; 32]]);
    }
}
