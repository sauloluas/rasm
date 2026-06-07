pub mod overroot;

pub use overroot::Overroot;

#[derive(Debug)]
pub enum Instruction {
    Add(Register, Register, Register), // Add
    Init(Register, Immediate),         // Initialize
    Str(MemoryAddress, Register),      // Store
    Leap(Label),                       // Leap
}

impl Instruction {
    pub fn build(line: &str) -> Result<Instruction, String> {
        let line: Vec<&str> = line.split_whitespace().collect();

        let operation = line[0];
        let param1 = line[1];
        let param2 = line.get(2);
        let param3 = line.get(3);

        Ok(if let (Some(param2), Some(param3)) = (param2, param3) {
            match operation {
                "add" => Self::Add(
                    Register::build(param1)?,
                    Register::build(param2)?,
                    Register::build(param3)?,
                ),
                _ => return Err(format!("Invalid operation: {operation}")),
            }
        } else if let Some(param2) = param2 {
            match operation {
                "init" => Self::Init(Register::build(param1)?, Immediate::build(param2)?),
                "copy" => Self::Add(
                    Register::build(param1)?,
                    Register::build(param2)?,
                    Register::zero(),
                ),
                "str" => Self::Str(MemoryAddress::build(param1)?, Register::build(param2)?),
                _ => return Err(format!("Invalid operation: {operation}")),
            }
        } else {
            match operation {
                "lp" => {
                    let label_name = format!("{}::", param1);
                    Self::Leap(Label::build(&label_name, None)?)
                }
                _ => return Err(format!("Invalid operation: {operation}")),
            }
        })
    }

    pub fn encode(&self) -> Result<String, String> {
        let word: u16 = match self {
            Self::Init(reg, imm) => 0xA000 | reg.bits() << 8 | imm.bits(),
            Self::Add(r1, r2, r3) => 0x0000 | r1.bits() << 8 | r2.bits() << 4 | r3.bits(),
            Self::Str(addr, reg) => 0xC000 | addr.bits() << 4 | reg.bits(),
            Self::Leap(label) => 0x7000 | label.bits()?,
            _ => return Err(format!("Operation {:?} not implemented yet!", self)),
        };

        Ok(format!("{word:04X}"))
    }
}

#[derive(Debug)]
pub struct Register {
    reg_id: u8,
}

impl Register {
    const ZERO: u8 = 0x0;

    pub fn build(param: &str) -> Result<Register, String> {
        let reg_id = match param {
            "z" | "zero" | "r0" => Self::ZERO,

            // general purpose
            "Acc" | "A" | "ra" => 0x1,
            "Bacc" | "B" | "rb" => 0x2,
            "Carr" | "C" | "rc" => 0x3,
            "Datt" | "D" | "rd" => 0x4,

            // index registers
            "i" => 0x5,
            "j" => 0x6,
            "k" => 0x7,
            "l" => 0x8,

            // pointer registers
            "p" => 9,
            "q" => 0xA,
            "r" => 0xB,
            "s" => 0xC,

            // temporary registers
            "t" => 0xD,
            "u" => 0xE,
            "v" => 0xF,

            _ => return Err(format!("Invalid register: {param}")),
        };

        Ok(Register { reg_id })
    }

    pub fn zero() -> Self {
        Self { reg_id: Self::ZERO }
    }

    fn bits(&self) -> u16 {
        self.reg_id as u16
    }
}

#[derive(Debug)]
pub struct Label {
    name: String,
    position: Option<u16>,
}

impl Label {
    pub fn build(param: &str, position: Option<u16>) -> Result<Label, String> {
        match param.strip_suffix("::") {
            Some(name) => {
                if name.is_empty() {
                    return Err("Label name cannot be empty".to_string());
                }

                Ok(Self {
                    name: name.to_string(),
                    position,
                })
            }
            None => Err(format!("Label must end with '::' but got: '{param}'")),
        }
    }

    fn bits(&self) -> Result<u16, String> {
        self.position
            .ok_or_else(|| format!("Label '{}' has no resolved position", self.name))
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

    fn bits(&self) -> u16 {
        self.literal as u16
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

    fn bits(&self) -> u16 {
        self.address as u16
    }
}
