use std::println;

use parser::boxmodel::{build_layout_tree, layout_tree};
use parser::css::parse_css;
use parser::datatypes::Node;
use parser::display::{build_display_list, DisplayCommand};
use parser::dom::parse_element;
use parser::print_token;
use parser::styles::style_tree;
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
            display: block;
            background-color: #FFFFFF;
            color: #000000;
        }
        body {
            margin: 10px;
            padding: 20px;
            background: #FFFFFF;
        }
        p {
            margin:100px;
            padding: 50px;
            border: 20px;
            border-color: #000000;
            background: #FF00FF;
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

    let mut node = Node::Element(element);
    let mut style_tree = style_tree(&node, &stylesheet);
    println!("{:#?}", style_tree);

    let mut layout = layout_tree(&style_tree);
    println!("{:#?}", layout);

    let mut displaylist = build_display_list(&layout);

    println!("{:#?}", displaylist);
}
