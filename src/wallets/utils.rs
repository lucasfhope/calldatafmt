use anyhow::{anyhow, Result};

pub(crate) fn clean_and_validate_calldata(calldata_in: &str) -> Result<&str> {
    let calldata = get_calldata_without_prefix(calldata_in);

    validate_calldata_length(calldata)?;
    validate_calldata_hex_characters(calldata)?;

    Ok(calldata)
}

fn get_calldata_without_prefix(calldata_in: &str) -> &str {
    calldata_in
        .strip_prefix("0x")
        .or_else(|| calldata_in.strip_prefix("0X"))
        .unwrap_or(calldata_in)
}

fn validate_calldata_length(calldata: &str) -> Result<()> {
    if calldata.len() < 8 {
        return Err(anyhow!(
            "Invalid calldata length: expected at least a 4-byte (8 hex char) function selector."
        ));
    }

    let body_len = calldata.len() - 8;
    if body_len % 64 != 0 {
        return Err(anyhow!(
            "Invalid calldata length: expected a 4-byte function selector followed by zero or more 32-byte (64 hex char) words."
        ));
    }

    Ok(())
}

fn validate_calldata_hex_characters(calldata: &str) -> Result<()> {
    if !calldata.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow!(
            "Invalid calldata: contains non-hexadecimal characters (expected 0-9, a-f, A-F)."
        ));
    }

    Ok(())
}
