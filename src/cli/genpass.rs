use crate::{process_gen_pass, CmdExecutor};
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_gen_pass(
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
            self.length,
        )?;
        println!("{}", password);

        let estimate = zxcvbn(&password, &[]);
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
