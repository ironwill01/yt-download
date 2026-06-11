# yt-download

A simple, fast, and asynchronous YouTube video/audio downloader built in Rust.

It acts as a lightweight CLI wrapper around **yt-dlp**, with support for concurrent downloads using Tokio.

## Features

- **Async concurrent downloads** — Download multiple videos at the same time
- **Video downloads** — Best quality video + audio
- **Audio downloads** — Extract audio and convert directly to MP3
- Modern Rust CLI with `clap`
- Clean async architecture using Tokio

## Installation

### Prerequisites
- Rust (latest stable)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) installed and available in your PATH
  - On Windows: `yt-dlp.exe`
  - On Linux/macOS: `yt-dlp`

### Build from source

```bash
git clone https://github.com/ironwill01/yt-download.git
cd yt-download
cargo build --release
