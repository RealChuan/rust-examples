use base64ct::{Base64, Encoding};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

pub fn rsa_encrypt(public_key_pem: &str, message: &str) -> String {
    let public_key =
        RsaPublicKey::from_public_key_pem(public_key_pem).expect("Failed to parse public key");
    let enc_data = public_key
        .encrypt(
            &mut rand::thread_rng(),
            rsa::Pkcs1v15Encrypt,
            message.as_bytes(),
        )
        .expect("Failed to encrypt message");

    return Base64::encode_string(&enc_data);
}

pub fn rsa_decrypt(private_key_pem: &str, encrypted_data: &str) -> String {
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(private_key_pem).expect("Failed to parse private key");
    let enc_data = Base64::decode_vec(encrypted_data).expect("Failed to decode base64 data");
    let dec_data = private_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("Failed to decrypt data");
    return String::from_utf8(dec_data).expect("Failed to convert decrypted data to string");
}
