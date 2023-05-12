pub struct HTML {
    pub name: String,
    pub head: Option<Head>,
    pub body: Option<Body>,
}

pub struct Head {
    pub child: Vec<HeadTag>,
}

pub struct Body {
    pub child: Vec<BodyTag>,
}

#[derive(Debug)]
pub enum Token {
    OpeningTag(String),
    ClosingTag(String),
    Data(String),
}

pub enum HeadTag {
    Title,
    Meta,
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

pub fn tokenize(lexemes: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for lexeme in lexemes {
        if lexeme.as_bytes()[0] == '<' as u8 && lexeme.as_bytes()[lexeme.len() - 1] == '>' as u8 {
            if lexeme.as_bytes()[1] == '/' as u8 {
                tokens.push(Token::ClosingTag(
                    (&lexeme[2..lexeme.len() - 1]).to_string(),
                ))
            } else {
                tokens.push(Token::OpeningTag(
                    (&lexeme[1..lexeme.len() - 1]).to_string(),
                ))
            }
        } else {
            tokens.push(Token::Data(lexeme));
        }
    }

    return tokens;
}

impl HTML {
    pub fn new(tokens: Vec<String>) -> HTML {
        HTML {
            name: "DOM".to_string(),
            head: None,
            body: None,
        }
    }
}
