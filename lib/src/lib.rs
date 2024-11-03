use aes::Aes256;
use block_padding::Pkcs7;
use cbc::{Decryptor, Encryptor}; // CBC mode
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use utils::{generate_random_16_bytes, pad_string_to_32_bytes};

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

pub fn encode_bytes(input: Vec<u8>, password: String) -> Vec<u8> {
    // ensure the password is at least 32 bytes
    let key: [u8; 32] = pad_string_to_32_bytes(&password);

    // Generate a random IV
    let iv: [u8; 16] = generate_random_16_bytes();

    let mut buffer: Vec<u8> = input.to_vec();

    // Pad the plaintext to a multiple of 16 bytes
    buffer.resize(input.len() + 16, 0);

    // Encrypt the plaintext using AES-256-CBC
    let ciphertext = Aes256CbcEnc::new_from_slices(&key, &iv)
        .unwrap()
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, input.len())
        .unwrap();

    let mut concatenated = Vec::new();

    //Embed IV in the beginning of the file
    concatenated.extend_from_slice(&iv);

    //Embed ciphertext in the end of the file
    concatenated.extend_from_slice(&ciphertext);

    concatenated
}

pub fn decode_bytes(input: Vec<u8>, password: String) -> Vec<u8> {
    // Split the input into IV and ciphertext
    // The first 16 bytes are the IV, the rest is the ciphertext
    let iv = &input[..16];
    let ciphertext = &input[16..];

    // Generate the key from the password
    // Ensure the password is at least 32 bytes
    let key = pad_string_to_32_bytes(&password);

    let mut buffer = ciphertext.to_vec();

    // Decrypt the ciphertext using AES-256-CBC
    let decoded = Aes256CbcDec::new_from_slices(&key, &iv)
        .unwrap()
        .decrypt_padded_mut::<Pkcs7>(&mut buffer);

    if decoded.is_err() {
        panic!("Failed to decrypt file")
    }

    match decoded {
        Ok(decoded) => decoded.to_vec(),
        Err(_) => {
            panic!("Failed to decrypt file")
        }
    }
}
