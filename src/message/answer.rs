#![allow(dead_code)]

use super::message::AsBytes;
use super::types::{QClass, QType};
use anyhow::{Result, Context, anyhow};
use log::debug;

#[derive(Debug)]
pub(crate) struct Answer {
    pub name: String,
    pub answer_type: QType,
    pub class: QClass,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl AsBytes for Answer {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for token in self.name.split(".") {
            let token_bytes = token.as_bytes();
            bytes.push(token_bytes.len() as u8);
            bytes.extend_from_slice(token_bytes);
        }

        bytes.push(0);
        bytes.extend_from_slice(&self.answer_type.as_u16().to_be_bytes());
        bytes.extend_from_slice(&self.class.as_u16().to_be_bytes());
        bytes.extend_from_slice(&self.ttl.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.data);

        bytes
    }
}

impl Default for Answer {
    fn default() -> Self {
        Answer {
            name: "codecrafters.io".to_string(),
            answer_type: QType::A,
            class: QClass::IN,
            ttl: 60,
            length: 4,
            data: vec![8, 8, 8, 8],
        }
    }
}

impl Answer {
   pub fn parse(buf: &[u8], start_pos: usize, a_count: u16) -> Result<Vec<Self>> {
        let mut answers = vec![];
        let mut pos = start_pos;

        for _ in 0..a_count {
            let (name, new_pos) = Self::parse_label(buf, Some(pos))?;
            pos = new_pos;

            let answer_type = (buf[pos] as u16) << 8 | buf[pos + 1] as u16;
            let class = (buf[pos + 2] as u16) << 8 | buf[pos + 3] as u16;
            let ttl = (buf[pos + 4] as u32) << 24
                    | (buf[pos + 5] as u32) << 16
                    | (buf[pos + 6] as u32) << 8
                    | buf[pos + 7] as u32;
            let length = (buf[pos + 8] as u16) << 8 | buf[pos + 9] as u16;

            pos += 10;

            let data = buf[pos..pos + length as usize].to_vec();
            pos += length as usize;

            answers.push(Answer {
                name,
                answer_type: QType::from_u16(answer_type)
                    .context("Answer QType should always be valid.")?,
                class: QClass::from_u16(class).context("Answer QClass should always be valid.")?,
                ttl,
                length,
                data,
            });
        }

        Ok(answers)
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