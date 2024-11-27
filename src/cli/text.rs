use super::{verify_file, verify_path};
use crate::{
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify, CmdExecutor,
};
use clap::Parser;
use core::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "Encrypt a message")]
    ENCRYPT(TextEncryptOpts),
    #[command(about = "Decrypt a message")]
    DECRYPT(TextDecryptOpts),
}

impl CmdExecutor for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
            TextSubCommand::ENCRYPT(opts) => opts.execute().await,
            TextSubCommand::DECRYPT(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file,)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);

        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file,)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_text_verify(&self.input, &self.key, &self.sig, self.format)?;
        println!("{}", verified);

        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser=verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = self.output;
                fs::write(name.join("ed25519.sk"), &key[0]).await?;
                fs::write(name.join("ed25519.pk"), &key[1]).await?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file,)]
    pub key: String,
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = process_text_encrypt(&self.input, &self.key)?;
        println!("{}", encoded);

        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file,)]
    pub key: String,
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = process_text_decrypt(&self.input, &self.key)?;
        println!("{}", decoded);

        Ok(())
    }
}

fn parse_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            v => Err(anyhow::anyhow!("Invalid base64 format: {}", v)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
