use markdown::mdast::Node;
use markdown::mdast::{AttributeContent, AttributeValue, MdxJsxTextElement};
use std::collections::HashSet;

pub fn extract_from_node<'a>(node: &'a Node, links: &mut HashSet<&'a String>) {
  let maybe_link = match node {
    Node::Image(img) => Some(&img.url),
    Node::Link(link) => Some(&link.url),
    Node::MdxJsxTextElement(el) => extract_from_jsx_text_element(el),
    _ => None,
  };

  if let Some(link) = maybe_link {
    links.insert(link);
  }
}

fn extract_from_jsx_text_element(el: &MdxJsxTextElement) -> Option<&String> {
  let Some(Some(href_attr)) = el.attributes.iter().find_map(|attr| match attr {
    AttributeContent::Property(p) if p.name == "href" => Some(&p.value),
    _ => None,
  }) else {
    return None;
  };
  match href_attr {
    AttributeValue::Literal(s) => Some(s),
    _ => None,
  }
}
