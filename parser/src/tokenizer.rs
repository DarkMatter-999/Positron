use crate::datatypes::Token;
use std::str::Chars;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Tokenizer {
            chars,
            current_char,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            if c == '<' {
                self.advance();
                if let Some(c) = self.current_char {
                    if c == '/' {
                        self.advance();
                        return Some(Token::ClosingTag(self.read_tag_name()));
                    } else {
                        return Some(Token::OpeningTag(self.read_tag_name()));
                    }
                }
            } else {
                return Some(Token::Data(self.read_text()));
            }
        }

        None
    }

    pub fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    fn read_tag_name(&mut self) -> String {
        let mut tag_name = String::new();

        while let Some(c) = self.current_char {
            if c == '>' {
                self.advance();
                break;
            } else {
                tag_name.push(c);
                self.advance();
            }
        }

        tag_name
    }

    fn read_text(&mut self) -> String {
        let mut text = String::new();

        while let Some(c) = self.current_char {
            if c == '<' {
                break;
            } else {
                text.push(c);
                self.advance();
            }
        }

        text
    }
}
