//! # Request
//!
//! crate , contains core parts of downloading

pub mod request {
    use clap::{Parser, Subcommand};

    use crate::requestcore::request::request::download::MediaType;

    /// CLI parser
    #[derive(Parser)]
    #[command(name = "yt-downloader")]
    #[command(version = "0.1.0")]
    #[command(about = "Small async YouTube video/audio downloader")]
    pub struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    /// CLI commands
    #[derive(Subcommand)]
    enum Commands {
        /// Download video
        Video {
            /// YouTube URLs
            urls: Vec<String>,
        },

        /// Download audio and convert it to MP3
        Audio {
            /// YouTube URLs
            urls: Vec<String>,
        },
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Video { urls } => {
                let config = CFG::new(urls, MediaType::Video);
                download::down_proc(config).await?;
            }

            Commands::Audio { urls } => {
                let config = CFG::new(urls, MediaType::Audio);
                download::down_proc(config).await?;
            }
        }

        Ok(())
    }

    /// Holds the settings for request of the user
    pub struct CFG {
        urls: Vec<String>,
        content_type: MediaType,
    }

    impl CFG {
        pub fn new(urls: Vec<String>, content_type: MediaType) -> Self {
            Self { urls, content_type }
        }
    }

    pub mod download {
        use super::CFG;
        use std::process::Stdio;
        use tokio::process::Command;

        /// Can switch the code between different media types
        #[derive(Clone, Copy)]
        pub enum MediaType {
            Video,
            Audio,
        }

        pub(super) async fn down_proc(
            config: CFG,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut handles = Vec::new();

            for url in config.urls {
                let media_type = config.content_type;

                let handle = tokio::spawn(async move {
                    match media_type {
                        MediaType::Video => video_file(&url).await,
                        MediaType::Audio => audio_file(&url).await,
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                handle.await??;
            }

            Ok(())
        }
        async fn video_file(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            use std::process::Stdio;

            println!("Starting yt-dlp video download for: {url}");

            let status = Command::new("yt-dlp.exe")
                .args([
                    "-P",
                    ".",
                    "-o",
                    "%(title)s [%(id)s].%(ext)s",
                    "-f",
                    "bv*+ba/b",
                    url,
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .await?;

            if !status.success() {
                return Err(format!("yt-dlp video download failed with status: {status}").into());
            }

            Ok(())
        }
        async fn audio_file(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            println!("Starting yt-dlp for: {url}");

            let status = Command::new("yt-dlp.exe")
                .args([
                    "-P",
                    ".",
                    "-x",
                    "--audio-format",
                    "mp3",
                    "--audio-quality",
                    "0",
                    url,
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .await?;

            if !status.success() {
                return Err(format!("yt-dlp failed with status: {status}").into());
            }

            Ok(())
        }
    }
}
