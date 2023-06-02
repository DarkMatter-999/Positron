use std::println;

use parser::boxmodel::{build_layout_tree, layout_tree};
use parser::css::parse_css;
use parser::datatypes::Node;
use parser::display::{build_display_list, DisplayCommand};
use parser::dom::parse_element;
use parser::print_token;
use parser::styles::style_tree;
use parser::tokenizer::Tokenizer;
use parser::window::make_window;

fn main() {
    let html = "
<html>
    <head prop=test prop2=val2>
        <title>TitleData</title>
    </head>
    <body>
        <p>This is some text</p>
        <p class=c1>one</p>
        <p class=c2>two</p>
        <p class=c3>three</p>
        <p class=c4>four</p>
    </body>
</html>
";
    let css = "
        * {
            display: block;
            background: #000000;
        }
        body {
            margin: 10px;
            background: #FFFFFF;
        }
        p {
            padding: 10px;
            margin: 10px;
        }
        .c1 {
            background: #FF0000;
        }
        .c2 {
            background: #00FF00;
        }
        .c3 {
            background: #00FFFF;
        }
        .c4 {
            background: #0000FF;
            color: #000000;
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

    make_window(displaylist);
}
