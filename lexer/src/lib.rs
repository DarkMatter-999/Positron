pub struct HTML {
    pub name: String,
    pub head: Option<Head>,
    pub body: Option<Body>,
}

pub struct Head {
    pub child: Vec<HeadTag>
}

pub struct Body {
    pub child: Vec<BodyTag>
}

#[derive(Debug)]
pub enum Token {
    OpeningTag(String),
    ClosingTag(String),
    Data(String),
}

pub enum HeadTag {
    Title,
    Meta
}

pub enum BodyTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
}

#[derive(Copy, Clone,PartialEq)]
enum State {
    Opening,
    Closing,
    Text,
}

pub fn parseTags(code: String) -> Vec<String> {
    let mut tags = Vec::new();
    let mut it = code.chars().peekable();

    let mut s = String::new();
    while let Some(&c) = it.peek() {
        match c {
            '<' => {
                if s.len() > 0 {
                    tags.push(s.clone());
                    s = String::new();
                }
                it.next();
                s += &c.to_string();
            },
            'a'..='z' => {
                it.next();
                s += &c.to_string();
            },
            'A'..='Z' => {
                it.next();
                s += &c.to_string();
            }
            '>' => { 
                it.next();
                s += &c.to_string();
                tags.push(s.clone());

                s = String::new();
            },
            '/' => {
                it.next();
                s += &c.to_string();
            },
            '\n' | ' ' | '\r' | '\t' => {
                it.next();
            }
            _ => {
                it.next();
                s += &c.to_string();
            }

        }
    }

    return tags;
}

impl HTML {
    pub fn new(name: String, html: String) -> HTML {
        println!("{:?}", parseTags(html));
        Self {
            name,
            head: None,
            body: None
        }
    }
}

//impl Head {
    
//}

