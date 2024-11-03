use rand::Rng;

/// . Pads the key to 32 bytes
pub fn pad_string_to_32_bytes(key: &str) -> [u8; 32] {
    let mut key_bytes = [0u8; 32];
    let bytes = key.as_bytes();
    let len = bytes.len().min(32);
    key_bytes[..len].copy_from_slice(&bytes[..len]);
    key_bytes
}

/// . Generates a random IV
pub fn generate_random_16_bytes() -> [u8; 16] {
    rand::thread_rng().gen() // Generate a 16-byte random IV
}
