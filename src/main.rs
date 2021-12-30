use std::env;
use std::fs;
use std::io::{Result};

mod symbol_table;
use symbol_table::SymbolTable;

mod instruction_parser;
use instruction_parser::parse_instructions;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: asm FILENAME");
        std::process::exit(1);
    }

    let filename = args[1].clone();

    println!("Assembling code from file {}", &filename);

    let contents = fs::read_to_string(&filename)
    .expect("Something went wrong reading the file - Fuck!");

    // First pass - collect symbols
    let mut symbol_table: SymbolTable = SymbolTable::from(&contents);

    // Second pass - build instructions
    let output: Vec<String> = parse_instructions(&contents, &mut symbol_table);

    let output_filename = filename.replace(".asm", ".hack");

    fs::write(format!("{}", output_filename), output.join("\n") + "\n").unwrap();

    println!("Success! Machine code output to file {}", output_filename);

    Ok(())
}
