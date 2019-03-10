use failure::{bail, Error};
use serde_syn::json;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{}", error);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let mut args = env::args_os();
    let _ = args.next(); // executable name

    let filepath = match (args.next(), args.next()) {
        (Some(arg), None) => PathBuf::from(arg),
        _ => bail!("Usage: rust2json path/to/filename.rs"),
    };

    let code = fs::read_to_string(&filepath)?;
    let syntax = syn::parse_file(&code)?;

    json::to_writer_pretty(io::stdout(), &syntax)?;
    Ok(())
}
