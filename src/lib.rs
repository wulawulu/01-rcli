mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
