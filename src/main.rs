use clap::{Parser, Subcommand};
use futures::executor::block_on;
use std::ffi::c_uint;

extern "C" {
    fn add(x_input_one: c_uint, x_input_two: c_uint) -> c_uint;
    fn sub(x_input_one: c_uint, x_input_two: c_uint) -> c_uint;
    fn mul(x_input_one: c_uint, x_input_two: c_uint) -> c_uint;
    fn div(x_input_one: c_uint, x_input_two: c_uint) -> c_uint;
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
    /// Input two uints to subtract
    Sub {
        input_one: c_uint,
        input_two: c_uint,
    },
    /// Input two uints to multiply
    Mul {
        input_one: c_uint,
        input_two: c_uint,
    },
    /// Input two uints to divide
    Div {
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
        Commands::Sub {
            input_one,
            input_two,
        } => {
            unsafe {
                sub(*input_one, *input_two);
            }
            println!("[Rust] input one - input two is: {}", input_one - input_two)
        }
        Commands::Mul {
            input_one,
            input_two,
        } => {
            unsafe {
                mul(*input_one, *input_two);
            }
            println!("[Rust] input one * input two is: {}", input_one * input_two)
        }
        Commands::Div {
            input_one,
            input_two,
        } => {
            unsafe {
                div(*input_one, *input_two);
            }
            println!("[Rust] input one / input two is: {}", input_one / input_two)
        }
    }
}
