use std::io::{self, Write};
use std::process::Command;

pub fn start() {
    loop {
        print!("vantis> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "ai" => {
                match Command::new("vantis").arg("ai").output() {
                    Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
                    Err(err) => eprintln!("failed to run vantis: {err}"),
                }
            }
            "exit" => break,
            _ => println!("unknown command"),
        }
    }
}
