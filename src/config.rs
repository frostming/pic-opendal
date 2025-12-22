use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub current_profile: Option<String>,
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "type")]
    pub scheme: String,
    pub base_url: String,
    #[serde(default)]
    pub filename_format: Option<String>,
    #[serde(flatten)]
    pub options: HashMap<String, String>,
}

const CONFIG_TEMPLATE: &str = r#"# pic-od configuration file
# Documentation: https://github.com/frostming/pic-opendal

# Current active profile
current_profile = "default"

# Profile definitions
# Each profile configures a storage backend

# [profiles.default]
# # Storage backend type: s3, gcs, azblob, oss, cos, obs, fs, webdav, etc.
# type = "s3"
#
# # S3-compatible storage options
# bucket = "my-bucket"
# region = "us-east-1"
# access_key_id = "YOUR_ACCESS_KEY"
# secret_access_key = "YOUR_SECRET_KEY"
# # Optional: root path in the bucket
# root = "/images"
#
# # Base URL for generating public URLs
# base_url = "https://cdn.example.com"
#
# # Filename format template
# # Available variables:
# #   {name}  - Original filename with extension
# #   {stem}  - Original filename without extension
# #   {ext}   - File extension
# #   {date}  - Current date (YYYYMMDD)
# filename_format = "{date}/{stem}.{ext}"
"#;

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, CONFIG_TEMPLATE)?;
            eprintln!("Created config template at: {}", path.display());
            return Ok(Config::default());
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        toml::from_str(&content).with_context(|| "Failed to parse config file")
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Cannot determine config directory")?;
        Ok(config_dir.join("pic-od").join("config.toml"))
    }

    pub fn get_profile(&self, name: Option<&str>) -> Result<&Profile> {
        let profile_name = name
            .or(self.current_profile.as_deref())
            .context("No profile specified and no current profile set")?;
        self.profiles
            .get(profile_name)
            .with_context(|| format!("Profile '{}' not found", profile_name))
    }

    pub fn set_current_profile(&mut self, name: &str) -> Result<()> {
        if !self.profiles.contains_key(name) {
            anyhow::bail!("Profile '{}' not found", name);
        }
        self.current_profile = Some(name.to_string());
        self.save()
    }
}
