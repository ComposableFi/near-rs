pub trait HostFunctions {
    fn sha256(data: &[u8]) -> [u8; 32];
}
