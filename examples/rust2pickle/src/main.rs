#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::fs;
use structopt::StructOpt;
mod pickle;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = Cli::from_args();
    let filepath = args.input;

    let code = fs::read_to_string(&filepath)?;
    let syntax = syn::parse_file(&code)?;

    let buf = pickle::to_vec(&syntax);

    let _ = fs::write(args.output, buf);
    Ok(())
}
