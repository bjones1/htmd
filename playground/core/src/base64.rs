//! Unpadded base64url ([RFC 4648 §5](https://www.rfc-editor.org/rfc/rfc4648#section-5)).

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub fn encode(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let group = chunk.iter().enumerate().fold(0u32, |group, (index, byte)| {
            group | u32::from(*byte) << (16 - 8 * index)
        });
        for index in 0..=chunk.len() {
            text.push(ALPHABET[(group >> (18 - 6 * index) & 63) as usize] as char);
        }
    }
    text
}

/// Decode strictly: padding, characters outside the alphabet, an impossible
/// length, or nonzero leftover bits make the text invalid.
pub fn decode(text: &str) -> Option<Vec<u8>> {
    if text.len() % 4 == 1 {
        return None;
    }
    let mut bytes = Vec::with_capacity(text.len() * 3 / 4);
    for chunk in text.as_bytes().chunks(4) {
        let mut group = 0u32;
        for (index, char) in chunk.iter().enumerate() {
            let value = ALPHABET.iter().position(|a| a == char)? as u32;
            group |= value << (18 - 6 * index);
        }
        let byte_count = chunk.len() - 1;
        if group & ((1 << (24 - 8 * byte_count)) - 1) != 0 {
            return None;
        }
        bytes.extend((0..byte_count).map(|index| (group >> (16 - 8 * index)) as u8));
    }
    Some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_rfc_4648_vectors() {
        for (plain, encoded) in [
            ("", ""),
            ("f", "Zg"),
            ("fo", "Zm8"),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg"),
            ("fooba", "Zm9vYmE"),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(encode(plain.as_bytes()), encoded);
            assert_eq!(decode(encoded).as_deref(), Some(plain.as_bytes()));
        }
        assert_eq!(encode(&[0xfb, 0xff]), "-_8");
    }

    #[test]
    fn rejects_malformed_text() {
        for text in ["Z", "Zg==", "Zh", "Z+8", "Zm9v!"] {
            assert_eq!(decode(text), None, "{text}");
        }
    }
}
