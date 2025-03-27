use std::error::Error;
use std::env;
use std::fs::read_to_string;
use rasm::*;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect(); 

    if args.len() < 2 {
        println!("try: rasm [input file name] [output file name] ");
        return Err("not enough arguments".into());
    }

    let file_path = &args[1];

    let contents = read_to_string(file_path)?;

    let lines: Vec<Vec<&str>> = 
        contents
        .lines()
        .filter(|line| *line != "")
        .filter(|line| line.get(..3).unwrap() != "///")
        .map(
            |line| 
            line.split(char::is_whitespace)
            .collect()
            )
        .collect();


    println!("{lines:#?}");


    Ok(())
}

