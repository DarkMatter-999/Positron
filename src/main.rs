use parser::boxmodel::layout_tree;
use parser::css::parse_css;
use parser::datatypes::Node;
use parser::display::build_display_list;
use parser::dom::parse_element;
use parser::print_token;
use parser::styles::style_tree;
use parser::tokenizer::Tokenizer;
use parser::window::make_window;

const width: u32 = 800;
const height: u32 = 600;

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

        <h1> Helloooo </h1>
    </body>
</html>
";
    let css = "
        * {
            display: block;
            background: #000000;
        }
        head {
            display: none;
        }
        body {
            margin: 10px;
            background: #FFFFFF;
        }
        p {
            margin: 10px;
            font-size: 48px;
            padding: 20px;
            color: #FFFFFF;
            border-width: 10px;
            border-color: #FF77FF;
        }
        .c1 {
            padding: 0px;
            background: #FF0000;
        }
        .c2 {
            border-width: 0px;
            background: #00FF00;
        }
        .c3 {
            width: 100px;
            background: #00FFFF;
        }
        .c4 {
            background: #0000FF;
            color: #000000;
        }
        h1 {
            padding: 50px;
            color: #000000;
            background: #FF7700;
            margin: 20px;
            border-width: 40px;
            border-color: #FF0000;
        }
        ";
    let mut tokenizer = Tokenizer::new(html);
    // print_token(&mut tokenizer);

    let mut tokenizer = Tokenizer::new(html);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }
    // println!("{:?}", tokens);

    let (element, _) = parse_element(&tokens);
    // println!("{:#?}", element);

    let stylesheet = parse_css(css.to_string());

    // println!("{:#?}", stylesheet);

    let node = Node::Element(element);
    let style_tree = style_tree(&node, &stylesheet);
    // println!("{:#?}", style_tree);

    let layout = layout_tree(width as f32, height as f32, &style_tree);
    // println!("{:#?}", layout);

    let displaylist = build_display_list(&layout);

    // println!("{:#?}", displaylist);

    make_window(width, height, displaylist);
}
