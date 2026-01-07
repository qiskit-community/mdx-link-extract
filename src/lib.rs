#![deny(clippy::all)]

use markdown::mdast::{AttributeContent, AttributeValue, MdxJsxTextElement};
use markdown::{mdast::Node, to_mdast, Constructs, ParseOptions};
use napi::Error;
use napi_derive::napi;
use std::collections::HashSet;
use std::fs;

use crate::notebook::extract_markdown_from_notebook_source;

mod notebook;

#[napi]
pub fn extract_links_from_file(file_path: String) -> Result<Vec<String>, Error> {
  let is_notebook = file_path.ends_with(".ipynb");
  let source = match fs::read_to_string(file_path) {
    Ok(s) => s,
    Err(e) => return Err(Error::from_reason(e.to_string())),
  };

  let markdown = if is_notebook {
    match extract_markdown_from_notebook_source(source) {
      Ok(md) => md,
      Err(reason) => return Err(Error::from_reason(reason)),
    }
  } else {
    source
  };

  extract_links(markdown)
}

/// Extract links from a markdown string. Supports GitHub-flavored markdown
/// (gfm), math, and JSX.
#[napi]
pub fn extract_links(markdown: String) -> Result<Vec<String>, Error> {
  let options = ParseOptions {
    constructs: Constructs {
      gfm_autolink_literal: true,
      gfm_footnote_definition: true,
      gfm_label_start_footnote: true,
      gfm_strikethrough: true,
      gfm_table: true,
      gfm_task_list_item: true,
      math_flow: true,
      math_text: true,
      mdx_jsx_flow: true,
      mdx_jsx_text: true,
      ..Constructs::mdx()
    },
    ..ParseOptions::mdx()
  };

  let ast = match to_mdast(markdown.as_str(), &options) {
    Ok(ast) => ast,
    Err(m) => return Err(Error::from_reason(m.to_string())),
  };

  let mut links = HashSet::<&String>::default();
  extract_from_node(&ast, &mut links);

  Ok(links.into_iter().cloned().collect())
}

fn extract_from_node<'a>(node: &'a Node, links: &mut HashSet<&'a String>) {
  let maybe_link = match node {
    Node::Image(img) => Some(&img.url),
    Node::Link(link) => Some(&link.url),
    Node::MdxJsxTextElement(el) => extract_from_jsx_text_element(el),
    _ => None,
  };

  if let Some(link) = maybe_link {
    links.insert(link);
  }

  if let Some(children) = node.children() {
    for child in children {
      extract_from_node(child, links);
    }
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
