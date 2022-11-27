use clap::{Parser, Subcommand};
use futures::executor::block_on;
use std::ffi::c_uint;

extern "C" {
    fn add(x_input_one: c_uint, x_input_two: c_uint) -> c_uint;
}

#[tokio::main]
async fn main() {
    block_on(cli_main());
}

#[derive(Parser)]
#[command(disable_help_subcommand(true), about, version, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Input two uints to add
    Add {
        input_one: c_uint,
        input_two: c_uint,
    },
}

async fn cli_main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add {
            input_one,
            input_two,
        } => {
            unsafe {
                add(*input_one, *input_two);
            }
            println!("[Rust] input one + input two is: {}", input_one + input_two)
        }
    }
}
