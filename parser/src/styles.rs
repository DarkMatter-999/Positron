use std::collections::HashMap;

use crate::{
    css::{Selector, SimpleSelector, Value},
    datatypes::{Element, Node},
};

type PropertyMap = HashMap<String, Value>;

struct StyleNode<'a> {
    node: &'a Node,
    style_values: PropertyMap,
    children: Vec<StyleNode<'a>>,
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
