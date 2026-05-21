use base64::Engine;

fn rot13(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                ((c as u8 - first + 13) % 26 + first) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn encode_str(args: &[String]) -> anyhow::Result<String> {
    let engine = base64::engine::general_purpose::STANDARD;
    let algorithm = args
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing algorithm argument"))?;
    let input = args
        .get(1..)
        .ok_or_else(|| anyhow::anyhow!("Missing input argument"))?
        .join(" ");

    match algorithm.to_lowercase().as_str() {
        "b64" => Ok(Engine::encode(&engine, input)),
        "hex" => Ok(hex::encode(input)),
        "rot13" => Ok(rot13(&input)),
        _ => Err(anyhow::anyhow!("Unsupported encoding algorithm: {}", algorithm)),
    }
}

pub fn decode_str(args: &[String]) -> anyhow::Result<String> {
    let engine = base64::engine::general_purpose::STANDARD;
    let algorithm = args
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing algorithm argument"))?;
    let input = args
        .get(1..)
        .ok_or_else(|| anyhow::anyhow!("Missing input argument"))?
        .join(" ");

    match algorithm.to_lowercase().as_str() {
        "b64" => Ok(String::from_utf8(Engine::decode(&engine, input)?)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8 after decoding: {}", e))?),
        "hex" => Ok(String::from_utf8(hex::decode(input)?)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8 after decoding: {}", e))?),
        "rot13" => Ok(rot13(&input)),
        _ => Err(anyhow::anyhow!("Unsupported decoding algorithm: {}", algorithm)),
    }
}