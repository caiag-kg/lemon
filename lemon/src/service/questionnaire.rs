use std::io::{stdin, stdout, Write, Error as IOError};
use crate::models::YesOrNo;

pub fn yes_or_no(question: &str) -> Result<YesOrNo, IOError> {
    // Function to get input result in Yes or No;
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
                },
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
    
    match result {
        true => {
            Ok(YesOrNo::Yes)
        }
        false => {
            Ok(YesOrNo::No)
        }
    }
}

impl YesOrNo {
    pub fn value_as_string(&self) -> Result<String, IOError> {
        match self {
            YesOrNo::Yes => {
                Ok("Yes".to_string())
            }
            YesOrNo::No => {
                Ok("No".to_string())
            }
        }
    }
}