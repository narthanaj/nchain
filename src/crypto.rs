use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::errors::{BlockchainError, Result};

#[derive(Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: PublicKey,
    signing_key: SigningKey,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(VerifyingKey);

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalSignature(Signature);

impl KeyPair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let mut secret_bytes = [0u8; 32];
        csprng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let public_key = PublicKey(signing_key.verifying_key());

        KeyPair {
            public_key,
            signing_key,
        }
    }

    pub fn from_private_key_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 32 {
            return Err(BlockchainError::InvalidTransaction {
                message: "Private key must be 32 bytes".to_string(),
            });
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);

        let signing_key = SigningKey::from_bytes(&key_bytes);
        let public_key = PublicKey(signing_key.verifying_key());

        Ok(KeyPair {
            public_key,
            signing_key,
        })
    }

    pub fn sign(&self, message: &[u8]) -> DigitalSignature {
        let signature = self.signing_key.sign(message);
        DigitalSignature(signature)
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn to_private_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
}

impl PublicKey {
    pub fn verify(&self, message: &[u8], signature: &DigitalSignature) -> bool {
        self.0.verify(message, &signature.0).is_ok()
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 32 {
            return Err(BlockchainError::InvalidTransaction {
                message: "Public key must be 32 bytes".to_string(),
            });
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);

        let verifying_key = VerifyingKey::from_bytes(&key_bytes)
            .map_err(|_| BlockchainError::InvalidTransaction {
                message: "Invalid public key format".to_string(),
            })?;

        Ok(PublicKey(verifying_key))
    }

    pub fn to_address(&self) -> String {
        hex::encode(&self.to_bytes()[..8])
    }
}

impl DigitalSignature {
    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 64 {
            return Err(BlockchainError::InvalidTransaction {
                message: "Signature must be 64 bytes".to_string(),
            });
        }

        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(bytes);

        let signature = Signature::from_bytes(&sig_bytes);
        Ok(DigitalSignature(signature))
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()))
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({})", self.to_address())
    }
}

impl fmt::Display for DigitalSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()))
    }
}

impl fmt::Debug for DigitalSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({}...)", &hex::encode(self.to_bytes())[..16])
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub keypair: KeyPair,
    pub name: String,
}

impl Wallet {
    pub fn new(name: String) -> Self {
        Wallet {
            keypair: KeyPair::generate(),
            name,
        }
    }

    pub fn from_private_key(name: String, private_key: &[u8]) -> Result<Self> {
        Ok(Wallet {
            keypair: KeyPair::from_private_key_bytes(private_key)?,
            name,
        })
    }

    pub fn address(&self) -> String {
        self.keypair.public_key.to_address()
    }

    pub fn sign_transaction(&self, transaction_data: &[u8]) -> DigitalSignature {
        self.keypair.sign(transaction_data)
    }
}

impl fmt::Debug for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wallet")
            .field("name", &self.name)
            .field("address", &self.address())
            .finish()
    }
}