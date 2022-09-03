#![warn(rust_2018_idioms, single_use_lifetimes)]

mod pickle;

use std::{
    env, fs,
    io::{self, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let (input_path, output_path) = match &args[..] {
        [input] => (input, None),
        [input, output] => (input, Some(output)),
        _ => {
            println!("Usage: rust2pickle <input_path> [output_path]");
            std::process::exit(1);
        }
    };

    let code = fs::read_to_string(input_path)?;
    let syntax = syn::parse_file(&code)?;

    let buf = pickle::to_vec(&syntax);
    if let Some(output_path) = output_path {
        fs::write(output_path, buf)?;
    } else {
        let mut writer = BufWriter::new(io::stdout().lock());
        writer.write_all(&buf)?;
        writer.flush()?;
    }
    Ok(())
}
