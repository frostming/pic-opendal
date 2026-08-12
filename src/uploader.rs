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

fn content_type_for_path(path: &Path) -> String {
    mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string()
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

        let content_type = content_type_for_path(path);

        self.operator
            .write_with(&remote_path, content)
            .content_type(&content_type)
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

#[cfg(test)]
mod tests {
    use super::content_type_for_path;
    use std::path::Path;

    #[test]
    fn provides_mime_type_for_common_image_extensions() {
        assert_eq!(content_type_for_path(Path::new("example.png")), "image/png");
        assert_eq!(
            content_type_for_path(Path::new("example.jpg")),
            "image/jpeg"
        );
        assert_eq!(
            content_type_for_path(Path::new("example.webp")),
            "image/webp"
        );
    }

    #[test]
    fn handles_case_insensitive_extensions() {
        assert_eq!(
            content_type_for_path(Path::new("example.JPG")),
            "image/jpeg"
        );
    }

    #[test]
    fn falls_back_to_octet_stream_for_unknown_extensions() {
        assert_eq!(
            content_type_for_path(Path::new("example.unknown-extension")),
            "application/octet-stream"
        );
    }
}
