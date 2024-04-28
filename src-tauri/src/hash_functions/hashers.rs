use hex;
use sha2::{Digest, Sha256};
use sha3::Sha3_256;
use tauri;


/// Converts a string to SHA2-256 String.
///
/// See 'https://emn178.github.io/online-tools/sha256.html'
#[tauri::command]
pub fn sha2_256(data: &[u8]) -> String {
    let hash_result = Sha256::digest(data);
    hex::encode(hash_result)
}

/// Converts a string to SHA3-256 String.
///
/// See https://emn178.github.io/online-tools/sha3_256.html
#[tauri::command]
pub fn sha3_256(data: &[u8]) -> String {
    let hash_result = Sha3_256::digest(data);
    hex::encode(hash_result)
}