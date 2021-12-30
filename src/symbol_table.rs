use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_address: u16
}

impl Default for SymbolTable {
    fn default() -> SymbolTable {
        let initial_values: [(String, u16);23] = [
            (String::from("R0"), 0),
            (String::from("R1"), 1),
            (String::from("R2"), 2),
            (String::from("R3"), 3),
            (String::from("R4"), 4),
            (String::from("R5"), 5),
            (String::from("R6"), 6),
            (String::from("R7"), 7),
            (String::from("R8"), 8),
            (String::from("R9"), 9),
            (String::from("R10"), 10),
            (String::from("R11"), 11),
            (String::from("R12"), 12),
            (String::from("R13"), 13),
            (String::from("R14"), 14),
            (String::from("R15"), 15),
            (String::from("SCREEN"), 16384),
            (String::from("KBD"), 24576),
            (String::from("SP"), 0),
            (String::from("LCL"), 1),
            (String::from("ARG"), 2),
            (String::from("THIS"), 3),
            (String::from("THAT"), 4),
        ];
        let table = HashMap::from(initial_values);
        const INITIAL_NEXT_ADDRESS: u16 = 16;
        SymbolTable{ table, next_address: INITIAL_NEXT_ADDRESS }
    }
}

impl SymbolTable {
    fn new() -> Self {
        Default::default()
    }

    pub fn from(contents: &str) -> SymbolTable {
        let mut symbol_table = SymbolTable::new();

        let mut line_number: u16 = 0;
        for mut line in contents.lines() {
            line = remove_comments(line);
            if line.is_empty() {
                continue;
            } 
            if line.trim().starts_with("(") && line.trim().ends_with(")") {
                let label = &line[1..line.len() - 1];
                symbol_table.add_label(label.to_string(), line_number);
            } else {
                line_number += 1;
            }
        }
        symbol_table
    }

    fn add_label(&mut self, label: String, address: u16) {
        self.table.insert(label, address);
    }

    fn add_symbol(&mut self, symbol: String) -> u16 {
        let address = self.next_address;
        self.table.insert(symbol, address);
        self.next_address += 1;
        address
    }

    pub fn get_address_of_symbol(&mut self, symbol: String) -> u16 {
        return match self.table.get(&symbol) {
            Some(address) => *address,
            None => self.add_symbol(symbol)
        }
    }
}

fn remove_comments(line: &str) -> &str {
    return line.split("//").collect::<Vec<_>>()[0].trim();
}
