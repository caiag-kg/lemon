use crate::models::YesOrNo;
use std::io::{stdin, stdout, Error as IOError, Write};

impl YesOrNo {
    pub fn ask(question: &str, clarify: bool) -> Result<bool, IOError> {
        // Function to get input result in Yes or No;
        match Self::re_ask(&format!("\n{question}"))? {
            true => match clarify {
                true => Self::re_ask(&format!("\n[CLARIFY] {question}")),
                false => Ok(true),
            },
            false => Ok(false),
        }
    }

    pub fn re_ask(question: &str) -> Result<bool, IOError> {
        let mut result = false;
        let mut input = String::new();
        let mut reloop = true;

        println!("{question}");
        print!("[y/n] default: No (press enter) >> ");

        while reloop {
            stdout().flush()?;
            stdin().read_line(&mut input)?;
            input = input.trim().to_lowercase();
            if !input.is_empty() {
                match input.as_str() {
                    "y" | "yes" => {
                        result = true;
                        reloop = false;
                    }
                    "n" | "no" | "" => {
                        reloop = false;
                    }
                    _ => {
                        input.clear();
                        println!("Invalid input");
                        print!("[y/n] default: No (press enter) >> ");
                    }
                }
            } else {
                reloop = false;
            }
        }

        Ok(result)
    }

    pub fn value_as_string(&self) -> Result<String, IOError> {
        match self {
            YesOrNo::Yes => Ok("Yes".to_string()),
            YesOrNo::No => Ok("No".to_string()),
        }
    }
}
