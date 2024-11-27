use super::verify_path;
use crate::{process_http_serve, CmdExecutor};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser=verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}
