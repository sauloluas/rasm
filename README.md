# RASM - Root Assembler

A simple assembler written in Rust that converts RASM language instructions into hexadecimal machine code.

## Features

- **Instruction Set**: Supports basic RASM operations like init, copy, add, and store
- **Constant System**: Define constants using `!` directives that work like C macros
- **Multiple Number Formats**: Supports decimal and hexadecimal (with 'h' suffix) literals
- **Register Support**: 8 registers with both short and long names
- **Memory Operations**: Direct memory addressing support

## Usage

```bash
cargo run [input_file]
```

Example:
```bash
cargo run prototype.txt
```

## Assembly Language Syntax

### Comments
- Lines starting with `///` are comments and are ignored
- Empty lines are also ignored

### Variable/Constant Definitions
Lines starting with `!` define variables/constants that work like C macros:

```assembly
!VARIABLE_NAME value
```

Examples:
```assembly
!ACC_REG A
!MEMORY_ADDR 20h
!INIT_VALUE 42
!HEX_CONSTANT FFh
```

Variables are replaced with their values during compilation:
```assembly
init ACC_REG INIT_VALUE    // Becomes: init A 42
str MEMORY_ADDR ACC_REG    // Becomes: str 20h A
```

### Instructions

#### `init` - Initialize Register
Initialize a register with an immediate value.
```assembly
init <register> <immediate>
```
Examples:
```assembly
init A 10        // Initialize register A with decimal 10
init B 20h       // Initialize register B with hex 0x20
```

#### `copy` - Copy Register
Copy value from one register to another.
```assembly
copy <dest_register> <src_register>
```
Examples:
```assembly
copy B A         // Copy value from A to B
```

#### `adcp` - Add Copying
Add values between registers.
```assembly
adcp <dest_register> <src_register>
```
Examples:
```assembly
adcp A B         // Add B to A, store result in A
```

#### `str` - Store
Store register value to memory address.
```assembly
str <memory_address> <register>
```
Examples:
```assembly
str 20h A        // Store register A to memory address 0x20
str 100 B        // Store register B to memory address 100
```

### Registers

| Short Name | Long Name | ID |
|------------|-----------|----| 
| A          | Acc       | 0  |
| B          | Bacc      | 1  |
| C          | Carr      | 2  |
| D          | Datt      | 3  |
| E          | E         | 4  |
| F          | F         | 5  |
| G          | G         | 6  |
| H          | H         | 7  |

### Number Formats

- **Decimal**: Regular numbers (e.g., `42`, `100`)
- **Hexadecimal**: Numbers ending with 'h' (e.g., `20h`, `FFh`)

## Example Program

```assembly
!x: 0h
!y: 1h
!z: 2h

/// let byte x = 10
init	Acc		10

/// let byte y = 3
init	Bacc	3

/// ley byte z = x + y
copy	Carr	Acc
adcp	Carr	Bacc

/// storing values
str		x		Acc
str		y		Bacc
str		z		Carr
```

This program would generate the following machine code:
```
05000A
050103
0A0200
0B0201
070000
070101
070202
```

## Machine Code Format

Each instruction is encoded as 3 bytes in hexadecimal:
- Byte 1: Operation code
- Byte 2: First parameter
- Byte 3: Second parameter

### Operation Codes
- `05`: init (Initialize register with immediate)
- `0A`: copy (Copy between registers)
- `0B`: adcp (Add copying between registers)
- `07`: str (Store register to memory)

## Error Handling

The assembler provides helpful error messages for:
- Invalid instruction formats
- Unknown registers
- Invalid number formats
- Malformed directive syntax
- Missing files

## Building

```bash
cargo build --release
```
