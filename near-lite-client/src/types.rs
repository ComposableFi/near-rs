pub(crate) type Signature = [u8; 32];

pub enum SignatureType {
    Ed25519,
}

pub type EpochId = Vec<u8>;

pub type BlockProducer = (SignatureType, Signature);
