use rasm::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("try: rasm [input file name] [output file name] ");
        return Err("not enough arguments".into());
    }

    let file_path = &args[1];

    let contents = read_to_string(file_path)?;
    
    println!("{contents}");

    // filter out empty lines and comments (lines starting with '///')
    let lines: Vec<&str> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| line.get(..3).unwrap() != "///")
        .collect();

    // Extract constant lines (C-like macros behavior)
    let constant_lines: Vec<&str> = lines
        .clone()
        .into_iter()
        .filter(|line| line.get(..1).unwrap() == "!")
        .collect();

    // Parse constants from directives
    // Format: !CONSTANT_NAME value
    // Example: !MY_REG A, !MEMORY_ADDR 20h, !CONSTANT 42
    let mut constants: HashMap<String, String> = HashMap::new();

    for line in constant_lines {
        let parts: Vec<&str> = line[1..].split(": ").collect(); // Skip the '!' character

        if parts.len() != 2 {
            return Err(format!("Invalid constant format: {}", line).into());
        }

        let var_name = parts[0].to_string();
        let var_value = parts[1].to_string();

        constants.insert(var_name, var_value);
    }

    // Extract instruction lines (exclude directives and comments)
    let meaningful_lines: Vec<Vec<&str>> = lines
        .into_iter()
        .filter(|line| line.get(..1).unwrap() != "!")
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
                .collect::<Result<Vec<String>, String>>()
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

    Ok(())
}
