use crate::config;
use std::io::Read;
use std::path::Path;

mod extract;
mod process;
mod template;

/// Generates readme data from `source` file
///
/// Optionally, a template can be used to render the output
pub fn generate_readme<R: Read>(
    project_root: &Path,
    crate_name: &str,
    source: &mut R,
    template: Option<&mut R>,
    add_title: bool,
    add_badges: bool,
    add_license: bool,
    indent_headings: bool,
) -> Result<String, String> {
    let lines = extract::extract_docs(source, crate_name).map_err(|e| format!("{}", e))?;

    let readme = process::process_docs(lines, indent_headings).join("\n");

    // get template from file
    let template = if let Some(template) = template {
        Some(get_template_string(template)?)
    } else {
        None
    };

    // get manifest from Cargo.toml
    let cargo = config::get_manifest(project_root)?;

    template::render(template, readme, &cargo, add_title, add_badges, add_license)
}

/// Load a template String from a file
fn get_template_string<R: Read>(template: &mut R) -> Result<String, String> {
    let mut template_string = String::new();
    if let Err(e) = template.read_to_string(&mut template_string) {
        return Err(format!("Error: {}", e));
    }

    Ok(template_string)
}
