use crate::errors::{Res, ToR};
use crate::load_env::get_env_var;
use aes_gcm::aead::{
    consts::{B0, B1, U12},
    generic_array::GenericArray,
    Aead, KeyInit,
};
use aes_gcm::aes::{
    cipher::typenum::{UInt, UTerm},
    Aes256,
};
use aes_gcm::{Aes256Gcm, AesGcm, Key, Nonce};

use base64::{engine::general_purpose::STANDARD, Engine};

type K = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>;

fn get_keys() -> Res<(AesGcm<Aes256, K>, GenericArray<u8, K>)> {
    let key_bytes = get_env_var("KEY")?.into_bytes();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let nonce_bytes = get_env_var("NONCE")?.into_bytes();
    let nonce = Nonce::<U12>::from_slice(&nonce_bytes);
    Ok((cipher, nonce.to_owned()))
}

pub fn encrypt(input: &str) -> Res<String> {
    let (cipher, nonce) = get_keys()?;
    Ok(STANDARD.encode(cipher.encrypt(&nonce, input.as_bytes()).to_r()?))
}

pub fn decrypt(encrypted: &str) -> Res<String> {
    let (cipher, nonce) = get_keys()?;
    let ciphertext = STANDARD.decode(encrypted).to_r()?;
    Ok(String::from_utf8_lossy(&cipher.decrypt(&nonce, ciphertext.as_ref()).to_r()?).to_string())
}
