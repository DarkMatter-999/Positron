pub struct HTML {
    pub name: String,
    pub head: Option<Head>,
    pub body: Option<Body>,
}

pub struct Head {
    pub child: Vec<HeadTag>
}

pub struct Body {
    pub child: Vec<BodyTag>
}

pub enum HeadTag {
    Title,
    Meta
}

pub enum BodyTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
}

pub fn parseTags(code: String) -> Vec<String> {
    let mut tags = Vec::new();
    let mut it = code.chars().peekable();

    let mut s = String::new();
    let mut flag = true;
    while let Some(&c) = it.peek() {
        match c {
        
            '<' => {
                it.next();
                s = String::new();
                flag = false;
            },
            'a'..='z' => {
                it.next();
                s += &c.to_string();
            },
            '>' => {
                it.next();
                flag = true;
                tags.push(s.clone());
            }
            _ => {it.next();}
        }
    }

    return tags;
}

impl HTML {
    pub fn new(name: String, html: String) -> HTML {
        println!("{:?}", parseTags(html));
        Self {
            name,
            head: None,
            body: None
        }
    }
}

//impl Head {
    
//}

