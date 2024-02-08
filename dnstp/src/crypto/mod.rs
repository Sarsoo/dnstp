//! Method for handling cryptography including ECDH shared secret derivation and symmetric key encryption

#[cfg(test)]
mod tests;

use p256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret, NistP256};
use p256::elliptic_curve::ecdh::SharedSecret;
use aes_gcm_siv::{aead::{Aead, KeyInit}, AeadCore, Aes256GcmSiv, Nonce};

use rand_core::OsRng;

pub fn get_random_asym_pair() -> (EphemeralSecret, EncodedPoint)
{
    let secret = EphemeralSecret::random(&mut OsRng);
    let public_point = EncodedPoint::from(secret.public_key());

    (secret, public_point)
}

pub fn get_shared_asym_secret(secret: EphemeralSecret, opposing_public_key: EncodedPoint) -> Result<SharedSecret<NistP256>, ()> {

    match PublicKey::from_sec1_bytes(opposing_public_key.as_ref()) {
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

pub fn generate_aes_nonce() -> Nonce
{
    Aes256GcmSiv::generate_nonce(OsRng)
}

pub fn asym_to_sym_key(secret: &SharedSecret<NistP256>) -> Aes256GcmSiv
{
    Aes256GcmSiv::new(secret.raw_secret_bytes())
}

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