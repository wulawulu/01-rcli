use crate::{get_reader, Base64Format};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    // avoid accidental newlines
    let buffer = buffer.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
    };
    // TODO: decoded data might not be string
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/tmp.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
