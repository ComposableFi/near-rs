use crate::types::Signature;

pub trait SignatureVerification {
    fn verify(&self, data: impl AsRef<[u8]>, public_keys: impl AsRef<[Signature]>) -> bool;
}

#[cfg(test)]
pub struct DummyVerificator {}

#[cfg(test)]
impl SignatureVerification for DummyVerificator {
    fn verify(&self, _data: impl AsRef<[u8]>, _public_keys: impl AsRef<[Signature]>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_verificator() {
        let dummy_verificator = DummyVerificator {};
        dummy_verificator.verify(b"data", vec![[0; 32]]);
    }
}
