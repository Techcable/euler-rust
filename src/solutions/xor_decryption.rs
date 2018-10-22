use std::str::{self, FromStr};
use std::cmp::Ordering;

use itertools::Itertools;

const COMMON_ENGLISH_WORDS: &[&str] = &[
    "is", "has", "want", "too", "he", "she", "time", "person",
    "be", "have", "good", "new", "do"
];

pub fn solve() -> u64 {
    let raw_data: &str = include_str!("cipher.txt");
    let mut bytes = Vec::new();
    for n in raw_data.split(',') {
        bytes.push(u8::from_str(n).unwrap())
    }
    let mut best_match: Option<(usize, String)> = None;
    for key in ::utils::product(&(b'a'..=b'z').collect::<Vec<_>>(), 3) {
        if let Some(text) = decrypt_xor(&bytes, &key) {
            let common_words = text.split_whitespace()
                .filter(|t| COMMON_ENGLISH_WORDS.contains(t))
                .count();
            if !text.is_ascii() { continue } // Guarenteed to be ascii
            if text.chars().any(|c| c.is_ascii_control()) { continue }
            trace!("Decrypted {:?} with {} common words using {}", text, common_words, format_key(&key));
            if common_words == 0 {
                trace!("Zero common words for {:?} with key {}", text, format_key(&key));
            } else if best_match.is_none() {
                best_match = Some((common_words, text));
            } else {
                let best_match_words = best_match.as_ref().unwrap().0;
                match common_words.cmp(&best_match_words) {
                    Ordering::Less => {}, // ignore
                    Ordering::Equal => {
                        let best_match = &*best_match.as_ref().unwrap().1;
                        warn!(
                            "Equal number of common words ({}) for {:?} and {:?}",
                            common_words, text,
                            best_match
                        );
                    },
                    Ordering::Greater => {
                        {
                            let best_match = &*best_match.as_ref().unwrap().1;
                            debug!(
                                "Increased number of common words from {:?} ({}) to {:?} ({})",
                                best_match, best_match_words, text, common_words
                            )
                        }
                        best_match = Some((common_words, text));
                    },
                }
            }
        }
    }
    let (common_words, best_match) = best_match.unwrap();
    info!("Found best match {:?} with {} common words", best_match, common_words);
    best_match.chars().map(|s| s as u64).sum()
}

#[cfg_attr(not(test), allow(unused))]
fn encrypt_xor(text: &str, key: &[u8]) -> Vec<u8> {
    assert!(text.is_ascii());
    let mut result = Vec::with_capacity(text.len());
    for (&b, &key_byte) in text.as_bytes().iter().zip(key.iter().cycle()) {
        result.push(b ^ key_byte);
    }
    result
}
fn decrypt_xor(bytes: &[u8], key: &[u8]) -> Option<String> {
    let mut result = Vec::with_capacity(bytes.len());
    for (&b, &key_byte) in bytes.iter().zip(key.iter().cycle()) {
        result.push(b ^ key_byte);
    }
    match String::from_utf8(result) {
        Ok(value) => Some(value),
        Err(_) => {
            debug!("Key {} produces invalid UTF8", format_key(key));
            None
        }
    }
}

fn format_key(key: &[u8]) -> String {
    str::from_utf8(key).map(|s| format!("{:?}", s))
        .unwrap_or_else(|_| format!("{:?}", key))
}

#[cfg(test)]
mod test {
    use super::{decrypt_xor, encrypt_xor};
    const TEST_KEYS: &[&[u8]] = &[
        b"acd",
        b"zrt",
        b"fdw",
        b"piano player"
    ];
    const TEST_STRINGS: &[&str] = &[
        "The rain in spain falls gently on the plain.",
        "The quick brown fox jumps over the lazy dog.",
        "Everybody clap your hands!!!! :D"
    ];
    #[test]
    fn xor_encryption_roundtrip() {
        for &key in TEST_KEYS {
            for &text in TEST_STRINGS {
                let encrypted = encrypt_xor(text, key);
                let decrypted = decrypt_xor(&encrypted, key)
                    .unwrap();
                assert_eq!(decrypted, text)
            }
        }
    }
}