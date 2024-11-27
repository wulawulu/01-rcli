mod base64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::{CsvOpts, OutputFormat},
    genpass::GenPassOpts,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};
use clap::{command, Parser};
use std::path::{Path, PathBuf};
use crate::CmdExecutor;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSC, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

impl CmdExecutor for SubCommand{
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) =>opts.execute().await,
            SubCommand::Base64(opts) =>opts.execute().await,
            SubCommand::Text(opts) => opts.execute().await,
            SubCommand::Http(opts) => opts.execute().await,
        }
    }
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
