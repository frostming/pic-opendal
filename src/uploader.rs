use crate::config::Profile;
use crate::template::render_filename;
use anyhow::{Context, Result};
use opendal::Operator;
use std::collections::HashMap;
use std::path::Path;

pub struct Uploader {
    operator: Operator,
    base_url: String,
    filename_format: Option<String>,
}

impl Uploader {
    pub fn new(profile: &Profile) -> Result<Self> {
        let operator = Self::build_operator(&profile.scheme, &profile.options)?;
        Ok(Self {
            operator,
            base_url: profile.base_url.clone(),
            filename_format: profile.filename_format.clone(),
        })
    }

    fn build_operator(scheme: &str, options: &HashMap<String, String>) -> Result<Operator> {
        let operator = Operator::via_iter(scheme, options.clone())?;
        Ok(operator)
    }

    pub async fn upload(&self, path: &Path) -> Result<String> {
        let content = tokio::fs::read(path)
            .await
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let remote_path = render_filename(self.filename_format.as_deref(), path);

        eprintln!("Uploading {} -> {}", path.display(), remote_path);

        self.operator
            .write(&remote_path, content)
            .await
            .with_context(|| format!("Failed to upload to: {}", remote_path))?;

        eprintln!("Uploaded successfully");

        let root = self.operator.info().root();
        let root = root.trim_matches('/');
        let remote_path_trimmed = remote_path.trim_start_matches('/');
        let full_path = if root.is_empty() {
            remote_path_trimmed.to_string()
        } else {
            format!("{}/{}", root, remote_path_trimmed)
        };
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), full_path);
        Ok(url)
    }

    pub async fn upload_many(&self, paths: &[&Path]) -> Vec<Result<String>> {
        let mut results = Vec::with_capacity(paths.len());
        for path in paths {
            results.push(self.upload(path).await);
        }
        results
    }
}
