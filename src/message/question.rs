#![allow(dead_code)]

use super::message::AsBytes;
use super::types::{QClass, QType};
use anyhow::{anyhow, Context, Result};
use log::{info, debug};

#[derive(Debug, Clone)]
pub(crate) struct Question {
    pub name: String,
    pub question_type: QType,
    pub class: QClass,
}

impl AsBytes for Question {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for token in self.name.split(".") {
            let token_bytes = token.as_bytes();
            bytes.push(token_bytes.len() as u8);
            bytes.extend_from_slice(token_bytes);
        }
        bytes.push(0);
        bytes.extend_from_slice(&self.question_type.as_u16().to_be_bytes());
        bytes.extend_from_slice(&self.class.as_u16().to_be_bytes());

        bytes
    }
}

impl Default for Question {
    fn default() -> Self {
        Question {
            name: "codecrafters.io".to_string(),
            class: QClass::IN,
            question_type: QType::A,
        }
    }
}

impl Question {
    pub fn parse(buf: &[u8], q_count: u16) -> Result<(Vec<Self>, usize)> {
        info!("Parsing questions, count: {}", q_count);
        let mut res = vec![];
        let mut pos = 12;

        for _ in 0..q_count {
            let name: String;
            debug!("Parsing question at position: {}", pos);
            (name, pos) = Self::parse_label(buf, Some(pos))?;
            debug!("Parsed label: '{}', new position: {}", name, pos);

            debug!("Remaining bytes: '{:?}'", &buf[pos..]);
            debug!("Current pos before type/class: {}", pos);
            let question_type = (buf[pos] as u16) << 8 | buf[pos + 1] as u16;
            let class = ((buf[pos + 2]) as u16) << 8 | buf[pos + 3] as u16;
            pos += 4;

            debug!("Parsed question_type: {}, class: {}", question_type, class);

            res.push(Self {
                name,
                question_type: QType::from_u16(question_type)
                    .context("QType should always be valid.")?,
                class: QClass::from_u16(class).context("QClass should always be valid.")?,
            });
        }
        info!("Finished parsing questions");
        return Ok((res, pos));
    }

    fn parse_label(buf: &[u8], pos: Option<usize>) -> Result<(String, usize)> {
        let mut name = String::new();
        let mut pos = pos.unwrap_or(12);
        let mut compression: bool = false;
        debug!("Starting parse_label at position: {}", pos);

        while pos < buf.len() && buf[pos] != 0 {
            debug!("At position {}: buf[{}] = {:02X}", pos, pos, buf[pos]);

            if !name.is_empty() {
                name.push('.');
            }

            compression = (buf[pos] >> 6) & 0b11 == 0b11;
            debug!("Compression flag: {}", compression);

            if compression {
                let compression_pointer = ((buf[pos] & 0x3F) as u16) << 8 | buf[pos + 1] as u16;
                pos += 2;
                debug!("Following compression pointer to: {}", compression_pointer);
                let (label, _) = Self::parse_label(buf, Some(compression_pointer as usize))?;
                name.push_str(&label);
            } else {
                let len = buf[pos] as usize;
                pos += 1;
                debug!("Label length: {}", len);
                if pos + len > buf.len() {
                    return Err(anyhow!("Label extends beyond buffer length"));
                }
                let label = std::str::from_utf8(&buf[pos..pos + len])
                    .context("Should be utf-8 encoded.")?
                    .to_string();
                pos += len;
                name.push_str(&label);
                debug!("Parsed label: '{}'", label);
            }
        }
        if !compression {
            pos += 1;
        }
        debug!(
            "Completed label parsing with name: '{}', next position: {}",
            name, pos
        );
        Ok((name, pos))
    }
}
