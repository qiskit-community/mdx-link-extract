use napi::Error;
use napi_derive::napi;
use tokio::fs;

use crate::anchors::extract_anchors_from_ref;
use crate::notebook::extract_markdown_from_notebook_source;

mod anchors;
mod links;
mod notebook;

fn file_read_error(path: String, reason: String) -> Error {
  let message = format!("Could not read \"{path}\": {reason}");
  Error::from_reason(message)
}

/// Extracts links and anchors from an MDX file or notebook containing MDX.
///
/// Example:
/// ```ts
/// const [links, anchors] = await extractFromFile("notebook.ipynb");
/// ```
#[napi(ts_return_type = "Promise<[string[], string[]]>")]
pub async fn extract_from_file(file_path: String) -> Result<Vec<Vec<String>>, Error> {
  let is_notebook = file_path.ends_with(".ipynb");
  let source = match fs::read_to_string(&file_path).await {
    Ok(s) => s,
    Err(e) => return Err(file_read_error(file_path, e.to_string())),
  };

  let markdown = if is_notebook {
    match extract_markdown_from_notebook_source(source) {
      Ok(md) => md,
      Err(e) => return Err(file_read_error(file_path, e.to_string())),
    }
  } else {
    source
  };

  let anchors = extract_anchors_from_ref(&markdown);
  match extract_links(markdown) {
    Ok(links) => Ok(vec![links, anchors]),
    Err(e) => Err(Error::from_reason(e.to_string())),
  }
}

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
  links::extract_links(markdown)
}
