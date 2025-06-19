// Implement extract_tx_version function below
use hex::FromHexError;
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let transaction_bytes = hex::decode(raw_tx_hex);
    match transaction_bytes {
        Ok(bytes) => {
            if bytes.len() < 4 {
                return Err("Transaction data too short".to_string());
            }
            let version_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
            Ok(u32::from_le_bytes(version_bytes))
        }
        Err(e) => {
            if let FromHexError::InvalidHexCharacter { c, index } = e {
                Err(format!(
                    "Hex decode error: Invalid hex character '{}' at position {}",
                    c, index
                ))
            } else {
                Err(e.to_string())
            }
        }
    }
}
