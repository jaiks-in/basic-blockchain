use p256::ecdsa::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct Wallet {
    pub private_key: SigningKey,
    pub public_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let private_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&private_key);
        Wallet {
            private_key,
            public_key,
        }
    }
}
