use k256::{ecdsa::{SigningKey, Signature, VerifyingKey, signature::Signer}, elliptic_curve::rand_core::OsRng};
use sha2::{Sha256, Digest};
use hex;

pub fn generate_wallet() -> (String, SigningKey) {
    let signing_key = SigningKey::random(&mut OsRng);
    let public_key = VerifyingKey::from(&signing_key);
    (hex::encode(public_key.to_sec1_bytes()), signing_key)
}

pub fn sign_message(message: &str, key: &SigningKey) -> String {
    let digest = Sha256::digest(message.as_bytes());
    let sig: Signature = key.sign(digest.as_slice());
    hex::encode(sig.to_der())
}