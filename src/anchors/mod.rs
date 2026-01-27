use markdown::mdast::{AttributeContent, AttributeValue, Heading, Node};
use std::collections::HashMap;

pub fn extract_from_node(node: &Node, anchor_occurrences: &mut HashMap<String, u32>) {
  match node {
    Node::Heading(h) => {
      let anchor = extract_from_heading(h);
      let existing_duplicates = anchor_occurrences.get(&anchor).unwrap_or(&0);
      anchor_occurrences.insert(anchor, *existing_duplicates + 1);
    }
    Node::MdxJsxFlowElement(el) => {
      if let Some(anchor) = extract_from_attributes(&el.attributes) {
        anchor_occurrences.insert(anchor, 1);
      };
    }
    Node::MdxJsxTextElement(el) => {
      if let Some(anchor) = extract_from_attributes(&el.attributes) {
        anchor_occurrences.insert(anchor, 1);
      };
    }
    _ => (),
  };
}

fn extract_from_attributes(attributes: &Vec<AttributeContent>) -> Option<String> {
  for attr in attributes.iter() {
    let AttributeContent::Property(prop) = attr else {
      continue;
    };
    if prop.name != "id" {
      continue;
    };
    if let Some(AttributeValue::Literal(text)) = prop.value.clone() {
      return Some(text);
    }
  }
  return None;
}

fn extract_from_heading(heading: &Heading) -> String {
  let mut text = String::with_capacity(100);
  for child in heading.children.iter() {
    extract_text(child, &mut text);
  }
  heading_to_anchor(text)
}

pub fn extract_text<'a>(node: &'a Node, s: &mut String) {
  let maybe_text = match node {
    Node::Text(text) => Some(&text.value),
    Node::InlineCode(text) => Some(&text.value),
    _ => None,
  };
  if let Some(text) = maybe_text {
    s.push_str(text.as_str())
  };

  if let Some(children) = node.children() {
    for child in children {
      extract_text(child, s);
    }
  }
}

pub fn deduplicate_anchors(anchor_occurrences: HashMap<String, u32>) -> Vec<String> {
  anchor_occurrences
    .into_iter()
    .flat_map(|(anchor, duplications)| {
      (0..duplications).map(move |n| match n {
        0 => format!("#{anchor}"),
        n => format!("#{anchor}-{n}"),
      })
    })
    .collect()
}

fn heading_to_anchor(heading: String) -> String {
  heading
    .trim()
    .to_lowercase()
    .chars()
    .filter_map(|c| match c {
      ' ' => Some('-'),
      '.' => None,
      ',' => None,
      ';' => None,
      ':' => None,
      '!' => None,
      '?' => None,
      '`' => None,
      '\\' => None,
      '(' => None,
      ')' => None,
      '"' => None,
      '\'' => None,
      x => Some(x),
    })
    .collect()
}
