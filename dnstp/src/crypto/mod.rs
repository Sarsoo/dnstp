//! Method for handling cryptography including ECDH shared secret derivation and symmetric key encryption
//!
//! **Step 1**: [`get_random_asym_pair`] Generate a public/private key pair on each side
//!
//! **Step 2**: Swap the [`p256::EncodedPoint`]s from step 1 between parties
//!
//! **Step 3**: [`get_shared_asym_secret`] Combine one private key with the other public key to end up at the same shared secret
//!
//! **Step 4**: [`asym_to_sym_key`] Take the [`p256::NistP256`] shared asymmetric secret key and use it as a symmetric key ready for encryption decryption
//!
//! **Step 5**: [`generate_aes_nonce`] Get a nonce to use when encrypting
//!
//! **Step 6**: [`encrypt`] Use the key from step 4 with the nonce from step 5 to encrypt arbitrary data
//!
//! **Step 7**: [`decrypt`] Use the same key from step 6 and ***the same nonce from step 6*** to decrypt the outputted ciphertext from step 6.

#[cfg(test)]
mod tests;

use std::str::FromStr;
use p256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret, NistP256};
use p256::elliptic_curve::ecdh::SharedSecret;
use aes_gcm_siv::{aead::{Aead, KeyInit}, AeadCore, Aes256GcmSiv, Nonce};

use rand_core::OsRng;

pub const PUBLIC_KEY_OPENING: &str = "-----BEGIN PUBLIC KEY-----\n";
pub const PUBLIC_KEY_CLOSING: &str = "\n-----END PUBLIC KEY-----\n";

/// Generate a public/private key pair
pub fn get_random_asym_pair() -> (EphemeralSecret, String)
{
    let secret = EphemeralSecret::random(&mut OsRng);
    let public_point = secret.public_key().to_string();

    (secret, public_point)
}

/// Use one private key and an opposing public key to arrive at the same shared secret
pub fn get_shared_asym_secret(secret: EphemeralSecret, opposing_public_key: String) -> Result<SharedSecret<NistP256>, ()> {

    match PublicKey::from_str(&opposing_public_key) {
        Ok(other_public) => {
            Ok(secret.diffie_hellman(&other_public))
        }
        Err(_) => {
            Err(())
        }
    }
}

// pub fn generate_aes_nonce() -> Nonce
// {
//     let mut nonce_buffer: [u8; 12] = [0; 12];
//     &OsRng.fill_bytes(&mut nonce_buffer);
//
//     Nonce::from(nonce_buffer)
// }

/// Generate a safe nonce to use in symmetric encryption
pub fn generate_aes_nonce() -> Nonce
{
    Aes256GcmSiv::generate_nonce(OsRng)
}

/// Turn the asymmetric shared secret into a symmetric encryption key
pub fn asym_to_sym_key(secret: &SharedSecret<NistP256>) -> Aes256GcmSiv
{
    Aes256GcmSiv::new(secret.raw_secret_bytes())
}

/// Symmetrically encrypt data using a key derived from ECDH
pub fn encrypt(key: &Aes256GcmSiv, nonce: &Nonce, bytes: &Vec<u8>) -> Result<Vec<u8>, ()>
{
    match key.encrypt(nonce, bytes.as_ref()) {
        Ok(r) => {
            Ok(r)
        }
        Err(_) => {
            Err(())
        }
    }
}

/// Symmetrically decrypt data using a key derived from ECDH
pub fn decrypt(key: &Aes256GcmSiv, nonce: &Nonce, bytes: &Vec<u8>) -> Result<Vec<u8>, ()>
{
    match key.decrypt(nonce, bytes.as_ref()) {
        Ok(r) => {
            Ok(r)
        }
        Err(_) => {
            Err(())
        }
    }
}

pub fn trim_public_key(public: &String) -> String
{
    public[27.. 125 + 27].to_string().replace('\n', ".")
}

pub fn fatten_public_key(public: &String) -> String
{
    let mut fattened = public.clone();
    fattened.insert_str(0, PUBLIC_KEY_OPENING);
    fattened.push_str(PUBLIC_KEY_CLOSING);

    fattened.replace('.', "\n")
}