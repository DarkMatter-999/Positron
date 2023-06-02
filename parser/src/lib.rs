pub mod boxmodel;
pub mod css;
pub mod datatypes;
pub mod display;
pub mod dom;
pub mod styles;
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
