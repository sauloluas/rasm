use std::collections::HashMap;

#[derive(Debug)]
pub struct Overroot {
    constants: HashMap<String, String>,
    labels: HashMap<String, u16>,
    instructions: Vec<crate::Instruction>,
}

impl TryFrom<String> for Overroot {
    type Error = String;

    fn try_from(contents: String) -> Result<Self, Self::Error> {
        let mut overroot = Self {
            constants: HashMap::new(),
            labels: HashMap::new(),
            instructions: Vec::new(),
        };

        for line in contents.lines() {
            let line_is_comment = line.get(..3) == Some("///");

            if line.is_empty() || line_is_comment {
                continue;
            }

            if line.contains("::") {
                overroot.insert_label(line)?;
            } else if line.contains(":=") {
                overroot.insert_constant(line)?;
            } else {
                overroot.push_instruction(line)?;
            }
        }

        let unresolved: Vec<(usize, String)> = overroot.instructions
            .iter()
            .enumerate()
            .filter_map(|(i, inst)| {
                if let crate::Instruction::Leap(crate::Label { name, position: None }) = inst {
                    Some((i, name.clone()))
                } else {
                    None
                }
            })
            .collect();

        for (i, name) in unresolved {
            let position = overroot.labels.get(&name)
                .copied()
                .ok_or_else(|| format!("Label '{name}' not found"))?;

            overroot.instructions[i] = crate::Instruction::Leap(
                crate::Label { name, position: Some(position) }
            );
        }

        Ok(overroot)
    }
}

impl Overroot {
    fn insert_constant(&mut self, line: &str) -> Result<(), String> {
        let parts: Vec<&str> = line.split(":=").collect();

        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(format!("Invalid constant format: `{line}`"));
        }

        let name = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();

        self.constants.insert(name, value);

        Ok(())
    }

    fn push_instruction(&mut self, line: &str) -> Result<(), String> {
        let instruction_line = line
            .split_whitespace()
            .map(|token| {
                // Check if token is defined as a constant and replace it
                // If not found, keep the original token
                if self.constants.contains_key(token) {
                    self.constants[token].as_str()
                } else {
                    token
                }
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let instruction = crate::Instruction::build(instruction_line.as_str())?;

        self.instructions.push(instruction);

        Ok(())
    }

    fn insert_label(&mut self, line: &str) -> Result<(), String> {
        let label_index = self.instructions.len() as u16;

        if label_index > 0xFFF {
            return Err(format!("Label position {label_index} exceeds 12-bit limit (max 4095)"));
        }

        let label_name = line.strip_suffix("::").unwrap().to_string();

        self.labels.insert(label_name, label_index);

        Ok(())
    }

    pub fn encode(self) -> Result<Vec<String>, String> {
        self.instructions
            .iter()
            .map(|instruction| instruction.encode())
            .collect()
    }
}
