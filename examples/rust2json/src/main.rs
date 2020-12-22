#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{
    fs,
    io::{self, BufWriter, Write},
};
use structopt::StructOpt;
use syn_serde::json;

type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output_path: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let code = fs::read_to_string(&args.input_path)?;
    let syntax = syn::parse_file(&code)?;

    if let Some(outpath) = args.output_path {
        let buf = json::to_string_pretty(&syntax);
        fs::write(outpath, buf)?;
    } else {
        let writer = io::stdout();
        let mut writer = BufWriter::new(writer.lock());
        json::to_writer_pretty(&mut writer, &syntax)?;
        writer.flush()?;
    }
    Ok(())
}
