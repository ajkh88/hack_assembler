//! # Hack Assembler
//! 
//! This is an implemetation of an assembler that converts Hack Assembly Code
//! which is written in files with an .asm extension, to Hack Machine Language
//! which is denoted by the .hack file extension. The languages are defined as 
//! part of the NAND to Tetris course and in the book "The Elements of Computer
//! Systems" by Noam Nisan and Shimon Schocken 

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

    // Check the file type is correct
    assert!(filename.ends_with(".asm"), "File must be of type .asm");

    println!("Assembling code from file {}", &filename);

    let contents = fs::read_to_string(&filename)
    .expect(&format!("Something went wrong when reading the file \"{}\"...", filename));

    // First pass - collect symbols
    let mut symbol_table: SymbolTable = SymbolTable::from(&contents);

    // Second pass - build instructions
    let output: Vec<String> = parse_instructions(&contents, &mut symbol_table);

    let output_filename = filename.replace(".asm", ".hack");

    fs::write(format!("{}", output_filename), output.join("\n") + "\n")
    .expect(&format!("Something went wrong when writing to the file \"{}\"...", output_filename));

    println!("Success! Machine code output to file {}", output_filename);

    Ok(())
}
