use calculator::Data;
use calculator::History;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = calculator::db::Database::new("history.db")?;
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    db.create_tables()?;

    loop {
        let result = calculator::get_int(
            &mut reader,
            &mut stdout,
            "Please enter:
            1- To use calculator
            2- To see the history
            3- To delete the file
            4 - To exit\n",
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
                        let history = History { data, result };
                        db.save_in_db(&history)?;
                    }
                    Err(msg) => println!("{}", msg),
                }
            }
            2 => match db.read_db() {
                Ok(histories) => {
                    for h in histories {
                        println!(
                            "{}",
                            calculator::format_result(
                                h.data.num1,
                                h.data.op,
                                h.data.num2,
                                h.result,
                            )
                        )
                    }
                }
                Err(e) => println!("Read failed: {}", e),
            },

            3 => match db.delete_db() {
                Ok(_) => {}
                Err(e) => return Err(Box::new(e)),
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
