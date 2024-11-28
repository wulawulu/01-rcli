use crate::CmdExecutor;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Sign jwt")]
    Sign(JwtSignOpts),
    #[command(name = "verify", about = "verify jwt")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub aud: String,
    #[arg(short, long)]
    pub exp: usize,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret("secret".as_ref()),
        )?;
        println!("{}", token);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
    #[arg(short, long)]
    pub aud: String,
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut validation = Validation::default();
        validation.set_audience(&[&self.aud]);
        let token = decode::<JwtSignOpts>(
            &self.token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        )?;
        println!("{:?}", token.claims);
        Ok(())
    }
}
