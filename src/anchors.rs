use markdown::mdast::{AttributeContent, AttributeValue, Heading, Node};
use std::collections::HashMap;

/// If `node` is a heading or mdx element with `id` prop, extract the heading
/// text and add it to `anchor_occurences`
pub fn extract_from_node(node: &Node, anchor_occurrences: &mut HashMap<String, u32>) {
  match node {
    Node::Heading(h) => {
      let anchor = anchor_from_heading(h);
      let existing_duplicates = anchor_occurrences.get(&anchor).unwrap_or(&0);
      anchor_occurrences.insert(anchor, *existing_duplicates + 1);
    }
    Node::MdxJsxFlowElement(el) => {
      if let Some(anchor) = get_id_prop(&el.attributes) {
        anchor_occurrences.insert(anchor, 1);
      };
    }
    Node::MdxJsxTextElement(el) => {
      if let Some(anchor) = get_id_prop(&el.attributes) {
        anchor_occurrences.insert(anchor, 1);
      };
    }
    _ => (),
  };
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

fn get_id_prop(attributes: &Vec<AttributeContent>) -> Option<String> {
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

fn anchor_from_heading(heading: &Heading) -> String {
  let mut text = String::with_capacity(100);
  for child in heading.children.iter() {
    get_text(child, &mut text);
  }
  heading_to_anchor(text)
}

/// Get plain text from a node and all its children
pub fn get_text<'a>(node: &'a Node, s: &mut String) {
  let maybe_text = match node {
    Node::Text(text) => Some(&text.value),
    Node::InlineCode(text) => Some(&text.value),
    Node::InlineMath(text) => Some(&text.value),
    _ => None,
  };
  if let Some(text) = maybe_text {
    s.push_str(text.as_str())
  };

  if let Some(children) = node.children() {
    for child in children {
      get_text(child, s);
    }
  }
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
      '{' => None,
      '}' => None,
      x => Some(x),
    })
    .collect()
}
