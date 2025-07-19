pub mod overroot;

pub use overroot::Overroot;

#[derive(Debug)]
pub enum Instruction {
    Adcp(Register, Register),      // Add copying
    Adl(Register, MemoryAddress),  // Add loading
    Asn(MemoryAddress, Immediate), // Assign
    Copy(Register, Register),      // Copy
    Init(Register, Immediate),     // Initialize
    Load(Register, MemoryAddress), // Load
    Str(MemoryAddress, Register),  // Store
}

impl Instruction {
    pub fn build(line: &str) -> Result<Instruction, String> {
        let line: Vec<&str> = line.split_whitespace().collect();

        let operation = line[0];
        let param1 = line[1];
        let param2 = line[2];

        let inst = match operation {
            "init" => Self::Init(Register::build(param1)?, Immediate::build(param2)?),
            "copy" => Self::Copy(Register::build(param1)?, Register::build(param2)?),
            "adcp" => Self::Adcp(Register::build(param1)?, Register::build(param2)?),
            "str" => Self::Str(MemoryAddress::build(param1)?, Register::build(param2)?),
            _ => return Err(format!("Invalid operation: {operation}")),
        };

        Ok(inst)
    }

    pub fn encode(&self) -> Result<String, String> {
        let code = match self {
            Self::Init(param1, param2) => [5, param1.reg_id, param2.literal],
            Self::Copy(register1, register2) => [10, register1.reg_id, register2.reg_id],
            Self::Adcp(register1, register2) => [11, register1.reg_id, register2.reg_id],
            Self::Str(memaddr, register) => [7, memaddr.address, register.reg_id],
            _ => {
                return Err(format!("Operation {:?} not implemented yet!", self));
            }
        };

        Ok(code.map(|byte| format!("{byte:02X}")).join(""))
    }
}

#[derive(Debug)]
pub struct Register {
    reg_id: u8,
}

impl Register {
    pub fn build(param: &str) -> Result<Register, String> {
        let reg_id = match param {
            "Acc" | "A" => 0,
            "Bacc" | "B" => 1,
            "Carr" | "C" => 2,
            "Datt" | "D" => 3,
            "E" => 4,
            "F" => 5,
            "G" => 6,
            "H" => 7,
            _ => return Err(format!("Invalid register: {param}")),
        };

        Ok(Register { reg_id })
    }
}

#[derive(Debug)]
pub struct MemoryAddress {
    address: u8,
}

impl MemoryAddress {
    pub fn build(param: &str) -> Result<MemoryAddress, String> {
        Immediate::build(param)
            .map(|immediate| MemoryAddress {
                address: immediate.literal,
            })
            .map_err(|_| format!("Invalid memory address: {param}"))
    }
}

#[derive(Debug)]
pub struct Immediate {
    literal: u8,
}

impl Immediate {
    pub fn build(param: &str) -> Result<Immediate, String> {
        let literal = if param.ends_with('h') {
            u8::from_str_radix(param.strip_suffix('h').unwrap(), 16).map_err(|e| {
                format!("Invalid hexadecimal immediate literal: {param}, error: {e}")
            })?
        } else {
            param
                .parse::<u8>()
                .map_err(|e| format!("Invalid decimal immediate literal: {param}, error: {e}"))?
        };

        Ok(Immediate { literal })
    }
}
