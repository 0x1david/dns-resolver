use super::message::AsBytes;
use super::types::{QClass, QType};
use super::utils::LabelDecompression;
use anyhow::{Context, Result};
use log::{debug, info};

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

impl LabelDecompression for Question {}
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
}
