#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{
    env, fs,
    io::{self, BufWriter, Write},
};

use syn_serde::json;

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

    let json = fs::read_to_string(input_path)?;
    let syntax: syn::File = json::from_str(&json)?;
    let out = prettyplease::unparse(&syntax);

    if let Some(output_path) = output_path {
        fs::write(output_path, out)?;
    } else {
        let mut writer = BufWriter::new(io::stdout().lock());
        writer.write_all(out.as_bytes())?;
        writer.flush()?;
    }
    Ok(())
}
