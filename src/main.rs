use lexer;

fn main() {
    let strr = "
<html>
    <head>
        <title>TitleData</title>
    </head>
    <body>
    </body>
</html>
*/";

    let dom = lexer::HTML::new("DOM".to_string(), strr.to_string()) ;
    
}
