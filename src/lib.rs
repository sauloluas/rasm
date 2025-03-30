#[derive(Debug)]
pub enum Instruction {
    Adcp(Register, Register),               // Add copying
    Adl(Register, MemoryAddress),           // Add loading
    Asn(MemoryAddress, Immediate),          // Assign
    Copy(Register, Register),               // Copy
    Init(Register, Immediate),              // Initialize
    Load(Register, MemoryAddress),          // Load
    Str(MemoryAddress, Register),           // Store
}

impl Instruction {
    pub fn build(line: Vec<&str>) -> Result<Instruction, String> {

        let operation = line[0];
        let param1 = line[1];
        let param2 = line[2];

        let inst = match operation {
            "init" => Self::Init(
                Register::build(param1)?,
                Immediate::build(param2)?
            ),
            "copy" => Self::Copy(
                Register::build(param1)?,
                Register::build(param2)?
            ),
            "adcp" => Self::Adcp(
                Register::build(param1)?,
                Register::build(param2)?
            ),
            "str" => Self::Str(
                MemoryAddress::build(param1)?,
                Register::build(param2)?
            ),
            _ => return Err(format!("Invalid operation: {operation}"))
        };

        Ok(inst)

    }
}

#[derive(Debug)]
pub struct Register {
    reg_id: u8
}

impl Register {
    pub fn build(param: &str) -> Result<Register, String> {
        let reg_id = match param {
            "Acc"  | "A" => 0,
            "Bacc" | "B" => 1,
            "Carr" | "C" => 2,
            "Datt" | "D" => 3,
            "E" => 4,
            "F" => 5,
            "G" => 6,
            "H" => 7,
            _ => return Err(format!("Invalid register: {param}"))
        };

        Ok( Register { reg_id } )

    }
}

#[derive(Debug)]
pub struct MemoryAddress {
    address: u8
}

impl MemoryAddress {
    pub fn build(param: &str) -> Result<MemoryAddress, String> {
        let address = match param {
            _ => return Err(format!("Invalid memory address: {param}"))
        };
    }
}

#[derive(Debug)]
pub struct Immediate {
    literal: u8
}

impl Immediate {
    pub fn build(param: &str) -> Result<Immediate, String> {
        let literal = match param {
            _ => return Err(format!("Invalid immediate literal: {param}"))
        };
    }
}