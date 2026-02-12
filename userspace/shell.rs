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
const ONBOARDING_ENCRYPTED_BACKUP_DEFAULT_PATH: &str = "/home/.vantis_onboarding_backup.enc";
const ONBOARD_USAGE: &str = "usage: onboard [--hostname <name>] [--user <name>] [--profile <name>] | onboard status | onboard reset --yes | onboard export [path] | onboard import [path] | onboard export-encrypted [path] --pass <password> | onboard import-encrypted [path] --pass <password>";

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

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn derive_stream_seed(password: &str, salt: u64) -> u64 {
    let mut key_material = password.as_bytes().to_vec();
    key_material.extend_from_slice(format!(":{salt:016x}").as_bytes());
    let mut seed = fnv1a64(&key_material) ^ salt.rotate_left(17);
    if seed == 0 {
        seed = 0x9e37_79b9_7f4a_7c15;
    }
    seed
}

fn xor_crypt_bytes(input: &[u8], password: &str, salt: u64) -> Vec<u8> {
    let mut state = derive_stream_seed(password, salt);
    let mut out = Vec::with_capacity(input.len());
    for byte in input {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let key_byte = (state as u8) ^ ((state >> 16) as u8) ^ ((state >> 40) as u8);
        out.push(*byte ^ key_byte);
    }
    out
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn decode_hex(text: &str) -> Result<Vec<u8>, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    if trimmed.len() % 2 != 0 {
        return Err("hex payload has odd length".to_string());
    }
    let mut out = Vec::with_capacity(trimmed.len() / 2);
    let bytes = trimmed.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let chunk = std::str::from_utf8(&bytes[i..i + 2]).map_err(|_| "hex parse failed".to_string())?;
        let value = u8::from_str_radix(chunk, 16).map_err(|_| "hex parse failed".to_string())?;
        out.push(value);
        i += 2;
    }
    Ok(out)
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

fn resolve_encrypted_backup_path(path_arg: Option<&str>) -> Result<String, String> {
    let path = path_arg
        .unwrap_or(ONBOARDING_ENCRYPTED_BACKUP_DEFAULT_PATH)
        .trim();
    if path.is_empty() {
        return Err("encrypted backup path cannot be empty".to_string());
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

fn parse_encrypted_backup_args(args: &[&str]) -> Result<(Option<String>, String), String> {
    let mut path: Option<String> = None;
    let mut pass: Option<String> = None;
    let mut idx = 1usize;
    while idx < args.len() {
        match args[idx] {
            "--pass" => {
                if idx + 1 >= args.len() {
                    return Err(ONBOARD_USAGE.to_string());
                }
                pass = Some(args[idx + 1].to_string());
                idx += 2;
            }
            token if token.starts_with("--") => {
                return Err(ONBOARD_USAGE.to_string());
            }
            token => {
                if path.is_some() {
                    return Err(ONBOARD_USAGE.to_string());
                }
                path = Some(token.to_string());
                idx += 1;
            }
        }
    }
    let Some(pass) = pass else {
        return Err(ONBOARD_USAGE.to_string());
    };
    if pass.is_empty() {
        return Err("password cannot be empty".to_string());
    }
    Ok((path, pass))
}

fn onboard_export_encrypted(args: &[&str]) -> Result<(), String> {
    let (path_arg, pass) = parse_encrypted_backup_args(args)?;
    let path = resolve_encrypted_backup_path(path_arg.as_deref())?;
    let mut map = read_profile_config();
    ensure_profile_defaults(&mut map);
    let plaintext = serialize_profile_config(&map);

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let salt = (nanos as u64) ^ ((nanos >> 64) as u64) ^ 0x7f4a_7c15_d1b5_c3e9;
    let checksum = fnv1a64(plaintext.as_bytes());
    let ciphertext = xor_crypt_bytes(plaintext.as_bytes(), &pass, salt);
    let payload = format!(
        "VANTIS_ONBOARDING_ENCRYPTED_V1\nsalt={salt:016x}\nchecksum={checksum:016x}\ndata={}\n",
        encode_hex(&ciphertext)
    );

    if let Some(parent) = Path::new(&path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to prepare encrypted backup directory: {err}"))?;
        }
    }
    fs::write(&path, payload)
        .map_err(|err| format!("failed to export encrypted onboarding backup: {err}"))?;
    println!("[VANTIS] ONBOARDING EXPORTED ENCRYPTED: {path}");
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

fn onboard_import_encrypted(args: &[&str]) -> Result<(), String> {
    let (path_arg, pass) = parse_encrypted_backup_args(args)?;
    let path = resolve_encrypted_backup_path(path_arg.as_deref())?;
    let content =
        fs::read_to_string(&path).map_err(|err| format!("failed to read encrypted onboarding backup: {err}"))?;

    let mut lines = content.lines();
    let header = lines.next().unwrap_or_default();
    if header.trim() != "VANTIS_ONBOARDING_ENCRYPTED_V1" {
        return Err("failed to decrypt onboarding backup: unsupported format".to_string());
    }

    let mut salt_hex = String::new();
    let mut checksum_hex = String::new();
    let mut data_hex = String::new();
    for line in lines {
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "salt" => salt_hex = value.trim().to_string(),
                "checksum" => checksum_hex = value.trim().to_string(),
                "data" => data_hex = value.trim().to_string(),
                _ => {}
            }
        }
    }
    if salt_hex.is_empty() || checksum_hex.is_empty() || data_hex.is_empty() {
        return Err("failed to decrypt onboarding backup: incomplete payload".to_string());
    }

    let salt = u64::from_str_radix(&salt_hex, 16)
        .map_err(|_| "failed to decrypt onboarding backup: invalid salt".to_string())?;
    let expected_checksum = u64::from_str_radix(&checksum_hex, 16)
        .map_err(|_| "failed to decrypt onboarding backup: invalid checksum".to_string())?;
    let ciphertext = decode_hex(&data_hex)
        .map_err(|_| "failed to decrypt onboarding backup: invalid ciphertext".to_string())?;
    let plaintext_bytes = xor_crypt_bytes(&ciphertext, &pass, salt);
    let actual_checksum = fnv1a64(&plaintext_bytes);
    if actual_checksum != expected_checksum {
        return Err("failed to decrypt onboarding backup: integrity check failed".to_string());
    }

    let plaintext = String::from_utf8(plaintext_bytes)
        .map_err(|_| "failed to decrypt onboarding backup: invalid utf-8 payload".to_string())?;
    let mut map = parse_profile_config_text(&plaintext);
    if map.is_empty() {
        return Err("failed to decrypt onboarding backup: empty profile payload".to_string());
    }
    ensure_profile_defaults(&mut map);
    map.insert("mode".to_string(), "installed".to_string());

    let profile = map
        .get("profile")
        .cloned()
        .ok_or_else(|| "missing profile in encrypted onboarding backup".to_string())?;
    let user = map
        .get("user")
        .cloned()
        .ok_or_else(|| "missing user in encrypted onboarding backup".to_string())?;
    let hostname = map
        .get("hostname")
        .cloned()
        .ok_or_else(|| "missing hostname in encrypted onboarding backup".to_string())?;
    apply_profile_update(&mut map, "profile", &profile)?;
    apply_profile_update(&mut map, "user", &user)?;
    apply_profile_update(&mut map, "hostname", &hostname)?;

    write_profile_config(map.clone())?;
    write_welcome_message(&map);
    mark_onboarding_done("import_encrypted");
    println!("[VANTIS] ONBOARDING IMPORTED ENCRYPTED: {path}");
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

    if args.first() == Some(&"export-encrypted") {
        return onboard_export_encrypted(&args);
    }

    if args.first() == Some(&"import-encrypted") {
        return onboard_import_encrypted(&args);
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
                println!("              onboard export-encrypted [path] --pass <password>");
                println!("              onboard import-encrypted [path] --pass <password>");
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
