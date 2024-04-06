use super::message::AsBytes;
use super::types::QRIndicator;

#[derive(Debug, Clone)]
pub struct Header {
    pub id: u16,

    // query/response indcicator
    pub qr: QRIndicator,

    // Specifies the kind of query in a message - 4bits
    pub opcode: u8,

    // True if the responding server "owns" the domain queried
    pub authorative_answer: bool,

    // 1 if message is larger than 512bytes. Always 0 in UDP responses
    pub truncation: bool,

    // Sender sets this to 1 if the server should recursively resolve this query, 0 otherwise
    pub recursion_desired: bool,

    // Server sets this to 1 indicate that recursion is availabl(e
    pub recursion_available: bool,

    // Used by DNSSEC queries. At inception, it was reserved for future use - 3bitss
    pub reserved: u8,

    pub response_code: u8,

    pub question_count: u16,

    pub answer_record_count: u16,

    pub authority_record_count: u16,

    pub additional_record_count: u16,
}

impl AsBytes for Header {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            (self.id >> 8) as u8,
            (self.id & 0xFF) as u8,
            (self.qr as u8) << 7
                | (self.opcode & 0b1111) << 3
                | (self.authorative_answer as u8) << 2
                | (self.truncation as u8) << 1
                | (self.recursion_desired as u8),
            ((self.recursion_available as u8) << 7)
                | (self.reserved as u8 & 0b111) << 4
                | (self.response_code as u8 & 0b1111),
            (self.question_count >> 8) as u8,
            (self.question_count & 0xFF) as u8,
            (self.answer_record_count >> 8) as u8,
            (self.answer_record_count & 0xFF) as u8,
            (self.authority_record_count >> 8) as u8,
            (self.authority_record_count & 0xFF) as u8,
            (self.additional_record_count >> 8) as u8,
            (self.additional_record_count & 0xFF) as u8,
        ]
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            id: 1234,
            qr: QRIndicator::Response,
            opcode: 0,
            authorative_answer: false,
            truncation: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: 0,
            response_code: 0,
            question_count: 1,
            answer_record_count: 1,
            authority_record_count: 0,
            additional_record_count: 0,
        }
    }
}

impl Header {
    pub fn parse(buf: &[u8]) -> Self {
        Header {
            id: ((buf[0] as u16) << 8 | buf[1] as u16),
            qr: QRIndicator::from_uint(buf[2] >> 7),
            opcode: (buf[2] >> 3) & 0xF,
            authorative_answer: (buf[2] >> 2) & 0b1 == 1,
            truncation: (buf[2] >> 1) & 0b1 == 1,
            recursion_desired: buf[2] & 0b1 == 1,
            recursion_available: (buf[3] >> 7 & 0b1) == 1,
            reserved: (buf[3] >> 4 & 0b111),
            response_code: (buf[3] & 0xF),
            question_count: ((buf[4] as u16) << 8 | (buf[5] as u16)),
            answer_record_count: ((buf[6] as u16) << 8 | (buf[7] as u16)),
            authority_record_count: ((buf[8] as u16) << 8 | (buf[9] as u16)),
            additional_record_count: ((buf[10] as u16) << 8 | (buf[11] as u16)),
        }
    }
}
