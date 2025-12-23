mod config;
mod template;
mod uploader;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::Config;
use std::path::PathBuf;
use uploader::Uploader;

#[derive(Parser)]
#[command(name = "pic-od", about = "Image upload CLI using OpenDAL")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Upload images and return URLs
    Upload {
        /// Use a specific profile for this command
        #[arg(short, long, env = "PIC_OD_PROFILE")]
        profile: Option<String>,
        /// Image paths to upload
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
    /// Set the current target profile
    Profile {
        /// Profile name to set as current
        name: String,
    },
    /// List all available profiles
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload { profile, paths } => {
            let config = Config::load()?;
            let profile = config.get_profile(profile.as_deref())?;
            let uploader = Uploader::new(profile)?;

            let path_refs: Vec<_> = paths.iter().map(|p| p.as_path()).collect();
            let results = uploader.upload_many(&path_refs).await;

            for (path, result) in paths.iter().zip(results) {
                match result {
                    Ok(url) => println!("{}", url),
                    Err(e) => eprintln!("Error uploading {}: {}", path.display(), e),
                }
            }
        }
        Commands::Profile { name } => {
            let mut config = Config::load()?;
            config.set_current_profile(&name)?;
            println!("Current profile set to: {}", name);
        }
        Commands::List => {
            let config = Config::load()?;
            if config.profiles.is_empty() {
                println!("No profiles configured.");
                println!("Add profiles to: {}", Config::config_path()?.display());
            } else {
                let current = config.current_profile.as_deref();
                for name in config.profiles.keys() {
                    let marker = if current == Some(name) { " *" } else { "" };
                    println!("{}{}", name, marker);
                }
            }
        }
    }

    Ok(())
}
