use std::collections::{HashMap, HashSet};

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

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attributes: AttrMap,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(name: String) -> Element {
        Element {
            name: name,
            children: Vec::new(),
            attributes: HashMap::new(),
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

    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}
