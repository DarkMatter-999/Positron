use std::println;

use parser::tokenizer::Tokenizer;
use parser::{parse_element, print_token};

fn main() {
    let strr = "
<html>
    <head>
        <title>TitleData</title>
    </head>
    <body>
        <p>This is some text</p>
    </body>
</html>
";

    let mut tokenizer = Tokenizer::new(strr);
    print_token(&mut tokenizer);

    let mut tokenizer = Tokenizer::new(strr);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);

    let (element, _) = parse_element(&tokens);
    println!("{:#?}", element);
}
