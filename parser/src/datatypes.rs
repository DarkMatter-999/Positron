#[derive(Debug, Clone)]
pub enum Token {
    OpeningTag(String),
    ClosingTag(String),
    Data(String),
}

pub enum HeadTag {
    Title,
    Meta,
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

#[derive(Debug)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug)]
pub struct Element {
    pub name: String,
    children: Vec<Node>,
}

impl Element {
    pub fn new(name: String) -> Element {
        Element {
            name: name,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn print(&self, indent: usize) {
        println!("{:indent$}{}", "", self.name, indent = indent);
        for child in &self.children {
            match child {
                Node::Element(element) => element.print(indent + 2),
                Node::Text(text) => println!("{:indent$}{}", "", text, indent = indent + 2),
            }
        }
    }
}
