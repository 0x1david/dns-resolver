use super::answer::Answer;
use super::header::Header;
use super::question::Question;
use crate::message::types::{QClass, QType};
use log::debug;

pub(crate) trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub(crate) struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answer: Vec<Answer>,
}

impl AsBytes for Message {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.as_bytes();
        self.questions.iter().for_each(|question| {
            bytes.extend(question.as_bytes());
        });
        self.answer.iter().for_each(|answer| {
            bytes.extend(answer.as_bytes());
        });
        bytes
    }
}

impl Default for Message {
    fn default() -> Self {
        Message {
            questions: vec![Question::default()],
            answer: vec![Answer::default()],
            header: Header::default(),
        }
    }
}

impl Message {
    pub fn parse_request(buf: &[u8]) -> Self {
        let header = Header::parse(&buf);
        debug!("Parsed request header: {:?}", header);

        let (questions, _) =
            Question::parse(&buf, header.question_count).expect("Parsing Should Succeed");
        debug!("Parsed question(s): {:?}", questions);

        Self {
            header,
            questions,
            answer: vec![],
        }
    }
    pub fn parse_resolver_response(buf: &[u8]) -> Self {
        let header = Header::parse(&buf);
        debug!("Parsed request header: {:?}", header);

        let (questions, pos) =
            Question::parse(&buf, header.question_count).expect("Parsing Should Succeed");
        debug!("Parsed question(s): {:?}", questions);

        let answers =
            Answer::parse(&buf, pos, header.question_count).expect("Parsing Should Succeed");
        debug!("Parsed answer(s): {:?}", answers);

        Self {
            header,
            questions,
            answer: answers,
        }
    }

    pub fn create_answerless_response(&self) -> Self {
        let mut header = Header::default();

        header.id = self.header.id;
        header.opcode = self.header.opcode;
        header.recursion_desired = self.header.recursion_desired;
        header.question_count = self.header.question_count;
        header.answer_record_count = self.header.question_count;
        header.response_code = match self.header.opcode {
            0 => 0,
            _ => 4,
        };
        debug!("Updated response header: {:?}", header);

        let response = Message {
            header,
            questions: self.questions.to_vec(),
            answer: vec![],
        };
        debug!("Response message prepared: {:?}", response);
        response
    }

    pub fn create_response(&self) -> Self {
        let mut answers: Vec<Answer> = vec![];
        let mut header = Header::default();
        for q in &self.questions {
            let answer = Answer {
                name: q.name.clone(),
                answer_type: QType::A,
                class: QClass::IN,
                ttl: 60,
                length: 4,
                data: vec![192, 168, 0, 1],
            };
            debug!("Generated answer: {:?}", answer);
            answers.push(answer);
        }

        header.id = self.header.id;
        header.opcode = self.header.opcode;
        header.recursion_desired = self.header.recursion_desired;
        header.question_count = self.header.question_count;
        header.answer_record_count = self.header.question_count;
        header.response_code = match self.header.opcode {
            0 => 0,
            _ => 4,
        };
        debug!("Updated response header: {:?}", header);

        let response = Message {
            header,
            questions: self.questions.to_vec(),
            answer: answers,
        };
        debug!("Response message prepared: {:?}", response);
        response
    }
    pub fn split_as_bytes(&self) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = vec![];
        let mut header = self.header.clone();

        header.question_count = 1;
        self.questions.iter().for_each(|question| {
            let mut bytes: Vec<u8> = header.as_bytes();

            bytes.extend(question.as_bytes());

            result.push(bytes)
        });
        result
    }
}
