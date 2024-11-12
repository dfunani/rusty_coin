use sha2::{Digest, Sha256};

use super::encoding::encode;

pub fn hash(data: impl AsRef<[u8]>) -> String {
    let mut client = Sha256::new();
    client.update(data);
    let hash_value = client.finalize();

    return encode(&hash_value);
}
