use markdown::mdast::{AttributeContent, AttributeValue, MdxJsxTextElement};
use markdown::{mdast::Node, to_mdast, Constructs, ParseOptions};
use napi::Error;
use napi_derive::napi;
use std::collections::HashSet;
use tokio::fs;

use crate::notebook::extract_markdown_from_notebook_source;

mod notebook;

fn file_read_error(path: String, reason: String) -> Result<Vec<String>, Error> {
  let message = format!("Could not read \"{path}\": {reason}");
  Err(Error::from_reason(message))
}

#[napi]
pub async fn extract_links_from_file(file_path: String) -> Result<Vec<String>, Error> {
  let is_notebook = file_path.ends_with(".ipynb");
  let source = match fs::read_to_string(&file_path).await {
    Ok(s) => s,
    Err(e) => return file_read_error(file_path, e.to_string()),
  };

  let markdown = if is_notebook {
    match extract_markdown_from_notebook_source(source) {
      Ok(md) => md,
      Err(e) => return file_read_error(file_path, e.to_string()),
    }
  } else {
    source
  };

  extract_links(markdown)
}

use crate::anchors::extract_anchors_from_ref;

mod anchors;

/// Extract anchors from a markdown string. Anchors are either:
///  * slugified headings, deduplicated if the same heading appears more than once
///  * `id` props of HTML tags. These are not deduplicated as they should be unique per file
#[napi]
pub fn extract_anchors(markdown: String) -> Vec<String> {
  extract_anchors_from_ref(&markdown)
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
