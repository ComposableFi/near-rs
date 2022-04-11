#[cfg(test)]
use sha2::{Digest as DigestTrait, Sha256};

pub trait BlockValidation {
    type SignatureVerification;
    type Digest;

    fn validate_light_block(&self, block_view: Vec<u8>) -> bool;
}

trait Digest {
    fn digest(&self, data: impl AsRef<[u8]>) -> Vec<u8>;
}

#[cfg(test)]
pub struct Sha256Digest;

#[cfg(test)]
impl Digest for Sha256Digest {
    fn digest(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        Sha256::digest(data).to_vec()
    }
}

#[cfg(test)]
use crate::signature::DummyVerificator;

#[cfg(test)]
impl BlockValidation for Sha256Digest {
    type SignatureVerification = DummyVerificator;
    type Digest = Sha256Digest;

    fn validate_light_block(&self, block_view: Vec<u8>) -> bool {
        true
    }
}
