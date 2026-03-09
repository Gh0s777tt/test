// Dynamic Linker (ld.so)
// Dynamic library loading, symbol resolution, relocation

use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// ELF Types
// ============================================================================

/// ELF header magic
pub const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];

/// ELF class (32-bit or 64-bit)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElfClass {
    Elf32 = 1,
    Elf64 = 2,
}

/// ELF data encoding (little-endian or big-endian)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElfData {
    LittleEndian = 1,
    BigEndian = 2,
}

/// ELF file type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElfType {
    Relocatable = 1,
    Executable = 2,
    SharedObject = 3,
    Core = 4,
}

/// ELF machine type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElfMachine {
    X86 = 3,
    X86_64 = 62,
    ARM = 40,
    AArch64 = 183,
}

/// ELF header
#[derive(Debug, Clone)]
pub struct ElfHeader {
    pub magic: [u8; 4],
    pub class: ElfClass,
    pub data: ElfData,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub pad: [u8; 7],
    pub elf_type: ElfType,
    pub machine: ElfMachine,
    pub version2: u32,
    pub entry: u64,
    pub phoff: u64,
    pub shoff: u64,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

/// Program header type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    ShLib = 5,
    Phdr = 6,
    Tls = 7,
}

/// Program header
#[derive(Debug, Clone)]
pub struct ProgramHeader {
    pub p_type: PhType,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_flags: u32,
    pub p_align: u64,
}

/// Section header type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShType {
    Null = 0,
    Progbits = 1,
    Symtab = 2,
    Strtab = 3,
    Rela = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    NoBits = 8,
    Rel = 9,
    Shlib = 10,
    Dynsym = 11,
}

/// Section header
#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: ShType,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

/// Symbol binding
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymBind {
    Local = 0,
    Global = 1,
    Weak = 2,
}

/// Symbol type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymType {
    Notype = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    Common = 5,
    Tls = 6,
}

/// Symbol table entry
#[derive(Debug, Clone)]
pub struct Symbol {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

impl Symbol {
    pub fn get_bind(&self) -> SymBind {
        SymBind::from((self.st_info >> 4) as u8)
    }

    pub fn get_type(&self) -> SymType {
        SymType::from(self.st_info & 0xF)
    }
}

impl From<u8> for SymBind {
    fn from(value: u8) -> Self {
        match value {
            0 => SymBind::Local,
            1 => SymBind::Global,
            2 => SymBind::Weak,
            _ => SymBind::Local,
        }
    }
}

impl From<u8> for SymType {
    fn from(value: u8) -> Self {
        match value {
            0 => SymType::Notype,
            1 => SymType::Object,
            2 => SymType::Func,
            3 => SymType::Section,
            4 => SymType::File,
            5 => SymType::Common,
            6 => SymType::Tls,
            _ => SymType::Notype,
        }
    }
}

/// Relocation type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelType {
    None = 0,
    Direct64 = 1,
    PcRelative32 = 2,
    Got32 = 3,
    Plt32 = 4,
    Copy = 5,
    GlobDat = 6,
    JumpSlot = 7,
    Relative = 8,
    GotRelative = 9,
}

/// Relocation entry
#[derive(Debug, Clone)]
pub struct Relocation {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

impl Relocation {
    pub fn get_type(&self) -> RelType {
        RelType::from((self.r_info & 0xFFFFFFFF) as u32)
    }

    pub fn get_symbol(&self) -> u32 {
        (self.r_info >> 32) as u32
    }
}

impl From<u32> for RelType {
    fn from(value: u32) -> Self {
        match value {
            0 => RelType::None,
            1 => RelType::Direct64,
            2 => RelType::PcRelative32,
            3 => RelType::Got32,
            4 => RelType::Plt32,
            5 => RelType::Copy,
            6 => RelType::GlobDat,
            7 => RelType::JumpSlot,
            8 => RelType::Relative,
            9 => RelType::GotRelative,
            _ => RelType::None,
        }
    }
}

// ============================================================================
// Dynamic Linker
// ============================================================================

/// Dynamic linker
pub struct DynamicLinker {
    loaded_libraries: BTreeMap<String, LoadedLibrary>,
    symbol_table: BTreeMap<String, u64>,
    next_handle: AtomicU64,
}

/// Loaded library
#[derive(Debug, Clone)]
pub struct LoadedLibrary {
    pub handle: u64,
    pub name: String,
    pub base_address: u64,
    pub symbols: BTreeMap<String, u64>,
    pub relocations: Vec<Relocation>,
}

impl DynamicLinker {
    pub fn new() -> Self {
        Self {
            loaded_libraries: BTreeMap::new(),
            symbol_table: BTreeMap::new(),
            next_handle: AtomicU64::new(1),
        }
    }

    /// Load ELF file
    pub fn load_elf(&mut self, data: &[u8]) -> Result<LoadedLibrary, &'static str> {
        // Parse ELF header
        let header = self.parse_elf_header(data)?;
        
        // Validate ELF header
        if header.magic != ELF_MAGIC {
            return Err("Invalid ELF magic");
        }
        
        if header.class != ElfClass::Elf64 {
            return Err("Only 64-bit ELF supported");
        }
        
        if header.machine != ElfMachine::X86_64 {
            return Err("Only x86_64 ELF supported");
        }
        
        // Parse program headers
        let program_headers = self.parse_program_headers(data, &header)?;
        
        // Parse section headers
        let section_headers = self.parse_section_headers(data, &header)?;
        
        // Find symbol table and string table
        let mut symbols = BTreeMap::new();
        let mut relocations = Vec::new();
        
        for sh in &section_headers {
            if sh.sh_type == ShType::Dynsym {
                symbols = self.parse_symbol_table(data, sh)?;
            } else if sh.sh_type == ShType::Rela {
                relocations = self.parse_relocations(data, sh)?;
            }
        }
        
        // Create loaded library
        let handle = self.next_handle.fetch_add(1, Ordering::SeqCst);
        let library = LoadedLibrary {
            handle,
            name: String::from("unknown"),
            base_address: 0,
            symbols,
            relocations,
        };
        
        // Add to symbol table
        for (name, addr) in &library.symbols {
            self.symbol_table.insert(name.clone(), *addr);
        }
        
        Ok(library)
    }

    /// Parse ELF header
    fn parse_elf_header(&self, data: &[u8]) -> Result<ElfHeader, &'static str> {
        if data.len() < 64 {
            return Err("ELF file too small");
        }

        let header = ElfHeader {
            magic: [data[0], data[1], data[2], data[3]],
            class: if data[4] == 1 { ElfClass::Elf32 } else { ElfClass::Elf64 },
            data: if data[5] == 1 { ElfData::LittleEndian } else { ElfData::BigEndian },
            version: data[6],
            os_abi: data[7],
            abi_version: data[8],
            pad: [data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16]],
            elf_type: match data[16] {
                1 => ElfType::Relocatable,
                2 => ElfType::Executable,
                3 => ElfType::SharedObject,
                4 => ElfType::Core,
                _ => return Err("Invalid ELF type"),
            },
            machine: match u16::from_le_bytes([data[18], data[19]]) {
                3 => ElfMachine::X86,
                62 => ElfMachine::X86_64,
                40 => ElfMachine::ARM,
                183 => ElfMachine::AArch64,
                _ => return Err("Unsupported machine type"),
            },
            version2: u32::from_le_bytes([data[20], data[21], data[22], data[23]]),
            entry: u64::from_le_bytes([
                data[24], data[25], data[26], data[27],
                data[28], data[29], data[30], data[31],
            ]),
            phoff: u64::from_le_bytes([
                data[32], data[33], data[34], data[35],
                data[36], data[37], data[38], data[39],
            ]),
            shoff: u64::from_le_bytes([
                data[40], data[41], data[42], data[43],
                data[44], data[45], data[46], data[47],
            ]),
            flags: u32::from_le_bytes([data[48], data[49], data[50], data[51]]),
            ehsize: u16::from_le_bytes([data[52], data[53]]),
            phentsize: u16::from_le_bytes([data[54], data[55]]),
            phnum: u16::from_le_bytes([data[56], data[57]]),
            shentsize: u16::from_le_bytes([data[58], data[59]]),
            shnum: u16::from_le_bytes([data[60], data[61]]),
            shstrndx: u16::from_le_bytes([data[62], data[63]]),
        };

        Ok(header)
    }

    /// Parse program headers
    fn parse_program_headers(&self, data: &[u8], header: &ElfHeader) -> Result<Vec<ProgramHeader>, &'static str> {
        let mut program_headers = Vec::new();
        let offset = header.phoff as usize;
        let entry_size = header.phentsize as usize;
        let num_entries = header.phnum as usize;

        for i in 0..num_entries {
            let entry_offset = offset + i * entry_size;
            if entry_offset + entry_size > data.len() {
                return Err("Program header out of bounds");
            }

            let p = &data[entry_offset..entry_offset + entry_size];
            let ph = ProgramHeader {
                p_type: match u32::from_le_bytes([p[0], p[1], p[2], p[3]]) {
                    0 => PhType::Null,
                    1 => PhType::Load,
                    2 => PhType::Dynamic,
                    3 => PhType::Interp,
                    4 => PhType::Note,
                    5 => PhType::ShLib,
                    6 => PhType::Phdr,
                    7 => PhType::Tls,
                    _ => return Err("Invalid program header type"),
                },
                p_offset: u64::from_le_bytes([
                    p[8], p[9], p[10], p[11],
                    p[12], p[13], p[14], p[15],
                ]),
                p_vaddr: u64::from_le_bytes([
                    p[16], p[17], p[18], p[19],
                    p[20], p[21], p[22], p[23],
                ]),
                p_paddr: u64::from_le_bytes([
                    p[24], p[25], p[26], p[27],
                    p[28], p[29], p[30], p[31],
                ]),
                p_filesz: u64::from_le_bytes([
                    p[32], p[33], p[34], p[35],
                    p[36], p[37], p[38], p[39],
                ]),
                p_memsz: u64::from_le_bytes([
                    p[40], p[41], p[42], p[43],
                    p[44], p[45], p[46], p[47],
                ]),
                p_flags: u32::from_le_bytes([p[4], p[5], p[6], p[7]]),
                p_align: u64::from_le_bytes([
                    p[48], p[49], p[50], p[51],
                    p[52], p[53], p[54], p[55],
                ]),
            };

            program_headers.push(ph);
        }

        Ok(program_headers)
    }

    /// Parse section headers
    fn parse_section_headers(&self, data: &[u8], header: &ElfHeader) -> Result<Vec<SectionHeader>, &'static str> {
        let mut section_headers = Vec::new();
        let offset = header.shoff as usize;
        let entry_size = header.shentsize as usize;
        let num_entries = header.shnum as usize;

        for i in 0..num_entries {
            let entry_offset = offset + i * entry_size;
            if entry_offset + entry_size > data.len() {
                return Err("Section header out of bounds");
            }

            let s = &data[entry_offset..entry_offset + entry_size];
            let sh = SectionHeader {
                sh_name: u32::from_le_bytes([s[0], s[1], s[2], s[3]]),
                sh_type: match u32::from_le_bytes([s[4], s[5], s[6], s[7]]) {
                    0 => ShType::Null,
                    1 => ShType::Progbits,
                    2 => ShType::Symtab,
                    3 => ShType::Strtab,
                    4 => ShType::Rela,
                    5 => ShType::Hash,
                    6 => ShType::Dynamic,
                    7 => ShType::Note,
                    8 => ShType::NoBits,
                    9 => ShType::Rel,
                    10 => ShType::ShLib,
                    11 => ShType::Dynsym,
                    _ => return Err("Invalid section header type"),
                },
                sh_flags: u64::from_le_bytes([
                    s[8], s[9], s[10], s[11],
                    s[12], s[13], s[14], s[15],
                ]),
                sh_addr: u64::from_le_bytes([
                    s[16], s[17], s[18], s[19],
                    s[20], s[21], s[22], s[23],
                ]),
                sh_offset: u64::from_le_bytes([
                    s[24], s[25], s[26], s[27],
                    s[28], s[29], s[30], s[31],
                ]),
                sh_size: u64::from_le_bytes([
                    s[32], s[33], s[34], s[35],
                    s[36], s[37], s[38], s[39],
                ]),
                sh_link: u32::from_le_bytes([s[40], s[41], s[42], s[43]]),
                sh_info: u32::from_le_bytes([s[44], s[45], s[46], s[47]]),
                sh_addralign: u64::from_le_bytes([
                    s[48], s[49], s[50], s[51],
                    s[52], s[53], s[54], s[55],
                ]),
                sh_entsize: u64::from_le_bytes([
                    s[56], s[57], s[58], s[59],
                    s[60], s[61], s[62], s[63],
                ]),
            };

            section_headers.push(sh);
        }

        Ok(section_headers)
    }

    /// Parse symbol table
    fn parse_symbol_table(&self, data: &[u8], sh: &SectionHeader) -> Result<BTreeMap<String, u64>, &'static str> {
        let mut symbols = BTreeMap::new();
        let offset = sh.sh_offset as usize;
        let entry_size = sh.sh_entsize as usize;
        let num_entries = (sh.sh_size / entry_size) as usize;

        for i in 0..num_entries {
            let entry_offset = offset + i * entry_size;
            if entry_offset + entry_size > data.len() {
                return Err("Symbol entry out of bounds");
            }

            let s = &data[entry_offset..entry_offset + entry_size];
            let symbol = Symbol {
                st_name: u32::from_le_bytes([s[0], s[1], s[2], s[3]]),
                st_info: s[4],
                st_other: s[5],
                st_shndx: u16::from_le_bytes([s[6], s[7]]),
                st_value: u64::from_le_bytes([
                    s[8], s[9], s[10], s[11],
                    s[12], s[13], s[14], s[15],
                ]),
                st_size: u64::from_le_bytes([
                    s[16], s[17], s[18], s[19],
                    s[20], s[21], s[22], s[23],
                ]),
            };

            // Only add global symbols
            if symbol.get_bind() == SymBind::Global {
                // TODO: Get symbol name from string table
                let name = format!("symbol_{}", i);
                symbols.insert(name, symbol.st_value);
            }
        }

        Ok(symbols)
    }

    /// Parse relocations
    fn parse_relocations(&self, data: &[u8], sh: &SectionHeader) -> Result<Vec<Relocation>, &'static str> {
        let mut relocations = Vec::new();
        let offset = sh.sh_offset as usize;
        let entry_size = sh.sh_entsize as usize;
        let num_entries = (sh.sh_size / entry_size) as usize;

        for i in 0..num_entries {
            let entry_offset = offset + i * entry_size;
            if entry_offset + entry_size > data.len() {
                return Err("Relocation entry out of bounds");
            }

            let r = &data[entry_offset..entry_offset + entry_size];
            let relocation = Relocation {
                r_offset: u64::from_le_bytes([
                    r[0], r[1], r[2], r[3],
                    r[4], r[5], r[6], r[7],
                ]),
                r_info: u64::from_le_bytes([
                    r[8], r[9], r[10], r[11],
                    r[12], r[13], r[14], r[15],
                ]),
                r_addend: i64::from_le_bytes([
                    r[16], r[17], r[18], r[19],
                    r[20], r[21], r[22], r[23],
                ]),
            };

            relocations.push(relocation);
        }

        Ok(relocations)
    }

    /// Resolve symbol
    pub fn resolve_symbol(&self, name: &str) -> Option<u64> {
        self.symbol_table.get(name).copied()
    }

    /// Apply relocations
    pub fn apply_relocations(&mut self, library: &mut LoadedLibrary) -> Result<(), &'static str> {
        for rel in &library.relocations {
            match rel.get_type() {
                RelType::Direct64 => {
                    // Direct 64-bit relocation
                    // In real implementation, this would write the symbol address to r_offset
                }
                RelType::Relative => {
                    // Relative relocation
                    // In real implementation, this would write the relative address
                }
                RelType::JumpSlot => {
                    // PLT jump slot
                    // In real implementation, this would write the PLT entry
                }
                _ => {
                    // Other relocation types
                }
            }
        }
        Ok(())
    }

    /// Get linker statistics
    pub fn get_stats(&self) -> LinkerStats {
        LinkerStats {
            loaded_libraries: self.loaded_libraries.len(),
            total_symbols: self.symbol_table.len(),
        }
    }
}

/// Linker statistics
#[derive(Debug, Clone)]
pub struct LinkerStats {
    pub loaded_libraries: usize,
    pub total_symbols: usize,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_linker_create() {
        let linker = DynamicLinker::new();
        assert_eq!(linker.loaded_libraries.len(), 0);
    }

    #[test]
    fn test_parse_elf_header() {
        let linker = DynamicLinker::new();
        let mut data = vec![0u8; 64];
        data[0..4].copy_from_slice(&ELF_MAGIC);
        data[4] = 2; // ELF64
        data[5] = 1; // Little endian
        data[16] = 2; // Executable
        data[18] = 62; // x86_64
        
        let header = linker.parse_elf_header(&data);
        assert!(header.is_ok());
        let header = header.unwrap();
        assert_eq!(header.magic, ELF_MAGIC);
        assert_eq!(header.class, ElfClass::Elf64);
        assert_eq!(header.machine, ElfMachine::X86_64);
    }

    #[test]
    fn test_symbol_bind() {
        let symbol = Symbol {
            st_name: 0,
            st_info: 0x11, // Global, Func
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
        };
        assert_eq!(symbol.get_bind(), SymBind::Global);
        assert_eq!(symbol.get_type(), SymType::Func);
    }

    #[test]
    fn test_relocation_type() {
        let rel = Relocation {
            r_offset: 0,
            r_info: 0x0000000100000001, // Symbol 1, type 1 (Direct64)
            r_addend: 0,
        };
        assert_eq!(rel.get_type(), RelType::Direct64);
        assert_eq!(rel.get_symbol(), 1);
    }
}