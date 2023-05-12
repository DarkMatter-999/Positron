use lexer;
use parser;

fn main() {
    let strr = "
<html>
    <head>
        <title>TitleData</title>
    </head>
    <body>
    </body>
</html>
";

    let lexemes = lexer::parseTags(strr.to_string());
    println!("{:?}", lexemes);

    let tokens = parser::tokenize(lexemes);
    println!("{:?}", tokens);
}
