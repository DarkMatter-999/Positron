use lexer;

fn main() {
    let strr = "
<html>
    <head>
        <title>Title</title>
    </head>
    <body>
    </body>
</html>
*/";

    let dom = lexer::HTML::new("DOM".to_string(), strr.to_string()) ;
    
}
