//! PROCFS - Process Virtual File System
//! Provides process and system information through /proc

use super::*;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

/// PROCFS entry type
pub enum ProcEntry {
    /// Static text content
    Static(&'static str),
    /// Dynamic content generator
    Dynamic(fn() -> String),
    /// Directory
    Directory,
}

/// PROCFS inode
pub struct ProcfsInode {
    /// Inode number
    pub ino: u64,
    /// Entry type
    pub entry: ProcEntry,
    /// Child entries (for directories)
    pub children: BTreeMap<String, u64>,
}

/// PROCFS superblock
pub struct ProcfsSuperblock {
    /// Root inode (always 1)
    pub root_ino: u64,
    /// Next inode number
    pub next_ino: u64,
    /// All inodes
    pub inodes: BTreeMap<u64, ProcfsInode>,
}

impl ProcfsSuperblock {
    /// Create a new PROCFS instance
    pub fn new() -> Self {
        let mut sb = Self {
            root_ino: 1,
            next_ino: 2,
            inodes: BTreeMap::new(),
        };
        
        // Create root directory
        let root = ProcfsInode {
            ino: 1,
            entry: ProcEntry::Directory,
            children: BTreeMap::new(),
        };
        sb.inodes.insert(1, root);
        
        // Add standard entries
        sb.add_static_file("version", "VantisOS v1.5.0 'Quantum Ready'\n");
        sb.add_static_file("osrelease", "1.5.0\n");
        sb.add_static_file("ostype", "VantisOS\n");
        sb.add_static_file("hostname", "vantis\n");
        sb.add_dynamic_file("cpuinfo", Self::generate_cpuinfo);
        sb.add_dynamic_file("meminfo", Self::generate_meminfo);
        sb.add_dynamic_file("uptime", Self::generate_uptime);
        sb.add_dynamic_file("loadavg", Self::generate_loadavg);
        
        // Create /proc/self symlink info
        sb.add_dynamic_file("self/status", Self::generate_self_status);
        
        sb
    }
    
    /// Add a static file
    fn add_static_file(&mut self, name: &str, content: &'static str) {
        let ino = self.next_ino;
        self.next_ino += 1;
        
        // Handle paths with directories
        if name.contains('/') {
            let parts: Vec<&str> = name.split('/').collect();
            if parts.len() == 2 {
                // Create directory first
                let dir_ino = self.next_ino;
                self.next_ino += 1;
                
                let dir = ProcfsInode {
                    ino: dir_ino,
                    entry: ProcEntry::Directory,
                    children: BTreeMap::new(),
                };
                self.inodes.insert(dir_ino, dir);
                
                // Add directory to root
                if let Some(root) = self.inodes.get_mut(&1) {
                    root.children.insert(String::from(parts[0]), dir_ino);
                }
                
                // Create file
                let file_ino = self.next_ino;
                self.next_ino += 1;
                
                let file = ProcfsInode {
                    ino: file_ino,
                    entry: ProcEntry::Static(content),
                    children: BTreeMap::new(),
                };
                self.inodes.insert(file_ino, file);
                
                // Add file to directory
                if let Some(dir) = self.inodes.get_mut(&dir_ino) {
                    dir.children.insert(String::from(parts[1]), file_ino);
                }
            }
        } else {
            let file = ProcfsInode {
                ino,
                entry: ProcEntry::Static(content),
                children: BTreeMap::new(),
            };
            self.inodes.insert(ino, file);
            
            if let Some(root) = self.inodes.get_mut(&1) {
                root.children.insert(String::from(name), ino);
            }
        }
    }
    
    /// Add a dynamic file
    fn add_dynamic_file(&mut self, name: &str, generator: fn() -> String) {
        let ino = self.next_ino;
        self.next_ino += 1;
        
        let file = ProcfsInode {
            ino,
            entry: ProcEntry::Dynamic(generator),
            children: BTreeMap::new(),
        };
        self.inodes.insert(ino, file);
        
        if let Some(root) = self.inodes.get_mut(&1) {
            root.children.insert(String::from(name), ino);
        }
    }
    
    /// Generate CPU info
    fn generate_cpuinfo() -> String {
        String::from(
            "processor\t: 0\n\
             vendor_id\t: VantisOS\n\
             cpu family\t: 6\n\
             model\t\t: 42\n\
             model name\t: Virtual CPU\n\
             stepping\t: 1\n\
             cpu MHz\t\t: 3000.000\n\
             cache size\t: 8192 KB\n\
             core id\t\t: 0\n\
             cpu cores\t: 1\n\
             fpu\t\t: yes\n\
             flags\t\t: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36\n\
             bogomips\t: 6000.00\n\n"
        )
    }
    
    /// Generate memory info
    fn generate_meminfo() -> String {
        // TODO: Read actual memory stats
        String::from(
            "MemTotal:       16384 kB\n\
             MemFree:        12288 kB\n\
             MemAvailable:   12288 kB\n\
             Buffers:          512 kB\n\
             Cached:          1024 kB\n\
             SwapCached:          0 kB\n\
             Active:          2048 kB\n\
             Inactive:        1024 kB\n\
             Shmem:            512 kB\n\
             KernelStack:      256 kB\n\
             CommitLimit:    16384 kB\n\
             Committed_AS:   2048 kB\n"
        )
    }
    
    /// Generate uptime
    fn generate_uptime() -> String {
        // TODO: Read actual uptime from timer
        String::from("0.00 0.00\n")
    }
    
    /// Generate load average
    fn generate_loadavg() -> String {
        String::from("0.00 0.00 0.00 1/64 0\n")
    }
    
    /// Generate self status
    fn generate_self_status() -> String {
        String::from(
            "Name:\tinit\n\
             State:\tS (sleeping)\n\
             Tgid:\t1\n\
             Pid:\t1\n\
             PPid:\t0\n\
             Uid:\t0\t0\t0\t0\n\
             Gid:\t0\t0\t0\t0\n\
             FDSize:\t64\n\
             Groups:\t0 \n\
             VmPeak:\t1024 kB\n\
             VmSize:\t1024 kB\n\
             VmRSS:\t512 kB\n\
             Threads:\t1\n\
             SigQ:\t0/256\n"
        )
    }
    
    /// Read from an inode
    pub fn read(&self, ino: u64, offset: usize, buf: &mut [u8]) -> Result<usize, FsError> {
        let inode = self.inodes.get(&ino)
            .ok_or(FsError::NotFound)?;
        
        let content = match &inode.entry {
            ProcEntry::Static(s) => s.as_bytes().to_vec(),
            ProcEntry::Dynamic(gen) => gen().into_bytes(),
            ProcEntry::Directory => {
                // List directory entries
                let mut entries = String::new();
                for (name, _) in &inode.children {
                    entries.push_str(name);
                    entries.push('\n');
                }
                entries.into_bytes()
            }
        };
        
        if offset >= content.len() {
            return Ok(0);
        }
        
        let end = core::cmp::min(offset + buf.len(), content.len());
        let count = end - offset;
        buf[..count].copy_from_slice(&content[offset..end]);
        
        Ok(count)
    }
    
    /// Look up a path
    pub fn lookup(&self, path: &str) -> Result<u64, FsError> {
        let mut current_ino = 1u64;
        
        for component in path.split('/').filter(|s| !s.is_empty()) {
            let inode = self.inodes.get(&current_ino)
                .ok_or(FsError::NotFound)?;
            
            current_ino = *inode.children.get(component)
                .ok_or(FsError::NotFound)?;
        }
        
        Ok(current_ino)
    }
}

/// Global PROCFS instance
pub static PROCFS: Mutex<Option<ProcfsSuperblock>> = Mutex::new(None);

/// Initialize PROCFS
pub fn init() {
    *PROCFS.lock() = Some(ProcfsSuperblock::new());
}