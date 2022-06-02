use sp_core::crypto::key_types;
use sp_io::crypto;
use runtime::new_test_ext;

mod runtime;

fn main() {
    let mut ext = new_test_ext();
    ext.execute_with(||{
        let msg = "Simple 32 bytes message for test";
        println!("Msg: \"{msg}\", len: {}", msg.len());

        let id = key_types::DUMMY;

        let ecdsa_key = crypto::ecdsa_generate(id, None);
        println!("ECDSA   Key: {ecdsa_key}");

        let ed25519_key = crypto::ed25519_generate(id, None);
        println!("ED25519 Key: {ed25519_key}");

        let sr25519_key = crypto::sr25519_generate(id, None);
        println!("SR25519 Key: {sr25519_key}");

        let msg = msg.as_bytes();

        let ecdsa_signature = crypto::ecdsa_sign(id, &ecdsa_key, msg).unwrap();
        println!("ECDSA Sig: {ecdsa_signature:?}, verified: {}", crypto::ecdsa_verify(&ecdsa_signature, msg, &ecdsa_key));

        let ed25519_signature = crypto::ed25519_sign(id, &ed25519_key, msg).unwrap();
        println!("ED25519 Sig: {ed25519_signature:?}, verified: {}", crypto::ed25519_verify(&ed25519_signature, msg, &ed25519_key));

        let sr25519_signature = crypto::sr25519_sign(id, &sr25519_key, msg).unwrap();
        println!("SR25519 Sig: {sr25519_signature:?}, verified: {}", crypto::sr25519_verify(&sr25519_signature, msg, &sr25519_key));
    });
}
