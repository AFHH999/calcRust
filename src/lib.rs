use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::result::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Data {
    pub num1: f64,
    pub op: Operations,
    pub num2: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    #[serde(flatten)]
    pub data: Data,
    pub result: f64,
}

pub const HISTORY_FILE: &str = "history.txt";

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Operations {
    Addition,
    Division,
    Multiplication,
    Subtraction,
}

impl Operations {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operations::Addition),
            '-' => Some(Operations::Subtraction),
            '*' => Some(Operations::Multiplication),
            '/' => Some(Operations::Division),
            _ => None,
        }
    }
    pub fn as_char(&self) -> char {
        match self {
            Operations::Addition => '+',
            Operations::Subtraction => '-',
            Operations::Multiplication => '*',
            Operations::Division => '/',
        }
    }
}

pub fn convert_to_json(data: Data, result: f64) -> Result<String, serde_json::Error> {
    serde_json::to_string(&History { data, result })
}

pub fn write_history(json_str: &str, path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{}", json_str)?;
    Ok(())
}

pub fn get_int<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt: &str,
) -> Result<u32, Error> {
    let mut input = String::new();

    loop {
        writer.write_all(prompt.as_bytes())?;
        writer.flush()?;

        input.clear();

        if reader.read_line(&mut input).is_err() {
            writeln!(writer, "Input error, please try again. ")?;
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(num) => return Ok(num),
            Err(_) => {
                writeln!(writer, "Bad input please enter an int number.")?;
            }
        }
    }
}

pub fn get_float<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt: &str,
) -> Result<f64, Error> {
    let mut input = String::new();

    loop {
        writer.write_all(prompt.as_bytes())?;
        writer.flush()?;

        input.clear();

        if reader.read_line(&mut input).is_err() {
            writeln!(writer, "Input error, please try again.")?;
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(num) => return Ok(num),
            Err(_) => {
                writeln!(writer, "Bad input, please enter a decimal number!")?;
            }
        }
    }
}

pub fn get_op<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt: &str,
) -> Result<Operations, Error> {
    let mut input = String::new();
    loop {
        writer.write_all(prompt.as_bytes())?;
        writer.flush()?;

        input.clear();

        if reader.read_line(&mut input).is_err() {
            writeln!(writer, "Bad input, please enter a character only!")?;
            continue;
        }
        let trimmed = input.trim();
        if trimmed.len() != 1 {
            writeln!(writer, "Please enter one character only!")?;
            continue;
        }
        let c = trimmed.chars().next().unwrap();
        if let Some(op) = Operations::from_char(c) {
            return Ok(op);
        } else {
            writeln!(writer, "Invalid character, use /, -, +, *")?;
        }
    }
}

pub fn calculation(data: &Data) -> Result<f64, String> {
    match data.op {
        Operations::Addition => Ok(data.num1 + data.num2),
        Operations::Subtraction => Ok(data.num1 - data.num2),
        Operations::Multiplication => Ok(data.num1 * data.num2),
        Operations::Division => {
            if data.num2.abs() < f64::EPSILON {
                Err("You can't divide by zero!".to_string())
            } else {
                Ok(data.num1 / data.num2)
            }
        }
    }
}

pub fn read_history() -> std::io::Result<()> {
    let file = File::open(HISTORY_FILE)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        match serde_json::from_str::<History>(&line) {
            Ok(history) => {
                println!(
                    " {} {} {} = {}",
                    history.data.num1, history.data.op, history.data.num2, history.result
                );
            }
            Err(e) => eprintln!("Failed to parse into JSON: {}", e),
        }
    }
    Ok(())
}

pub fn delete_history() -> std::io::Result<()> {
    std::fs::remove_file(HISTORY_FILE)?;
    println!("File delete successfully!");
    Ok(())
}

pub fn format_result(num1: f64, op: Operations, num2: f64, result: f64) -> String {
    format!("{} {} {} = {}", num1, op, num2, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let data = Data {
            num1: 2.0,
            op: Operations::Addition,
            num2: 3.0,
        };
        assert_eq!(calculation(&data), Ok(5.0));
    }

    #[test]
    fn test_multiplication() {
        let data = Data {
            num1: 2.5,
            op: Operations::Multiplication,
            num2: 3.5,
        };
        assert_eq!(calculation(&data), Ok(8.75));
    }

    #[test]
    fn test_subtraction() {
        let data = Data {
            num1: 6.0,
            op: Operations::Subtraction,
            num2: 3.0,
        };
        assert_eq!(calculation(&data), Ok(3.0));
    }

    #[test]
    fn test_division_by_zero() {
        let data = Data {
            num1: 8.0,
            op: Operations::Division,
            num2: 0.0,
        };
        assert!(calculation(&data).is_err());
    }

    #[test]
    fn test_division() {
        let data = Data {
            num1: 8.0,
            op: Operations::Division,
            num2: 2.0,
        };
        assert_eq!(calculation(&data), Ok(4.0));
    }

    #[test]
    fn test_format_result() {
        assert_eq!(
            format_result(2.0, Operations::Addition, 3.0, 5.0),
            "2 + 3 = 5"
        );
    }
}
