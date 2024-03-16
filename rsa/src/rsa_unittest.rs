mod rsa_rsa;
use rsa_rsa::{rsa_decrypt, rsa_encrypt};
mod openssl_rsa;
use base64ct::LineEnding;
use openssl::{pkey::PKey, rsa::Rsa};
use openssl_rsa::{openssl_rsa_decrypt, openssl_rsa_encrypt};
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};

fn check_encrypt_decrypt(private_key: &str, public_key: &str, data: &str) {
    println!("Private key:\n{}", private_key);
    println!("Public key:\n{}", public_key);
    {
        let encrypted = rsa_encrypt(public_key, data);
        // println!("Encrypted: {}", encrypted);
        let decrypted = rsa_decrypt(private_key, &encrypted);
        println!("Original: {} \nEncrypted: {}", data, decrypted);
        assert_eq!(data, decrypted);
    }
    {
        let openssl_encrypted = openssl_rsa_encrypt(public_key, data);
        // println!("Encrypted: {}", encrypted);
        let openssl_encrypted = openssl_rsa_decrypt(private_key, &openssl_encrypted);
        println!("Original: {} \nEncrypted: {}", data, openssl_encrypted);
        assert_eq!(data, openssl_encrypted);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rsa_test1() {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let priv_binding = priv_key
            .to_pkcs8_pem(LineEnding::LF)
            .expect("failed to encode private key");
        let pub_binding = pub_key
            .to_public_key_pem(LineEnding::LF)
            .expect("failed to encode public key");

        let private_key = priv_binding.as_str();
        let public_key = pub_binding.as_str();

        check_encrypt_decrypt(private_key, public_key, "admin");
    }

    #[test]
    fn rsa_test2() {
        let priv_key = Rsa::generate(2048).unwrap();
        let p_key = PKey::from_rsa(priv_key).unwrap();

        let private_key = p_key.private_key_to_pem_pkcs8().unwrap();
        let public_key = p_key.public_key_to_pem().unwrap();

        let private_key_str = std::str::from_utf8(&private_key).unwrap();
        let public_key_str = std::str::from_utf8(&public_key).unwrap();

        check_encrypt_decrypt(private_key_str, public_key_str, "password.");
    }
}
