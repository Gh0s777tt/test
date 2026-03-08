//! Terminal Application
//! Command-line terminal emulator for VantisOS

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use crate::gui::*;

/// Terminal application
pub struct Terminal {
    /// Command history
    pub history: Vec<String>,
    /// History position
    pub history_pos: usize,
    /// Current input line
    pub input: String,
    /// Output lines
    pub output: Vec<String>,
    /// Current directory
    pub cwd: String,
    /// Cursor position
    pub cursor: usize,
    /// Username
    pub username: String,
    /// Hostname
    pub hostname: String,
}

impl Terminal {
    pub fn new() -> Self {
        let mut terminal = Self {
            history: Vec::new(),
            history_pos: 0,
            input: String::new(),
            output: Vec::new(),
            cwd: String::from("/"),
            cursor: 0,
            username: String::from("user"),
            hostname: String::from("vantis"),
        };
        
        terminal.output.push(String::from("VantisOS Terminal v1.5.0"));
        terminal.output.push(String::from("Type 'help' for available commands."));
        terminal.output.push(String::new());
        
        terminal
    }
    
    pub fn execute(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        
        if parts.is_empty() {
            return;
        }
        
        let command = parts[0];
        let args = &parts[1..];
        
        let result = match command {
            "help" => self.cmd_help(),
            "ls" => self.cmd_ls(args),
            "cd" => self.cmd_cd(args),
            "pwd" => self.cmd_pwd(),
            "cat" => self.cmd_cat(args),
            "mkdir" => self.cmd_mkdir(args),
            "rm" => self.cmd_rm(args),
            "clear" => { self.output.clear(); String::new() },
            "uname" => self.cmd_uname(args),
            "whoami" => self.cmd_whoami(),
            "date" => self.cmd_date(),
            "echo" => args.join(" "),
            "neofetch" => self.cmd_neofetch(),
            "exit" => { self.output.push(String::from("Goodbye!")); String::new() },
            _ => format!("Command not found: {}", command),
        };
        
        if !result.is_empty() {
            for line in result.lines() {
                self.output.push(String::from(line));
            }
        }
        
        self.output.push(String::new());
    }
    
    fn cmd_help(&self) -> String {
        String::from(
            "Available commands:\n\
             help     - Show this help message\n\
             ls       - List directory contents\n\
             cd       - Change directory\n\
             pwd      - Print working directory\n\
             cat      - Display file contents\n\
             mkdir    - Create directory\n\
             rm       - Remove file or directory\n\
             clear    - Clear terminal\n\
             uname    - System information\n\
             whoami   - Current user\n\
             date     - Current date/time\n\
             echo     - Print text\n\
             neofetch - System info display\n\
             exit     - Close terminal"
        )
    }
    
    fn cmd_ls(&self, args: &[&str]) -> String {
        let path = if args.is_empty() { &self.cwd } else { args[0] };
        
        // Simulated directory listing
        let entries: &[&str] = match path.as_str() {
            "/" => &["bin/", "boot/", "dev/", "etc/", "home/", "lib/", "opt/", "proc/", "root/", "sys/", "tmp/", "usr/", "var/"],
            "/home" => &["user/"],
            "/home/user" => &["Desktop/", "Documents/", "Downloads/", "Pictures/", "Music/", "Videos/"],
            _ => &["(empty)"],
        };
        
        entries.join("  ")
    }
    
    fn cmd_cd(&mut self, args: &[&str]) -> String {
        if args.is_empty() {
            self.cwd = String::from("/home/user");
            String::new()
        } else {
            let target = args[0];
            let new_path = if target.starts_with('/') {
                String::from(target)
            } else {
                format!("{}/{}", self.cwd.trim_end_matches('/'), target)
            };
            self.cwd = new_path;
            String::new()
        }
    }
    
    fn cmd_pwd(&self) -> String {
        self.cwd.clone()
    }
    
    fn cmd_cat(&self, _args: &[&str]) -> String {
        String::from("(file content would be displayed here)")
    }
    
    fn cmd_mkdir(&self, args: &[&str]) -> String {
        if args.is_empty() {
            String::from("Usage: mkdir <directory>")
        } else {
            format!("Created directory: {}", args[0])
        }
    }
    
    fn cmd_rm(&self, args: &[&str]) -> String {
        if args.is_empty() {
            String::from("Usage: rm <file>")
        } else {
            format!("Removed: {}", args[0])
        }
    }
    
    fn cmd_uname(&self, args: &[&str]) -> String {
        if args.contains(&"-a") {
            String::from("VantisOS 1.5.0 Quantum Ready x86_64 Vantis Kernel")
        } else {
            String::from("VantisOS")
        }
    }
    
    fn cmd_whoami(&self) -> String {
        self.username.clone()
    }
    
    fn cmd_date(&self) -> String {
        // Simulated date
        String::from("Sat Mar  8 03:00:00 UTC 2025")
    }
    
    fn cmd_neofetch(&self) -> String {
        String::from(
            r#"        ▄▄▄▄▄▄        user@vantis
       ▄▀░░░░░░▀▄      -----------
      █░░░░░░░░░░█     OS: VantisOS 1.5.0
      █░░░░░░░░░░█     Kernel: Vantis 1.5.0
      █░░░░░░░░░░█     Uptime: 0 hours
      █░░░░░░░░░░█     Shell: vsh
      █░░░░░░░░░░█     DE: VantisDE
      █░░░░░░░░░░█     WM: VantisWM
       ▀▄░░░░░░▄▀      Theme: Dark
         ▀▀▀▀▀         CPU: x86_64
                        Memory: 16GB"#
        )
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        // Background
        surface.fill_rect(Rect::new(0, 0, width, height), Color::new(20, 20, 20));
        
        // Output
        let mut y = 12;
        for line in &self.output {
            surface.draw_text_sized(12, y, line, 12, Color::new(200, 200, 200));
            y += 16;
        }
        
        // Prompt
        let prompt = format!("{}@{}:{}$ ", self.username, self.hostname, self.cwd);
        surface.draw_text_sized(12, y, &prompt, 12, Color::new(100, 180, 255));
        
        let prompt_len = prompt.len() as u32 * 7;
        surface.draw_text_sized(12 + prompt_len, y, &self.input, 12, Color::WHITE);
        
        // Cursor
        let cursor_x = 12 + prompt_len + (self.cursor as u32 * 7);
        surface.fill_rect(Rect::new(cursor_x as i32, y as i32 + 2, 8, 14), Color::WHITE);
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}