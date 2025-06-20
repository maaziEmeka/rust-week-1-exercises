// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }
    let bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;
    let version_bytes: [u8; 4] = bytes[..4].try_into().unwrap();
    Ok(u32::from_le_bytes(version_bytes))
}
