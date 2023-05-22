use std::println;

use parser::css::parse_css;
use parser::dom::parse_element;
use parser::print_token;
use parser::tokenizer::Tokenizer;

fn main() {
    let html = "
<html>
    <head prop=test prop2=val2>
        <title>TitleData</title>
    </head>
    <body>
        <p>This is some text</p>
    </body>
</html>
";
    let css = "
        * {
            heh: #101010; 
        }
        .body {
            test: 1px;
            prop2: #ff7700;
        }
        #test { prop1: 1px; }
        p {
            prop3: 1000px;
        }
        ";
    let mut tokenizer = Tokenizer::new(html);
    print_token(&mut tokenizer);

    let mut tokenizer = Tokenizer::new(html);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);

    let (element, _) = parse_element(&tokens);
    println!("{:#?}", element);

    let mut stylesheet = parse_css(css.to_string());

    println!("{:#?}", stylesheet);
}
