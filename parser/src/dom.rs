use std::{collections::HashMap, println};

use crate::{datatypes::AttrMap, Element, Node, Token};

pub fn parse_element(tokens: &[Token]) -> (Element, &[Token]) {
    let mut element = Element::new("".to_string());

    if let Some(Token::OpeningTag(tag_name)) = tokens.first() {
        match tag_name.split(' ').next().clone() {
            Some(s) => element.name = s.to_string(),
            None => {
                panic!("No tagname");
            }
        }

        for pair in tag_name.split_whitespace() {
            let mut keyval = pair.split("=");
            if let (Some(key), Some(value)) = (keyval.next(), keyval.next()) {
                element
                    .attributes
                    .insert(key.to_string(), value.to_string());
            }
        }
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
