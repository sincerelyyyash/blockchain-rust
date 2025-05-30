use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use k256::ecdsa::{VerifyingKey, Signature, signature::Verifier};
use hex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction{
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: String,
}

impl Transaction{
    pub fn hash(&self) -> String{
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.from, self.to, self.amount));
        hex::encode(hasher.finalize())
    }

    pub fn is_valid(&self) -> bool {
        if self.from == "0"{
            return true;
    }

    let public_key_bytes = match hex::decode(&self.from){
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let signature_bytes = match hex::decode(&self.signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let verifying_key = match VerifyingKey::from_sec1_bytes(&public_key_bytes){
        Ok(key) => key,
        Err(_)=> return false,
    };

    let signature = match Signature::from_der(&signature_bytes){
        Ok(sig) => sig,
        Err(_) => return false,
    };

    let msg_hash = Sha256::digest(format!("{}{}{}", self.from, self.to, self.amount).as_bytes());

    verifying_key.verify(msg_hash.as_slice(), &signature).is_ok()
}
}