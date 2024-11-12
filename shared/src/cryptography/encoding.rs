use base64::{engine::general_purpose, Engine as _};

pub fn encode(data: &[u8]) -> String {
    let encoded = general_purpose::STANDARD.encode(data);
    return encoded;
}

pub fn decode(data: String) -> Vec<u8> {
    let decoded = general_purpose::STANDARD.decode(data.as_bytes()).unwrap();
    return decoded;
}
