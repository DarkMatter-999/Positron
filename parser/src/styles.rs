use std::collections::HashMap;

use crate::{
    css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value},
    datatypes::{Element, Node},
};

type PropertyMap = HashMap<String, Value>;

type MatchedRule<'a> = (Specificity, &'a Rule);

#[derive(Debug)]
pub struct StyleNode<'a> {
    node: &'a Node,
    pub style_values: PropertyMap,
    pub children: Vec<StyleNode<'a>>,
}

pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyleNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.style_values.get(name).cloned()
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

fn matches(elem: &Element, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_sel) => match_simple_selector(elem, simple_sel),
    }
}

fn match_simple_selector(elem: &Element, selector: &SimpleSelector) -> bool {
    if selector.tag.iter().any(|name| elem.name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    return true;
}

fn match_rule<'a>(elem: &Element, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selector
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(elem: &Element, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn specified_values(elem: &Element, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    for (_, rule) in rules {
        for declaration in &rule.declaration {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    return values;
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyleNode<'a> {
    StyleNode {
        node: root,
        style_values: match root {
            Node::Element(ref elem) => specified_values(elem, stylesheet),
            Node::Text(_) => HashMap::new(),
        },
        children: if let Node::Element(ref elem) = root {
            elem.children
                .iter()
                .map(|child| style_tree(child, stylesheet))
                .collect()
        } else {
            Vec::new()
        },
    }
}
