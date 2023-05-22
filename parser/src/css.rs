use std::{println, str::Chars};

#[derive(Debug)]
pub struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    selector: Vec<Selector>,
    declaration: Vec<Declaration>,
}

#[derive(Debug)]
enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    tag: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

#[derive(Debug)]
enum Unit {
    Px,
    Em,
    Pr,
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn parse_css(input: String) -> Stylesheet {
    let mut parser = CSS::new(input);

    Stylesheet {
        rules: parser.parse_rules(),
    }
}

pub struct CSS {
    pos: usize,
    chars: String,
}

impl CSS {
    pub fn new(input: String) -> Self {
        let mut chars = input;
        CSS {
            pos: 0,
            chars: chars,
        }
    }

    fn next_char(&mut self) -> char {
        self.chars[self.pos..].chars().next().unwrap()
    }

    pub fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {
            self.consume_whitespace();

            match self.next_char() {
                '#' => {
                    self.advance();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.advance();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.advance();
                }
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
                    selector.tag = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        return selector;
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
    fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while !self.eof() {
            self.consume_whitespace();
            match self.next_char() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
                    identifier.push(self.next_char());
                    self.advance();
                }
                _ => break,
            }
        }
        identifier
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }
        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selector: self.parse_selectors(),
            declaration: self.parse_declarations(),
        }
    }
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.advance();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        if self.next_char() != '{' {
            panic!("Errr");
        }
        self.advance();
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.advance();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert_eq!(self.next_char(), ':');
        self.advance();
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert_eq!(self.next_char(), ';');
        self.advance();

        Declaration {
            name: property_name,
            value: value,
        }
    }

    fn consume_whitespace(&mut self) {
        while !self.eof() {
            match self.next_char() {
                s => {
                    if s.is_whitespace() {
                        self.advance()
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let mut s = String::new();
        while !self.eof() {
            match self.next_char() {
                '0'..='9' | '.' => {
                    s.push(self.next_char());
                    self.advance();
                }
                _ => break,
            }
        }
        s.parse().unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit"),
        }
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.next_char(), '#');
        self.advance();
        Value::Color(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        })
    }

    /// Parse two hexadecimal digits.
    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.chars[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }
    fn eof(&self) -> bool {
        self.pos >= self.chars.len()
    }
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag.iter().count();
        (a, b, c)
    }
}
