use std::collections::HashMap;

use crate::symbol_table::SymbolTable;

/// parse_instructions takes the contents of the .asm file as a string of insctructions 
/// and parses them into a Vector of strings. The file is split into lines, comments are 
/// removed and then each line is parsed as an A or a C instruction as defined by the first 
/// character of the line. These are represented by structs which implement the insctruction
/// trait, which allows them to be converted to a bonary string, which are then collected into
/// a Vector and returned,
/// 
/// # Arguments
/// * `contents` - A pointer to the contents of the assembly code file to be parsed
/// * `symbol_table` - The table of symbols that will be used when the address of an insctuction is
/// non-numeric. This has already been defined in the first pass
/// 
pub fn parse_instructions(contents: &str, symbol_table: &mut SymbolTable) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for mut line in contents.lines() {
        line = remove_comments(line);
        if line.is_empty() || (line.trim().starts_with("(") && line.ends_with(")")) {
            continue;
        }
        let instruction = parse_instruction(line, symbol_table);
        result.push(instruction.to_binary_string());
    }
    result
}

fn parse_instruction(file_line: &str, symbol_table: &mut SymbolTable) -> Box<dyn Instruction> {
    if file_line.starts_with("@") {
        Box::new(AInstruction::parse(file_line.to_string(), symbol_table))
    } else {
        Box::new(CInstruction::parse(file_line.to_string()))
    }
}

// Remove anything after the // and trim any left over whitespace
fn remove_comments(instruction: &str) -> &str {
    return instruction.split("//").collect::<Vec<_>>()[0].trim();
}


trait Instruction {
    fn to_binary_string(&self) -> String;
}

#[derive(Debug)]
struct AInstruction {
    address: u16,
}

impl AInstruction {
    fn parse(file_line: String, symbol_table: &mut SymbolTable) -> AInstruction {
        let address_string = &file_line[1..];

        let address = match address_string.parse::<u16>() {
            Ok(value) => value,
            _ => symbol_table.get_address_of_symbol(address_string.to_string())
        };
        AInstruction{ address }
    }
}

impl Instruction for AInstruction {
    fn to_binary_string(&self) -> String {
        // Formatted to start with a 0 followed by a 15 digit binary address
        return format!("0{:015b}", self.address);
    }
}

#[derive(Debug)]
struct CInstruction {
    dest: u8,
    comp: u8,
    jmp: u8,
}

impl CInstruction {
    fn parse(file_line: String) -> CInstruction {

        let mut comp_table: HashMap<String, u8> = HashMap::new();
        comp_table.insert("0".to_string(),   0b_0_101010);
        comp_table.insert("1".to_string(),   0b_0_111111);
        comp_table.insert("-1".to_string(),  0b_0_111010);
        comp_table.insert("D".to_string(),   0b_0_001100);
        comp_table.insert("A".to_string(),   0b_0_110000);
        comp_table.insert("!D".to_string(),  0b_0_001101);
        comp_table.insert("!A".to_string(),  0b_0_110001);
        comp_table.insert("-D".to_string(),  0b_0_001111);
        comp_table.insert("-A".to_string(),  0b_0_110011);
        comp_table.insert("D+1".to_string(), 0b_0_011111);
        comp_table.insert("A+1".to_string(), 0b_0_110111);
        comp_table.insert("D-1".to_string(), 0b_0_001110);
        comp_table.insert("A-1".to_string(), 0b_0_110010);
        comp_table.insert("D+A".to_string(), 0b_0_000010);
        comp_table.insert("D-A".to_string(), 0b_0_010011);
        comp_table.insert("A-D".to_string(), 0b_0_000111);
        comp_table.insert("D&A".to_string(), 0b_0_000000);
        comp_table.insert("D|A".to_string(), 0b_0_010101);
        comp_table.insert("M".to_string(),   0b_1_110000);
        comp_table.insert("!M".to_string(),  0b_1_110001);
        comp_table.insert("-M".to_string(),  0b_1_110011);
        comp_table.insert("M+1".to_string(), 0b_1_110111);
        comp_table.insert("M-1".to_string(), 0b_1_110010);
        comp_table.insert("D+M".to_string(), 0b_1_000010);
        comp_table.insert("D-M".to_string(), 0b_1_010011);
        comp_table.insert("M-D".to_string(), 0b_1_000111);
        comp_table.insert("D&M".to_string(), 0b_1_000000);
        comp_table.insert("D|M".to_string(), 0b_1_010101);

        let mut dest_table: HashMap<String, u8> = HashMap::new();

        dest_table.insert("null".to_string(), 0b_000);
        dest_table.insert("M".to_string(),    0b_001);
        dest_table.insert("D".to_string(),    0b_010);
        dest_table.insert("DM".to_string(),   0b_011);
        dest_table.insert("MD".to_string(),   0b_011);
        dest_table.insert("A".to_string(),    0b_100);
        dest_table.insert("AM".to_string(),   0b_101);
        dest_table.insert("AD".to_string(),   0b_110);
        dest_table.insert("ADM".to_string(),  0b_111);

        let mut jmp_table: HashMap<String, u8> = HashMap::new();

        jmp_table.insert("null".to_string(), 0b_000);
        jmp_table.insert("JGT".to_string(),  0b_001);
        jmp_table.insert("JEQ".to_string(),  0b_010);
        jmp_table.insert("JGE".to_string(),  0b_011);
        jmp_table.insert("JLT".to_string(),  0b_100);
        jmp_table.insert("JNE".to_string(),  0b_101);
        jmp_table.insert("JLE".to_string(),  0b_110);
        jmp_table.insert("JMP".to_string(),  0b_111);

        let dest: u8;
        let mut rest_of_line: String;

        if file_line.contains("=") {
            let tokens =  file_line.split_once("=").unwrap();
            let mut dest_token = tokens.0.to_string();
            rest_of_line = tokens.1.to_string();
            dest_token.retain(|c| !c.is_whitespace());
            dest = *dest_table.get(&dest_token)
            .expect(&format!("ERROR: Failed to parse instruction {:?}", dest_token));
        } else {
            dest = 0;
            rest_of_line = file_line.clone();
        }

        let jmp: u8;

        if rest_of_line.contains(";") {
            let tokens =  rest_of_line.split_once(";").unwrap();
            let mut jmp_token = tokens.1.to_string();
            rest_of_line = tokens.0.to_string();
            jmp_token.retain(|c| !c.is_whitespace());
            jmp = *jmp_table.get(&jmp_token)
            .expect(&format!("ERROR: Failed to parse instruction {:?}", jmp_token));
        } else {
            jmp = 0;
        }

        let mut comp_token = rest_of_line.clone();
        comp_token.retain(|c| !c.is_whitespace());
        let comp = *comp_table.get(&comp_token)
        .expect(&format!("ERROR: Failed to parse instruction {:?}", comp_token));

        CInstruction{dest, comp, jmp}
    }
}

impl Instruction for CInstruction {
    fn to_binary_string(&self) -> String {
        // Formatted to start with 111 followed by the format accccccdddjjj
        // where a is the type of comp instruction, c is the comp instruction
        // d is the optional dest instruction and j is the optional jump instruction
        return format!("111{:07b}{:03b}{:03b}", self.comp, self.dest, self.jmp);
    }
}

