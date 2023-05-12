pub fn parseTags(code: String) -> Vec<String> {
    let mut tags = Vec::new();
    let mut it = code.chars().peekable();

    let mut s = String::new();
    while let Some(&c) = it.peek() {
        match c {
            '<' => {
                if s.len() > 0 {
                    tags.push(s.clone());
                    s = String::new();
                }
                it.next();
                s += &c.to_string();
            },
            'a'..='z' => {
                it.next();
                s += &c.to_string();
            },
            'A'..='Z' => {
                it.next();
                s += &c.to_string();
            }
            '>' => { 
                it.next();
                s += &c.to_string();
                tags.push(s.clone());

                s = String::new();
            },
            '/' => {
                it.next();
                s += &c.to_string();
            },
            '\n' | ' ' | '\r' | '\t' => {
                it.next();
            }
            _ => {
                it.next();
                s += &c.to_string();
            }

        }
    }

    return tags;
}

