pub mod db;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::BufRead;
use std::io::Error;
use std::io::Write;
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

    #[test]
    fn test_from_char_invalid() {
        // Testing random input be dealt correctly!
        assert_eq!(Operations::from_char('?'), None);
        assert_eq!(Operations::from_char(' '), None);
        assert_eq!(Operations::from_char('\n'), None);
    }

    #[test]
    fn test_from_char_valid() {
        assert_eq!(Operations::from_char('+'), Some(Operations::Addition));
    }

    #[test]
    fn test_get_op_recovery() {
        let input_data = "++\n \nz\n*\n";
        let mut input = std::io::Cursor::new(input_data);
        let mut output = Vec::new();

        let result = get_op(&mut input, &mut output, "Op: ");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Operations::Multiplication);
    }

    #[test]
    fn test_get_float_recovery() {
        let input_data = "104.53\n";
        let mut input = std::io::Cursor::new(input_data);
        let mut output = Vec::new();

        let result = get_float(&mut input, &mut output, "Enter num: ");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 104.53);
    }

    #[test]
    fn test_get_int_recovery() {
        let input_data = "150\n";
        let mut input = std::io::Cursor::new(input_data);
        let mut output = Vec::new();

        let result = get_int(&mut input, &mut output, "Enter num: ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 150);
    }

    #[test]
    fn test_get_int_chaos() {
        let mut input = std::io::Cursor::new("zbe\n10.5\n100");
        let mut output = Vec::new();

        let result = get_int(&mut input, &mut output, "Enter an int number: ");

        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn test_get_float_chaos() {
        let mut input = std::io::Cursor::new("abc\n44.94");
        let mut output = Vec::new();

        let result = get_float(&mut input, &mut output, "Enter float number: ");

        assert_eq!(result.unwrap(), 44.94);
    }
}
