pub enum Instruction {
    Adcp(Register, Register),
    Adl(Register, MemoryAddress),
    Copy(Register, Register),
    Init(Register, Immediate),
    Load(Register, MemoryAddress),
    Str(MemoryAddress, Register)

    
}