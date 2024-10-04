use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone)]
pub enum Payload {
    Raw(Vec<u8>),
    Regex(String),
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub payload: Payload,
}

pub fn parse_signatures(file_path: &str) -> Result<Vec<Signature>> {
    let file = File::open(file_path).context("Failed to open signatures file")?;
    let reader = BufReader::new(file);
    let mut signatures = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.context("Failed to read line from signatures file")?;
        if line.trim().is_empty() {
            continue;
        }

        let payload = if line.contains('(') && line.contains(')') {
            Payload::Regex(line)
        } else {
            Payload::Raw(unescape_string(&line)
                .with_context(|| format!("Invalid payload on line {}", index + 1))?)
        };

        signatures.push(Signature { payload });
    }

    if signatures.is_empty() {
        return Err(anyhow!("No valid signatures found in the file"));
    }

    Ok(signatures)
}

fn unescape_string(s: &str) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('x') => {
                    let hex = chars.next().and_then(|c1| {
                        chars.next().map(|c2| format!("{}{}", c1, c2))
                    }).unwrap_or_else(|| {
                        result.push(b'\\');
                        result.push(b'x');
                        String::new()
                    });
                    if !hex.is_empty() {
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            result.push(byte);
                        } else {
                            result.push(b'\\');
                            result.push(b'x');
                            result.extend(hex.bytes());
                        }
                    }
                },
                Some('0') => result.push(0),
                Some('n') => result.push(b'\n'),
                Some('r') => result.push(b'\r'),
                Some('t') => result.push(b'\t'),
                Some(c) => result.push(c as u8),
                None => result.push(b'\\'),
            }
        } else {
            result.push(c as u8);
        }
    }

    Ok(result)
}

pub fn generate_payload(signature: &Signature) -> Vec<u8> {
    match &signature.payload {
        Payload::Raw(v) => v.clone(),
        Payload::Regex(r) => generate_regex_match(r),
    }
}

fn generate_regex_match(regex_str: &str) -> Vec<u8> {
    let mut result = String::new();
    let mut chars = regex_str.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next_char) = chars.next() {
                    match next_char {
                        'd' => result.push(rand::thread_rng().gen_range(b'0'..=b'9') as char),
                        'w' => result.push(rand::thread_rng().gen_range(b'a'..=b'z') as char),
                        'x' => {
                            let hex = chars.next().and_then(|c1| chars.next().map(|c2| format!("{}{}", c1, c2)))
                                .unwrap_or_else(|| "00".to_string());
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                result.push(byte as char);
                            }
                        }
                        _ => result.push(next_char),
                    }
                }
            },
            '[' => {
                let mut class = String::new();
                for class_char in chars.by_ref() {  
                    if class_char == ']' { break; }
                    class.push(class_char);
                }
                if !class.is_empty() {
                    result.push(class.chars().nth(rand::thread_rng().gen_range(0..class.len())).unwrap());
                }
            },
            '(' => {
                let mut depth = 1;
                for group_char in chars.by_ref() { 
                    if group_char == '(' { depth += 1; }
                    if group_char == ')' { depth -= 1; }
                    if depth == 0 { break; }
                }
            },
            '+' | '*' => {
                if let Some(last_char) = result.chars().last() {
                    let repeat = rand::thread_rng().gen_range(0..5);
                    for _ in 0..repeat {
                        result.push(last_char);
                    }
                }
            },
            '.' => result.push(rand::thread_rng().gen_range(b'!'..=b'~') as char),
            _ => result.push(c),
        }
    }

    result.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_regex_match() {
        let regex_str = r"Hello [\w]+, your lucky number is \d+";
        let result = generate_regex_match(regex_str);
        let result_str = String::from_utf8_lossy(&result);
        assert!(result_str.starts_with("Hello "));
        assert!(result_str.contains(", your lucky number is "));
    }

    #[test]
    fn test_unescape_string() {
        assert_eq!(unescape_string(r"Hello\nWorld").unwrap(), b"Hello\nWorld");
        assert_eq!(unescape_string(r"Test\x41\x42\x43").unwrap(), b"TestABC");
        assert_eq!(unescape_string(r"\0\r\n\t").unwrap(), b"\0\r\n\t");
        assert_eq!(unescape_string(r"Incomplete\").unwrap(), b"Incomplete\\");
        assert_eq!(unescape_string(r"Incomplete\x").unwrap(), b"Incomplete\\x");
        assert_eq!(unescape_string(r"Incomplete\x4").unwrap(), b"Incomplete\\x4");
    }
}
