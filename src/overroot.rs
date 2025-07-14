use std::collections::HashMap;

#[derive(Default)]
pub struct Overroot {
    constants: HashMap<String, String>,
}

impl Overroot {
    pub fn expand_lines(&mut self, lines: &[&str]) -> Result<Vec<Vec<String>>, String> {
        self.parse_constants(lines)?;

        // Extract instruction lines
        let meaningful_lines: Vec<Vec<&str>> = lines
            .iter()
            .copied()
            .filter(|line| !line.contains(":="))
            .map(|line| line.split_whitespace().collect())
            .collect();

        // Replace constant names with their defined values
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

    // Extract constant lines
    pub fn parse_constants(&mut self, lines: &[&str]) -> Result<(), String> {
        lines
            .iter()
            .copied()
            .filter(|line| line.contains(":="))
            .try_for_each(|line| {
                // Parse constants from directives
                // Format: !CONSTANT_NAME: value
                // Example: !MY_REG: A, !MEMORY_ADDR: 20h, !CONSTANT: 42
                let parts: Vec<&str> = line.split(":=").collect();

                if parts.len() != 2 {
                    return Err(format!("Invalid constant format: {line}"));
                }

                let name = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();

                // println!("name: {name:#?}\nvalue: {value:#?}");

                self.constants.insert(name, value);

                Ok(())
            })
    }
}
