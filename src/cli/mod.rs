mod base64;
mod csv;
mod genpass;

pub use self::base64::{Base64Format, Base64SubCommand};
use clap::{command, Parser};
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;
use std::path::Path;

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
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
