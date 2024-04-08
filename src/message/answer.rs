use super::message::AsBytes;
use super::types::{QClass, QType};
use super::utils::LabelDecompression;
use anyhow::{Context, Result};

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

impl LabelDecompression for Answer {}
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
}
