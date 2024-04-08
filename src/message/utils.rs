use anyhow::{anyhow, Context, Result};
use log::debug;

pub trait LabelDecompression {
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
