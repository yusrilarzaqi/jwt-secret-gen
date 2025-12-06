use base64::Engine;
use rand::{rngs::OsRng, RngCore};

// Supported output formats for secrets.
#[derive(Clone, Copy, Debug)]
pub enum SecretFormat {
    Base64,
    UrlSafe,
    Hex,
}

// Generate a cryptographically secure random secret.
pub fn generate_secret(bytes: usize) -> Vec<u8> {
    let mut buf = vec![0u8; bytes];
    OsRng.fill_bytes(&mut buf);
    return buf;
}

// Encode raw bytes into the selected format.
pub fn encode_secret(raw: &[u8], format: SecretFormat) -> String {
    return match format {
        SecretFormat::Base64 => base64::engine::general_purpose::STANDARD.encode(raw),
        SecretFormat::UrlSafe => base64::engine::general_purpose::URL_SAFE.encode(raw),
        SecretFormat::Hex => raw.iter().map(|b| format!("{:02x}", b)).collect(),
    };
}

// Generate and encode a secret in one step
pub fn generate_encode(bytes: usize, format: SecretFormat) -> String {
    let raw = generate_secret(bytes);
    return encode_secret(&raw, format);
}
