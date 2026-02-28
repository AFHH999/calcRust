use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Data {
    num1: f64,
    op: char,
    num2: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct History {
    #[serde(flatten)]
    data: Data,
    result: f64,
}

const PATH_FILE: &str = "history.txt";

fn write(json_str: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(PATH_FILE)?;

    writeln!(file, "{}", json_str)?;
    Ok(())
}

fn get_int<R: BufRead, W: Write>(
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

fn get_float<R: BufRead, W: Write>(
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

fn get_op<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt: &str,
) -> Result<char, Error> {
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
        let op = trimmed.chars().next().unwrap();

        match op {
            '-' | '+' | '*' | '/' => return Ok(op),
            _ => {
                writeln!(writer, "Invalid character, use /, -, +, *")?;
            }
        }
    }
}

fn calculation(data: &Data) -> Result<f64, String> {
    match data.op {
        '+' => Ok(data.num1 + data.num2),
        '-' => Ok(data.num1 - data.num2),
        '*' => Ok(data.num1 * data.num2),
        '/' => {
            if data.num2.abs() < f64::EPSILON {
                Err("You can't divide by zero!".to_string())
            } else {
                Ok(data.num1 / data.num2)
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}

fn read() -> std::io::Result<()> {
    let file = File::open(PATH_FILE)?;
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

fn rm_file_history() -> std::io::Result<()> {
    std::fs::remove_file(PATH_FILE)?;
    println!("File delete successfully!");
    Ok(())
}

fn format_result(num1: f64, op: char, num2: f64, result: f64) -> String {
    format!("{} {} {} = {}", num1, op, num2, result)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();

    loop {
        let result = get_int(
            &mut reader,
            &mut stdout,
            "
            Please enter;
            1- To use calculator
            2- To see the history
            3- To delete the file
            4 - To exit
            ",
        );

        let menu = match result {
            Ok(value) => value,
            Err(e) => {
                eprintln!("system I/O error: {}", e);
                break;
            }
        };

        match menu {
            1 => {
                let num1 = get_float(&mut reader, &mut stdout, "Please insert the first number: ")?;
                let num2 = get_float(
                    &mut reader,
                    &mut stdout,
                    "Please insert the second number: ",
                )?;
                let op = get_op(&mut reader, &mut stdout, "Please enter the operation: ")?;

                let data = Data { num1, num2, op };

                match calculation(&data) {
                    Ok(result) => {
                        println!("{}", format_result(num1, op, num2, result));

                        let history = History { data, result };

                        let json_str = serde_json::to_string(&history).unwrap();
                        write(&json_str)?;
                    }
                    Err(msg) => println!("{}", msg),
                }
            }
            2 => match read() {
                Ok(_) => {}
                Err(e) => println!("Read failed: {}", e),
            },

            3 => {
                rm_file_history()?;
            }

            4 => {
                println!("Until next time!");
                break;
            }

            _ => println!("Invalid option, please try again!"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let data = Data {
            num1: 2.0,
            op: '+',
            num2: 3.0,
        };
        assert_eq!(calculation(&data), Ok(5.0));
    }

    #[test]
    fn test_multiplication() {
        let data = Data {
            num1: 2.5,
            op: '*',
            num2: 3.5,
        };
        assert_eq!(calculation(&data), Ok(8.75));
    }

    #[test]
    fn test_subtraction() {
        let data = Data {
            num1: 6.0,
            op: '-',
            num2: 3.0,
        };
        assert_eq!(calculation(&data), Ok(3.0));
    }

    #[test]
    fn test_division_by_zero() {
        let data = Data {
            num1: 8.0,
            op: '/',
            num2: 0.0,
        };
        assert!(calculation(&data).is_err());
    }

    #[test]
    fn test_division() {
        let data = Data {
            num1: 8.0,
            op: '/',
            num2: 2.0,
        };
        assert_eq!(calculation(&data), Ok(4.0));
    }

    #[test]
    fn test_format_result() {
        assert_eq!(format_result(2.0, '+', 3.0, 5.0), "2 + 3 = 5");
    }
}
