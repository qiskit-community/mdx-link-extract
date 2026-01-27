use markdown::{mdast::Node, to_mdast, Constructs, ParseOptions};
use napi::Error;

pub fn parse_mdx(source: String) -> Result<Node, Error> {
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

  to_mdast(source.as_str(), &options).map_err(|message| Error::from_reason(message.to_string()))
}

/// Walk the markdown AST and call a function on each node
pub fn walk_ast<'a>(node: &'a Node, f: &mut impl FnMut(&'a Node) -> ()) -> () {
  f(node);

  if let Some(children) = node.children() {
    for child in children {
      walk_ast(child, f);
    }
  }
}
