use base64::{engine, Engine as _};
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::rand::SystemRandom;
use serde_json;

pub struct SignatureManager {
    key_pair: Ed25519KeyPair,
}

impl SignatureManager {
    pub fn new() -> Self {
        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();

        SignatureManager { key_pair }
    }

    pub fn _sign(&self, message: &Vec<f64>) -> Vec<u8> {
        let serialized_prices = serde_json::to_string(&message).unwrap();
        self.key_pair.sign(serialized_prices.as_bytes()).as_ref().to_vec()
    }

    pub fn _public_key(&self) -> Vec<u8> {
        self.key_pair.public_key().as_ref().to_vec()
    }

    pub fn public_key_str(&self) -> String {
        let engine = engine::general_purpose::STANDARD_NO_PAD;
        engine.encode(self.key_pair.public_key().as_ref())
    }


    pub fn _verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        let public_key = UnparsedPublicKey::new(&ED25519, public_key);
        public_key.verify(message, signature).is_ok()
    }

}