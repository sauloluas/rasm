use std::collections::HashMap;

pub struct Overroot {
    constants: HashMap<String, String>,
}

impl Overroot {
    pub fn new() -> Self {
        Self {
            constants: HashMap::new(),
        }
    }

    pub fn expand_lines(&mut self, lines: &[&str]) -> Result<Vec<Vec<String>>, String> {
        self.parse_constants(lines)?;

        // Extract instruction lines (exclude directives and comments)
        let meaningful_lines: Vec<Vec<&str>> = lines
            .iter()
            .copied()
            .filter(|line| line.get(..1) != Some("!"))
            .map(|line| line.split_whitespace().collect())
            .collect();

        // Replace constant names with their defined values
        // This implements the macro-like behavior where constants are substituted
        Ok(meaningful_lines
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|token| {
                        // Check if token is defined as a constant and replace it
                        // If not found, keep the original token
                        if self.constants.contains_key(token) {
                            self.constants[token].clone()
                        } else {
                            token.to_string()
                        }
                    })
                    .collect()
            })
            .collect())
    }

    // Extract constant lines (C-like macros behavior)
    pub fn parse_constants(&mut self, lines: &[&str]) -> Result<(), String> {
        lines
            .iter()
            .copied()
            .filter(|line| line.starts_with("!"))
            .try_for_each(|line| {
                // Parse constants from directives
                // Format: !CONSTANT_NAME: value
                // Example: !MY_REG: A, !MEMORY_ADDR: 20h, !CONSTANT: 42
                let parts: Vec<&str> = line[1..].split(": ").collect(); // Skip the '!' character

                if parts.len() != 2 {
                    return Err(format!("Invalid constant format: {line}"));
                }

                let name = parts[0].to_string();
                let value = parts[1].to_string();

                self.constants.insert(name, value);

                Ok(())
            })
    }
}
