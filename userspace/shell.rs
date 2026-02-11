use std::io::{self, Write};
use std::process::Command;

pub fn start() {
    loop {
        print!("vantis> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let Some(cmd) = parts.next() else {
            continue;
        };

        match cmd {
            "help" => {
                println!("commands:");
                println!("  help                       - show this help");
                println!("  ai                         - run Cortex offline readiness check");
                println!("  install <disk> --yes       - install bootable VantisOS to target disk");
                println!("  reboot                     - reboot machine");
                println!("  exit                       - end current shell session");
            }
            "ai" => match Command::new("vantis").arg("ai").output() {
                Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
                Err(err) => eprintln!("failed to run vantis: {err}"),
            },
            "install" => {
                let args: Vec<&str> = parts.collect();
                if args.is_empty() {
                    eprintln!("usage: install <target-device> --yes");
                    eprintln!("example: install /dev/vda --yes");
                    continue;
                }

                match Command::new("vantis-installer").args(args).status() {
                    Ok(status) if status.success() => {}
                    Ok(status) => eprintln!("installer exited with status: {status}"),
                    Err(err) => eprintln!("failed to execute installer: {err}"),
                }
            }
            "reboot" => match Command::new("reboot").status() {
                Ok(_) => {}
                Err(err) => eprintln!("failed to reboot: {err}"),
            },
            "exit" => break,
            _ => println!("unknown command (type `help`)"),
        }
    }
}
