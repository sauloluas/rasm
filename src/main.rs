use std::error::Error;
use std::fs::read_to_string;
use std::{env, fs};

use rasm::Overroot;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("try: rasm <input file name> [output file name]");
        return Err("not enough arguments".into());
    }

    let file_path = &args[1];
    let output_path = args.get(2);

    if !file_path.ends_with(".rasm") {
        return Err(format!("Invalid file format: {}", file_path).into());
    }

    let contents =
        read_to_string(file_path).map_err(|err| format!("Failed to read file: {}", err))?;

    println!("{contents}\n");

    let overroot = Overroot::try_from(contents)?;

    println!("{overroot:#?}\n");

    let out = overroot.encode()?;

    println!("{}", out.join("\n"));

    if let Some(output_path) = output_path {
        fs::write(format!("{output_path}.hex"), out.join("\n"))?;

        let mut bytes: Vec<u8> = Vec::new();

        for line in out {
            for i in (0..line.len()).step_by(2) {
                let byte = u8::from_str_radix(&line[i..i + 2], 16)?;
                bytes.push(byte);
            }
        }

        fs::write(format!("{output_path}.brx"), &bytes)?;
    }

    Ok(())
}
