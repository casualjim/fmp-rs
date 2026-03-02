use clap::Parser;
use config::Cli;
use error::exit_with_error;
use eyre::Result;

mod cli;
mod config;
mod error;
mod output;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    
    cli::dispatch(args).await.map_err(|e| exit_with_error(e, 1))?;
    
    Ok(())
}
