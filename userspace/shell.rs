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
                let out = Command::new("vantis")
                    .arg("ai")
                    .output()
                    .unwrap();
                println!("{}", String::from_utf8_lossy(&out.stdout));
            }
            "exit" => break,
            _ => println!("unknown command"),
        }
    }
}
