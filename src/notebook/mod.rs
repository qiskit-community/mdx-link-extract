use serde::Deserialize;

pub fn extract_markdown_from_notebook_source(source: String) -> Result<String, String> {
  let notebook: Notebook = match serde_json::from_str(&source) {
    Ok(s) => s,
    Err(e) => return Err(e.to_string()),
  };

  let markdown: String = notebook
    .cells
    .into_iter()
    .filter(|cell| cell.cell_type == "markdown")
    .map(|cell| cell.source.join(""))
    .collect::<Vec<String>>()
    .join("\n\n");

  Ok(markdown)
}

#[derive(Deserialize)]
struct Notebook {
  cells: Vec<NotebookCell>,
}

#[derive(Deserialize)]
struct NotebookCell {
  cell_type: String,
  source: Vec<String>,
}
