use rasm::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

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

    println!("{contents}");

    // filter out empty lines and comments (lines starting with '///')
    let lines: Vec<&str> = contents
        .lines()
        .filter(|line| !line.is_empty() && line.get(..3) != Some("///"))
        .collect();

    // Extract constant lines (C-like macros behavior)
    let constant_lines: Vec<&str> = lines
        .clone()
        .into_iter()
        .filter(|line| line.starts_with("!"))
        .collect();

    // Parse constants from directives
    // Format: !CONSTANT_NAME value
    // Example: !MY_REG A, !MEMORY_ADDR 20h, !CONSTANT 42
    let mut constants: HashMap<String, String> = HashMap::new();

    for line in constant_lines {
        let parts: Vec<&str> = line[1..].split(": ").collect(); // Skip the '!' character

        if parts.len() != 2 {
            return Err(format!("Invalid constant format: {line}").into());
        }

        let name = parts[0].to_string();
        let value = parts[1].to_string();

        constants.insert(name, value);
    }

    // Extract instruction lines (exclude directives and comments)
    let meaningful_lines: Vec<Vec<&str>> = lines
        .into_iter()
        .filter(|line| line.get(..1) != Some("!"))
        .map(|line| line.split_whitespace().collect())
        .collect();

    // Replace constant names with their defined values
    // This implements the macro-like behavior where constants are substituted
    let replaced_lines: Result<Vec<Vec<String>>, String> = meaningful_lines
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|token| {
                    // Check if token is defined as a constant and replace it
                    // If not found, keep the original token
                    if constants.contains_key(token) {
                        Ok(constants[token].clone())
                    } else {
                        Ok(token.to_string())
                    }
                })
                .collect()
        })
        .collect();

    let replaced_lines = replaced_lines?;

    let instructions: Vec<Instruction> = replaced_lines
        .into_iter()
        .map(|line| {
            let str_refs: Vec<&str> = line.iter().map(|s| s.as_str()).collect();
            Instruction::build(str_refs)
        })
        .collect::<Result<Vec<Instruction>, _>>()?;

    println!("{instructions:#?}");

    let out: Vec<String> = instructions
        .into_iter()
        .map(|instruction| instruction.encode())
        .collect::<Result<Vec<String>, _>>()?;

    println!("{}", out.join("\n"));

    if let Some(output_path) = output_path {
        std::fs::write(format!("{output_path}.hex"), out.join("\n"))?;

        let mut bytes: Vec<u8> = Vec::new();

        for line in out {
            for i in (0..line.len()).step_by(2) {
                let byte = u8::from_str_radix(&line[i..i + 2], 16)?;
                bytes.push(byte);
            }
        }

        std::fs::write(format!("{output_path}"), &bytes)?;
    }

    Ok(())
}
