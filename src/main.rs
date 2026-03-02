use calculator::Data;
use calculator::HISTORY_FILE;
use calculator::convert_to_json;
use std::io;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();

    loop {
        let result = calculator::get_int(
            &mut reader,
            &mut stdout,
            "Please enter:
            1- To use calculator
            2- To see the history
            3- To delete the file
            4 - To exit",
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
                let num1 = calculator::get_float(&mut reader, &mut stdout, "First number: ")?;
                let num2 = calculator::get_float(&mut reader, &mut stdout, "Second number: ")?;
                let op = calculator::get_op(&mut reader, &mut stdout, "Operation: ")?;

                let data = Data { num1, num2, op };

                match calculator::calculation(&data) {
                    Ok(result) => {
                        println!("{}", calculator::format_result(num1, op, num2, result));

                        let json_str =
                            convert_to_json(data, result).map_err(std::io::Error::other)?; //This "map_err" transform the error type

                        calculator::write_history(&json_str, HISTORY_FILE)?;
                    }
                    Err(msg) => println!("{}", msg),
                }
            }
            2 => match calculator::read_history() {
                Ok(_) => {}
                Err(e) => println!("Read failed: {}", e),
            },

            3 => match calculator::delete_history() {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    println!("There is no history to erase!");
                }
                Err(e) => return Err(e),
            },

            4 => {
                println!("Until next time!");
                break;
            }

            _ => println!("Invalid option, please try again!"),
        }
    }
    Ok(())
}
