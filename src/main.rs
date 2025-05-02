use rasm::*;
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

    let lines: Vec<&str> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| line.get(..3).unwrap() != "///")
        .collect();

    // let directives: Vec<&str> =
    //     lines
    //     .clone()
    //     .into_iter()
    //     .filter(|line| line.get(..1).unwrap() == "!")
    //     .collect();

    let meaningful_lines: Vec<Vec<&str>> = lines
        .into_iter()
        .filter(|line| line.get(..1).unwrap() != "!")
        .map(|line| line.split_whitespace().collect())
        .collect();

    println!("{meaningful_lines:#?}");

    let instructions: Vec<Instruction> = meaningful_lines
        .into_iter()
        .map(Instruction::new)
        .collect::<Result<Vec<Instruction>, _>>()?;

    println!("{instructions:#?}");

    let out: Vec<String> = instructions
        .iter()
        .map(|instruction| instruction.build())
        .collect::<Result<Vec<String>, _>>()?;

    println!("{}", out.join("\n"));

    Ok(())
}
