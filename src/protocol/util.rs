pub fn encode_domain(mut name: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if let Some(pstrip) = name.strip_prefix(".") {
        name = pstrip.to_string();
    }

    if let Some(sstrip) = name.strip_suffix(".") {
        name = sstrip.to_string();
    }

    let mut buf = Vec::new();
    for part in name.split('.') {
        if part.len() == 0 {
            return Err("Invalid domain name".into());
        }
        buf.push(part.len() as u8);
        buf.extend(part.as_bytes());
    }
    buf.push(0);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_domain() {
        let encoded = encode_domain("example.com".to_string());
        assert!(encoded.is_ok());
        assert_eq!(
            encoded.unwrap(),
            vec![7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0]
        );

        let encoded = encode_domain("example.com.".to_string());
        assert!(encoded.is_ok());
        assert_eq!(
            encoded.unwrap(),
            vec![7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0]
        );

        let encoded = encode_domain(".example.com".to_string());
        assert!(encoded.is_ok());
        assert_eq!(
            encoded.unwrap(),
            vec![7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0]
        );

        let encoded = encode_domain("example..com".to_string());
        assert!(encoded.is_err());
    }
}
