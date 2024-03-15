use openssl::rsa::{Padding, Rsa};

pub fn openssl_rsa_encrypt(public_key_pem: &str, message: &str) -> String {
    let rsa =
        Rsa::public_key_from_pem(public_key_pem.as_bytes()).expect("Failed to parse public key");
    let mut buf = vec![0; rsa.size() as usize];
    let encrypted_len = rsa
        .public_encrypt(message.as_bytes(), &mut buf, Padding::PKCS1)
        .unwrap();
    return openssl::base64::encode_block(&buf[..encrypted_len]);
}

pub fn openssl_rsa_decrypt(private_key_pem: &str, base64_data: &str) -> String {
    let encrypted_data = openssl::base64::decode_block(base64_data).unwrap();

    let rsa =
        Rsa::private_key_from_pem(private_key_pem.as_bytes()).expect("Failed to parse private key");
    let mut buf = vec![0; rsa.size() as usize];
    let decrypted_len = rsa
        .private_decrypt(&encrypted_data, &mut buf, Padding::PKCS1)
        .unwrap();
    return String::from_utf8(buf[..decrypted_len].to_vec())
        .expect("Failed to convert decrypted data to string");
}
