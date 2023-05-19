mod datatypes;
pub mod tokenizer;

use datatypes::Element;
use datatypes::Node;
use datatypes::Token;
use tokenizer::Tokenizer;

pub fn print_token(tokenizer: &mut Tokenizer) {
    loop {
        match tokenizer.next_token() {
            Some(Token::OpeningTag(s)) => {
                println!("{:?}", Token::OpeningTag(s));
            }
            Some(Token::ClosingTag(s)) => {
                println!("{:?}", Token::ClosingTag(s));
            }
            Some(Token::Data(d)) => {
                println!("{:?}", Token::Data(d))
            }
            None => break,
        }
    }
}

pub fn parse_element(tokens: &[Token]) -> (Element, &[Token]) {
    let mut element = Element::new("".to_string());

    if let Some(Token::OpeningTag(tag_name)) = tokens.first() {
        element.name = tag_name.clone();
    } else {
        panic!("Expected open tag");
    }

    let mut rest = &tokens[1..];

    while let Some(token) = rest.first() {
        match token {
            Token::OpeningTag(_) => {
                let (child, new_rest) = parse_element(rest);
                element.add_child(Node::Element(child));
                rest = new_rest;
            }
            Token::ClosingTag(tag_name) => {
                if tag_name == &element.name {
                    return (element, &rest[1..]);
                } else {
                    panic!("Mismatched tags");
                }
            }
            Token::Data(text) => {
                element.add_child(Node::Text(text.clone()));
                rest = &rest[1..];
            }
        }
    }

    panic!("Expected close tag");
}
