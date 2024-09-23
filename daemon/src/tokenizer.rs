use std::fs;
use std::io::Read;

pub struct Tokenizer {
    file: Vec<u8>,
    offset: usize,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum TokenType {
    LBR,
    RBR,
    LPAREN,
    RPAREN,
    ARROW,
    RANG,
    LANG,
    MINUS,
    NEWL,
    Value
}

impl Tokenizer {
    pub fn new(filename: &str) -> Tokenizer {
        let mut file = Vec::new();
        fs::File::open(filename).expect("expected valid filename and read permissions").read_to_end(&mut file).expect("failed to read the provided configuration file");

        Tokenizer{
            file,
            offset: 0,
        }
    }

    fn read_token(&mut self) -> Token {
        match self.file[self.offset] {
            b'[' => {
                self.offset += 1;

                Token{token_type: TokenType::LBR, value: "[".to_string()}
            },
            b']' => {
                self.offset += 1;

                Token{token_type: TokenType::RBR, value: "]".to_string()}
            },
            b'(' =>{
                self.offset += 1;

                Token{token_type: TokenType::LPAREN, value: "(".to_string()}
            },
            b')' => {
                self.offset += 1;

                Token{token_type: TokenType::RPAREN, value: ")".to_string()}
            },
            b'-' => {
                self.offset += 1;
                if self.peek_read(b'>').is_some() {
                    Token{token_type: TokenType::ARROW, value: "->".to_string()}
                } else {
                    Token{token_type: TokenType::MINUS, value: "-".to_string()}
                }
            },
            b'>' => {
                self.offset += 1;

                Token{token_type: TokenType::RANG, value: ">".to_string()}
            },
            b'<' => {
                self.offset += 1;

                Token{token_type: TokenType::LANG, value: "<".to_string()}
            },
            b'\n' => {
                self.offset += 1;

                Token{token_type: TokenType::NEWL, value: "\n".to_string()}
            },
            _ => Token{token_type: TokenType::Value, value: self.read_value()},
        }
    }

    fn read_one(&mut self) -> Option<u8> {
        self.skip();

        if self.offset < self.file.len() {
            self.offset += 1;

            Some(self.file[self.offset])
        } else {
            None
        }
    }

    fn peek_read(&mut self, chr: u8) -> Option<u8> {
        if self.offset < self.file.len() && self.file[self.offset] == chr {
            self.offset += 1;

            return Some(self.file[self.offset])
        }

        None
    }

    fn read_value(&mut self) -> String {
        let start = self.offset;

        while let Some(chr) = self.read_one() {
            if !chr.is_ascii_alphanumeric() {
                break
            }
        }

        String::from_utf8(self.file[start..self.offset].to_vec()).expect("should not panic")
    }

    fn skip(&mut self) {
        while self.offset < self.file.len() {
            if self.file[self.offset] == b' ' {
                self.offset += 1;
            } else {
                break
            }
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip();

        if self.offset >= self.file.len() {
            return None
        }

        Some(self.read_token())
    }
}

