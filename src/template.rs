use chrono::Local;
use std::path::Path;

pub struct TemplateContext {
    pub name: String,
    pub stem: String,
    pub ext: String,
    pub date: String,
}

impl TemplateContext {
    pub fn new(path: &Path) -> Self {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file")
            .to_string();

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file")
            .to_string();

        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let date = Local::now().format("%Y%m%d").to_string();

        Self {
            name,
            stem,
            ext,
            date,
        }
    }

    pub fn render(&self, template: &str) -> String {
        template
            .replace("{name}", &self.name)
            .replace("{stem}", &self.stem)
            .replace("{ext}", &self.ext)
            .replace("{date}", &self.date)
    }
}

pub fn render_filename(template: Option<&str>, path: &Path) -> String {
    let ctx = TemplateContext::new(path);
    match template {
        Some(t) => ctx.render(t),
        None => ctx.name,
    }
}
