use crate::{
    boxmodel::{BoxType, LayoutBox, Rect},
    css::{Color, Value},
    datatypes::Node,
};

type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    Text(Color, Rect, String),
}

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    render_text(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

// Return the specified color for CSS property `name`, or None if no color was specified.
fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => match style.value(name) {
            Some(Value::Color(color)) => Some(color),
            _ => None,
        },
        BoxType::AnonymousBlock => None,
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color| {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ))
    });
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return, // bail out if no border-color is specified
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(
        color.clone(),
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    // Right border
    list.push(DisplayCommand::SolidColor(
        color.clone(),
        Rect {
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    // Top border
    list.push(DisplayCommand::SolidColor(
        color.clone(),
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    // Bottom border
    list.push(DisplayCommand::SolidColor(
        color.clone(),
        Rect {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ));
}

fn render_text(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "color") {
        Some(color) => color,
        _ => return, // bail out if no color is specified
    };

    let text = match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => match style.children.first() {
            s => match s {
                Some(st) => match st.node {
                    Node::Text(t) => Some(t),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        BoxType::AnonymousBlock => None,
    };

    match text {
        Some(t) => {
            list.push(DisplayCommand::Text(
                color,
                layout_box.dimensions.border_box(),
                t.to_string(),
            ));
        }
        None => (),
    }
}
