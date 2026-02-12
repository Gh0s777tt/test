use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const PROFILE_PATH: &str = "/home/.vantis_system_profile.conf";
const WELCOME_PATH: &str = "/home/.vantis_welcome.txt";
const FIRSTBOOT_MARKER_PATH: &str = "/home/.vantis_first_boot_done";
const ONBOARDING_DONE_PATH: &str = "/home/.vantis_onboarding_done";
const ONBOARDING_PENDING_PATH: &str = "/home/.vantis_onboarding_pending";
const ONBOARDING_BACKUP_DEFAULT_PATH: &str = "/home/.vantis_onboarding_backup.conf";
const ONBOARD_USAGE: &str = "usage: onboard [--hostname <name>] [--user <name>] [--profile <name>] | onboard status | onboard reset --yes | onboard export [path] | onboard import [path]";

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn ensure_profile_defaults(map: &mut BTreeMap<String, String>) {
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
}

fn serialize_profile_config(map: &BTreeMap<String, String>) -> String {
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
    for (key, value) in map {
        if !order.iter().any(|known| *known == key.as_str()) {
            out.push_str(key);
            out.push('=');
            out.push_str(value);
            out.push('\n');
        }
    }
    out
}

fn parse_profile_config_text(text: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
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

fn read_profile_config() -> BTreeMap<String, String> {
    let text = match fs::read_to_string(PROFILE_PATH) {
        Ok(content) => content,
        Err(_) => return BTreeMap::new(),
    };
    parse_profile_config_text(&text)
}

fn write_profile_config(mut map: BTreeMap<String, String>) -> Result<(), String> {
    ensure_profile_defaults(&mut map);
    let out = serialize_profile_config(&map);

    if let Err(err) = fs::create_dir_all("/home") {
        return Err(format!("failed to prepare /home: {err}"));
    }
    fs::write(PROFILE_PATH, &out).map_err(|err| format!("failed to write profile config: {err}"))?;

    if Path::new("/persist").exists() {
        let _ = fs::create_dir_all("/persist/vantis");
        let _ = fs::write("/persist/vantis/system_profile.conf", out);
    }
    Ok(())
}

fn write_welcome_message(map: &BTreeMap<String, String>) {
    let user = map
        .get("user")
        .cloned()
        .unwrap_or_else(|| "vantis".to_string());
    let hostname = map
        .get("hostname")
        .cloned()
        .unwrap_or_else(|| "vantis-node".to_string());
    let profile = map
        .get("profile")
        .cloned()
        .unwrap_or_else(|| "core".to_string());
    let first_boot = map
        .get("first_boot_unix_utc")
        .map(|value| format!("First boot unix timestamp: {value}\n"))
        .unwrap_or_default();
    let text = format!(
        "Welcome to VantisOS installed mode.\n{first_boot}User: {user}\nHost: {hostname}\nProfile: {profile}\n"
    );

    let _ = fs::write(WELCOME_PATH, &text);
    if Path::new("/persist").exists() {
        let _ = fs::create_dir_all("/persist/vantis");
        let _ = fs::write("/persist/vantis/welcome.txt", text);
    }
}

fn normalize_profile(value: &str) -> Option<String> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "core" | "gamer" | "wraith" | "server" => Some(normalized),
        _ => None,
    }
}

fn normalize_identifier(value: &str) -> Option<String> {
    let normalized = value.trim();
    if normalized.is_empty() || normalized.len() > 63 {
        return None;
    }
    if normalized
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.')
    {
        Some(normalized.to_string())
    } else {
        None
    }
}

fn apply_profile_update(map: &mut BTreeMap<String, String>, key: &str, value: &str) -> Result<(), String> {
    match key {
        "profile" => {
            let Some(normalized) = normalize_profile(value) else {
                return Err("profile must be one of: core, gamer, wraith, server".to_string());
            };
            map.insert("profile".to_string(), normalized);
            Ok(())
        }
        "user" | "hostname" => {
            let Some(normalized) = normalize_identifier(value) else {
                return Err(format!(
                    "{key} must use only [A-Za-z0-9._-] and be 1..63 chars"
                ));
            };
            map.insert(key.to_string(), normalized);
            Ok(())
        }
        _ => Err(format!("config key not supported: {key}")),
    }
}

fn read_marker_field(path: &str, key: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn mark_onboarding_pending(source: &str) {
    let stamp = format!("updated_unix_utc={}\nsource={source}\n", unix_now());
    let _ = fs::remove_file(ONBOARDING_DONE_PATH);
    let _ = fs::write(ONBOARDING_PENDING_PATH, &stamp);
    if Path::new("/persist").exists() {
        let _ = fs::create_dir_all("/persist/vantis");
        let _ = fs::remove_file("/persist/vantis/onboarding_done");
        let _ = fs::write("/persist/vantis/onboarding_pending", stamp);
    }
}

fn mark_onboarding_done(source: &str) {
    let stamp = format!("completed_unix_utc={}\nsource={source}\n", unix_now());
    let _ = fs::remove_file(ONBOARDING_PENDING_PATH);
    let _ = fs::write(ONBOARDING_DONE_PATH, &stamp);
    if Path::new("/persist").exists() {
        let _ = fs::create_dir_all("/persist/vantis");
        let _ = fs::remove_file("/persist/vantis/onboarding_pending");
        let _ = fs::write("/persist/vantis/onboarding_done", stamp);
    }
}

fn resolve_backup_path(path_arg: Option<&str>) -> Result<String, String> {
    let path = path_arg.unwrap_or(ONBOARDING_BACKUP_DEFAULT_PATH).trim();
    if path.is_empty() {
        return Err("backup path cannot be empty".to_string());
    }
    Ok(path.to_string())
}

fn onboard_export(path_arg: Option<&str>) -> Result<(), String> {
    let path = resolve_backup_path(path_arg)?;
    let mut map = read_profile_config();
    ensure_profile_defaults(&mut map);
    let out = serialize_profile_config(&map);

    if let Some(parent) = Path::new(&path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to prepare backup directory: {err}"))?;
        }
    }
    fs::write(&path, out).map_err(|err| format!("failed to export onboarding backup: {err}"))?;
    println!("[VANTIS] ONBOARDING EXPORTED: {path}");
    Ok(())
}

fn onboard_import(path_arg: Option<&str>) -> Result<(), String> {
    let path = resolve_backup_path(path_arg)?;
    let content =
        fs::read_to_string(&path).map_err(|err| format!("failed to read onboarding backup: {err}"))?;
    let mut map = parse_profile_config_text(&content);
    if map.is_empty() {
        return Err("onboarding backup is empty or invalid".to_string());
    }
    ensure_profile_defaults(&mut map);
    map.insert("mode".to_string(), "installed".to_string());

    let profile = map
        .get("profile")
        .cloned()
        .ok_or_else(|| "missing profile in onboarding backup".to_string())?;
    let user = map
        .get("user")
        .cloned()
        .ok_or_else(|| "missing user in onboarding backup".to_string())?;
    let hostname = map
        .get("hostname")
        .cloned()
        .ok_or_else(|| "missing hostname in onboarding backup".to_string())?;
    apply_profile_update(&mut map, "profile", &profile)?;
    apply_profile_update(&mut map, "user", &user)?;
    apply_profile_update(&mut map, "hostname", &hostname)?;

    write_profile_config(map.clone())?;
    write_welcome_message(&map);
    mark_onboarding_done("import");
    println!("[VANTIS] ONBOARDING IMPORTED: {path}");
    println!("profile={profile}");
    println!("user={user}");
    println!("hostname={hostname}");
    Ok(())
}

fn prompt_with_default(prompt: &str, default: &str) -> Result<String, String> {
    print!("{prompt} [{default}]: ");
    io::stdout()
        .flush()
        .map_err(|err| format!("failed to flush prompt: {err}"))?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|err| format!("failed to read input: {err}"))?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn run_onboarding(args: Vec<&str>) -> Result<(), String> {
    if args.first() == Some(&"status") {
        let mut map = read_profile_config();
        ensure_profile_defaults(&mut map);
        if Path::new(ONBOARDING_DONE_PATH).exists() {
            println!("onboarding_state=done");
            let source = read_marker_field(ONBOARDING_DONE_PATH, "source")
                .unwrap_or_else(|| "unknown".to_string());
            println!("onboarding_source={source}");
        } else {
            println!("onboarding_state=pending");
            let source = read_marker_field(ONBOARDING_PENDING_PATH, "source")
                .unwrap_or_else(|| "unknown".to_string());
            println!("onboarding_source={source}");
        }
        let profile = map.get("profile").cloned().unwrap_or_default();
        let user = map.get("user").cloned().unwrap_or_default();
        let hostname = map.get("hostname").cloned().unwrap_or_default();
        println!("profile={profile}");
        println!("user={user}");
        println!("hostname={hostname}");
        return Ok(());
    }

    if args.first() == Some(&"reset") {
        if args.len() != 2 || args[1] != "--yes" {
            return Err("usage: onboard reset --yes".to_string());
        }
        mark_onboarding_pending("manual_reset");
        println!("[VANTIS] ONBOARDING RESET");
        return Ok(());
    }

    if args.first() == Some(&"export") {
        if args.len() > 2 {
            return Err(ONBOARD_USAGE.to_string());
        }
        return onboard_export(args.get(1).copied());
    }

    if args.first() == Some(&"import") {
        if args.len() > 2 {
            return Err(ONBOARD_USAGE.to_string());
        }
        return onboard_import(args.get(1).copied());
    }

    let mut map = read_profile_config();
    ensure_profile_defaults(&mut map);

    let mut source = "interactive";
    if args.is_empty() {
        println!("[VANTIS] onboarding wizard started");
        let default_hostname = map
            .get("hostname")
            .cloned()
            .unwrap_or_else(|| "vantis-node".to_string());
        let default_user = map
            .get("user")
            .cloned()
            .unwrap_or_else(|| "vantis".to_string());
        let default_profile = map
            .get("profile")
            .cloned()
            .unwrap_or_else(|| "core".to_string());

        let hostname = prompt_with_default("hostname", &default_hostname)?;
        let user = prompt_with_default("user", &default_user)?;
        let profile = prompt_with_default("profile (core|gamer|wraith|server)", &default_profile)?;

        apply_profile_update(&mut map, "hostname", &hostname)?;
        apply_profile_update(&mut map, "user", &user)?;
        apply_profile_update(&mut map, "profile", &profile)?;
    } else {
        source = "non_interactive";
        let mut idx = 0usize;
        let mut changed = false;
        while idx < args.len() {
            match args[idx] {
                "--hostname" => {
                    if idx + 1 >= args.len() {
                        return Err(ONBOARD_USAGE.to_string());
                    }
                    apply_profile_update(&mut map, "hostname", args[idx + 1])?;
                    changed = true;
                    idx += 2;
                }
                "--user" => {
                    if idx + 1 >= args.len() {
                        return Err(ONBOARD_USAGE.to_string());
                    }
                    apply_profile_update(&mut map, "user", args[idx + 1])?;
                    changed = true;
                    idx += 2;
                }
                "--profile" => {
                    if idx + 1 >= args.len() {
                        return Err(ONBOARD_USAGE.to_string());
                    }
                    apply_profile_update(&mut map, "profile", args[idx + 1])?;
                    changed = true;
                    idx += 2;
                }
                "--help" | "-h" => {
                    return Err(ONBOARD_USAGE.to_string());
                }
                other => {
                    return Err(format!("unknown onboarding option: {other}"));
                }
            }
        }
        if !changed {
            return Err(ONBOARD_USAGE.to_string());
        }
    }

    write_profile_config(map.clone())?;
    write_welcome_message(&map);
    mark_onboarding_done(source);

    let profile = map.get("profile").cloned().unwrap_or_default();
    let user = map.get("user").cloned().unwrap_or_default();
    let hostname = map.get("hostname").cloned().unwrap_or_default();
    println!("[VANTIS] ONBOARDING COMPLETE");
    println!("profile={profile}");
    println!("user={user}");
    println!("hostname={hostname}");
    Ok(())
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
                println!("  onboard [flags]            - run first-boot onboarding wizard");
                println!("      flags: --hostname --user --profile");
                println!("      extras: onboard status | onboard reset --yes");
                println!("              onboard export [path] | onboard import [path]");
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
                if Path::new(FIRSTBOOT_MARKER_PATH).exists() {
                    println!("first_boot: done");
                } else {
                    println!("first_boot: pending");
                }
                if Path::new(ONBOARDING_DONE_PATH).exists() {
                    println!("onboarding: done");
                } else {
                    println!("onboarding: pending");
                }

                match fs::read_to_string(WELCOME_PATH) {
                    Ok(content) => {
                        let line = content.lines().next().unwrap_or_default();
                        if !line.is_empty() {
                            println!("welcome: {line}");
                        }
                    }
                    Err(_) => println!("welcome: n/a"),
                }
            }
            "onboard" => {
                let args: Vec<&str> = parts.collect();
                if let Err(err) = run_onboarding(args) {
                    eprintln!("{err}");
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
                        let mut map = read_profile_config();
                        ensure_profile_defaults(&mut map);
                        if let Err(err) = apply_profile_update(&mut map, key, &value) {
                            eprintln!("{err}");
                            continue;
                        }
                        match write_profile_config(map) {
                            Ok(_) => {
                                let refreshed = read_profile_config();
                                write_welcome_message(&refreshed);
                                mark_onboarding_done("config_set");
                                println!("config updated: {key}={value}");
                            }
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
