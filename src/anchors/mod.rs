use fancy_regex::Regex;
use std::collections::HashMap;

pub fn extract_anchors_from_ref(markdown: &str) -> Vec<String> {
  let heading_regex = Regex::new("^\\s*#{1,6}\\s+(.+?)\\s*$").unwrap();
  let id_regex = Regex::new("(?<=id=\")(.+?)(?=\")").unwrap();

  let mut anchor_occurrences = HashMap::<String, u32>::default();
  for line in markdown.split("\n") {
    if let Some(heading) = get_first_capture(line, &heading_regex) {
      let anchor = heading_to_anchor(heading);
      let existing_duplicates = anchor_occurrences.get(&anchor).unwrap_or(&0);
      anchor_occurrences.insert(anchor, *existing_duplicates + 1);
    }
    if let Some(id) = get_first_capture(line, &id_regex) {
      if !anchor_occurrences.contains_key(id) {
        anchor_occurrences.insert(id.to_string(), 1);
      }
    }
  }

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

fn heading_to_anchor(heading: &str) -> String {
  let heading_without_links = unwrap_markdown_links(heading);
  heading_without_links
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

fn get_first_capture<'a>(s: &'a str, r: &Regex) -> Option<&'a str> {
  let Ok(Some(captures)) = r.captures(s) else {
    return None;
  };
  Some(captures.get(1)?.as_str())
}

/// Unwraps the text inside every markdown link found in `markdown`.
///
/// Example:
/// "My [heading with links](/test)" -> "My heading with links"
fn unwrap_markdown_links(markdown: &str) -> String {
    let re = Regex::new(r"\[([^\[\]]+)\]\(([^)]+)\)").unwrap();
    re.replace_all(markdown, "$1").to_string()
}
