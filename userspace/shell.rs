use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::fs;

const PROFILE_PATH: &str = "/home/.vantis_system_profile.conf";

fn read_profile_config() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let text = match fs::read_to_string(PROFILE_PATH) {
        Ok(content) => content,
        Err(_) => return map,
    };

    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let k = key.trim();
            let v = value.trim();
            if !k.is_empty() {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }
    map
}

fn write_profile_config(mut map: BTreeMap<String, String>) -> Result<(), String> {
    if !map.contains_key("profile") {
        map.insert("profile".to_string(), "core".to_string());
    }
    if !map.contains_key("user") {
        map.insert("user".to_string(), "vantis".to_string());
    }
    if !map.contains_key("hostname") {
        map.insert("hostname".to_string(), "vantis-node".to_string());
    }
    if !map.contains_key("mode") {
        map.insert("mode".to_string(), "installed".to_string());
    }

    let order = [
        "profile",
        "user",
        "hostname",
        "mode",
        "first_boot_unix_utc",
        "notes",
    ];
    let mut out = String::new();
    for key in order {
        if let Some(value) = map.get(key) {
            out.push_str(key);
            out.push('=');
            out.push_str(value);
            out.push('\n');
        }
    }
    for (key, value) in &map {
        if !order.contains(&key.as_str()) {
            out.push_str(key);
            out.push('=');
            out.push_str(value);
            out.push('\n');
        }
    }

    if let Err(err) = fs::create_dir_all("/home") {
        return Err(format!("failed to prepare /home: {err}"));
    }
    fs::write(PROFILE_PATH, out).map_err(|err| format!("failed to write profile config: {err}"))
}

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
                println!("  firstboot                  - show first-boot setup status");
                println!("  config show                - print current installed profile config");
                println!("  config set <k> <v>         - set profile/user/hostname");
                println!("  reboot                     - reboot machine");
                println!("  exit                       - end current shell session");
            }
            "ai" => match Command::new("vantis").arg("ai").output() {
                Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
                Err(err) => eprintln!("failed to run vantis: {err}"),
            },
            "firstboot" => {
                let marker = "/home/.vantis_first_boot_done";
                if Path::new(marker).exists() {
                    println!("first_boot: done");
                } else {
                    println!("first_boot: pending");
                }

                match fs::read_to_string("/home/.vantis_welcome.txt") {
                    Ok(content) => {
                        let line = content.lines().next().unwrap_or_default();
                        if !line.is_empty() {
                            println!("welcome: {line}");
                        }
                    }
                    Err(_) => println!("welcome: n/a"),
                }
            }
            "config" => {
                let Some(subcmd) = parts.next() else {
                    eprintln!("usage: config show | config set <key> <value>");
                    continue;
                };

                match subcmd {
                    "show" => {
                        let map = read_profile_config();
                        if map.is_empty() {
                            println!("config: n/a");
                        } else {
                            for (key, value) in map {
                                println!("{key}={value}");
                            }
                        }
                    }
                    "set" => {
                        let Some(key) = parts.next() else {
                            eprintln!("usage: config set <profile|user|hostname> <value>");
                            continue;
                        };
                        let value = parts.collect::<Vec<_>>().join(" ");
                        if value.trim().is_empty() {
                            eprintln!("usage: config set <profile|user|hostname> <value>");
                            continue;
                        }
                        if key != "profile" && key != "user" && key != "hostname" {
                            eprintln!("config key not supported: {key}");
                            continue;
                        }
                        if key == "profile" {
                            let allowed = ["core", "gamer", "wraith", "server"];
                            if !allowed.contains(&value.as_str()) {
                                eprintln!("profile must be one of: core, gamer, wraith, server");
                                continue;
                            }
                        }

                        let mut map = read_profile_config();
                        map.insert(key.to_string(), value.clone());
                        match write_profile_config(map) {
                            Ok(_) => println!("config updated: {key}={value}"),
                            Err(err) => eprintln!("{err}"),
                        }
                    }
                    _ => eprintln!("usage: config show | config set <key> <value>"),
                }
            }
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
