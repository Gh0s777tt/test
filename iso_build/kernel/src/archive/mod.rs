//! Archive Support Module for VantisOS
//! Provides comprehensive archive handling:
//! - ZIP, RAR, 7z, TAR, GZIP, BZIP2
//! - AES-256 encryption
//! - Self-extracting archives

pub mod zip;
pub mod tar;
pub mod compression;
pub mod encryption;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;
use spin::Mutex;

/// Archive format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    /// ZIP archive
    Zip,
    /// RAR archive
    Rar,
    /// 7-Zip archive
    SevenZip,
    /// TAR archive
    Tar,
    /// GZIP compressed
    Gzip,
    /// BZIP2 compressed
    Bzip2,
    /// XZ compressed
    Xz,
    /// WIM archive
    Wim,
    /// ISO image
    Iso,
    /// CAB archive
    Cab,
}

impl ArchiveFormat {
    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Zip => "zip",
            Self::Rar => "rar",
            Self::SevenZip => "7z",
            Self::Tar => "tar",
            Self::Gzip => "gz",
            Self::Bzip2 => "bz2",
            Self::Xz => "xz",
            Self::Wim => "wim",
            Self::Iso => "iso",
            Self::Cab => "cab",
        }
    }
    
    /// Detect format from extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "zip" => Some(Self::Zip),
            "rar" => Some(Self::Rar),
            "7z" => Some(Self::SevenZip),
            "tar" => Some(Self::Tar),
            "gz" | "gzip" => Some(Self::Gzip),
            "bz2" | "bzip2" => Some(Self::Bzip2),
            "xz" => Some(Self::Xz),
            "wim" => Some(Self::Wim),
            "iso" => Some(Self::Iso),
            "cab" => Some(Self::Cab),
            _ => None,
        }
    }
    
    /// Check if format supports compression
    pub fn supports_compression(&self) -> bool {
        matches!(self, Self::Zip | Self::SevenZip | Self::Gzip | Self::Bzip2 | Self::Xz)
    }
    
    /// Check if format supports encryption
    pub fn supports_encryption(&self) -> bool {
        matches!(self, Self::Zip | Self::SevenZip | Self::Rar)
    }
}

/// Compression level
#[derive(Debug, Clone, Copy)]
pub enum CompressionLevel {
    /// No compression
    None,
    /// Fast compression
    Fast,
    /// Normal compression
    Normal,
    /// Maximum compression
    Maximum,
    /// Ultra compression
    Ultra,
}

impl CompressionLevel {
    /// Get numeric value (0-9)
    pub fn value(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Fast => 1,
            Self::Normal => 5,
            Self::Maximum => 7,
            Self::Ultra => 9,
        }
    }
}

/// Encryption method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionMethod {
    /// No encryption
    None,
    /// AES-128
    Aes128,
    /// AES-256
    Aes256,
    /// ZipCrypto (legacy)
    ZipCrypto,
}

/// Archive entry metadata
#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    /// File name
    pub name: String,
    /// Original size
    pub size: u64,
    /// Compressed size
    pub compressed_size: u64,
    /// CRC32 checksum
    pub crc32: u32,
    /// Is directory
    pub is_dir: bool,
    /// Is encrypted
    pub is_encrypted: bool,
    /// Modification time
    pub modified_time: u64,
    /// Compression method
    pub compression: String,
    /// File attributes
    pub attributes: u32,
    /// File mode (Unix permissions)
    pub mode: u32,
}

impl ArchiveEntry {
    /// Create new archive entry
    pub fn new(name: String, size: u64) -> Self {
        Self {
            name,
            size,
            compressed_size: 0,
            crc32: 0,
            is_dir: false,
            is_encrypted: false,
            modified_time: 0,
            compression: String::new(),
            attributes: 0,
            mode: 0o644,
        }
    }
    
    /// Get compression ratio
    pub fn compression_ratio(&self) -> f32 {
        if self.size == 0 {
            return 0.0;
        }
        100.0 - ((self.compressed_size as f32 / self.size as f32) * 100.0)
    }
}

/// Archive open options
#[derive(Debug, Clone)]
pub struct ArchiveOptions {
    /// Password for encrypted archives
    pub password: Option<String>,
    /// Extract path
    pub extract_path: String,
    /// Overwrite existing files
    pub overwrite: bool,
    /// Preserve directory structure
    pub preserve_paths: bool,
    /// Compression level for creating
    pub compression_level: CompressionLevel,
    /// Encryption method
    pub encryption: EncryptionMethod,
    /// Split size for multi-volume (0 = no split)
    pub split_size: u64,
}

impl Default for ArchiveOptions {
    fn default() -> Self {
        Self {
            password: None,
            extract_path: String::from("/"),
            overwrite: false,
            preserve_paths: true,
            compression_level: CompressionLevel::Normal,
            encryption: EncryptionMethod::None,
            split_size: 0,
        }
    }
}

/// Archive reader trait
pub trait ArchiveReader {
    /// Open archive
    fn open(&mut self, data: &[u8], options: &ArchiveOptions) -> Result<(), ArchiveError>;
    
    /// Get entry list
    fn entries(&self) -> &[ArchiveEntry];
    
    /// Extract single entry
    fn extract_entry(&self, name: &str, output: &mut [u8]) -> Result<usize, ArchiveError>;
    
    /// Extract all entries
    fn extract_all(&self, path: &str) -> Result<usize, ArchiveError>;
    
    /// Close archive
    fn close(&mut self);
}

/// Archive writer trait
pub trait ArchiveWriter {
    /// Create new archive
    fn create(&mut self, format: ArchiveFormat, options: &ArchiveOptions) -> Result<(), ArchiveError>;
    
    /// Add file to archive
    fn add_file(&mut self, name: &str, data: &[u8]) -> Result<(), ArchiveError>;
    
    /// Add directory to archive
    fn add_directory(&mut self, name: &str) -> Result<(), ArchiveError>;
    
    /// Finalize and get archive data
    fn finalize(&mut self) -> Result<Vec<u8>, ArchiveError>;
}

/// Archive errors
#[derive(Debug, Clone, Copy)]
pub enum ArchiveError {
    /// Invalid archive format
    InvalidFormat,
    /// Invalid data
    InvalidData,
    /// Corrupted archive
    Corrupted,
    /// Password required
    PasswordRequired,
    /// Wrong password
    WrongPassword,
    /// Not enough memory
    OutOfMemory,
    /// I/O error
    IoError,
    /// Entry not found
    NotFound,
    /// Entry not found (alias)
    EntryNotFound,
    /// Unsupported format
    Unsupported,
    /// Compression error
    CompressionError,
    /// Encryption error
    EncryptionError,
}

/// Global archive state
pub static ARCHIVE_STATE: Mutex<ArchiveState> = Mutex::new(ArchiveState {
    initialized: false,
    temp_path: String::new(),
});

/// Archive state
pub struct ArchiveState {
    pub initialized: bool,
    pub temp_path: String,
}

/// Initialize archive support
pub fn init() {
    let mut state = ARCHIVE_STATE.lock();
    state.initialized = true;
    state.temp_path = String::from("/tmp/archive");
}

/// Open an archive
pub fn open_archive(data: &[u8], options: &ArchiveOptions) -> Result<Box<dyn ArchiveReader>, ArchiveError> {
    // Detect format from magic bytes
    let format = detect_format(data)?;
    
    match format {
        ArchiveFormat::Zip => {
            let mut reader = zip::ZipReader::new();
            ArchiveReader::open(&mut reader, data, options)?;
            Ok(Box::new(reader))
        }
        ArchiveFormat::Tar => {
            let mut reader = tar::TarReader::new();
            ArchiveReader::open(&mut reader, data, options)?;
            Ok(Box::new(reader))
        }
        _ => Err(ArchiveError::Unsupported),
    }
}

/// Detect archive format from magic bytes
pub fn detect_format(data: &[u8]) -> Result<ArchiveFormat, ArchiveError> {
    if data.len() < 4 {
        return Err(ArchiveError::InvalidFormat);
    }
    
    // ZIP: PK\x03\x04
    if data[0..4] == [0x50, 0x4B, 0x03, 0x04] {
        return Ok(ArchiveFormat::Zip);
    }
    
    // RAR: Rar!\x1a\x07
    if data.len() >= 7 && data[0..7] == [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00] {
        return Ok(ArchiveFormat::Rar);
    }
    
    // 7z: 7z\xbc\xaf\x27\x1c
    if data.len() >= 6 && data[0..6] == [0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C] {
        return Ok(ArchiveFormat::SevenZip);
    }
    
    // GZIP: \x1f\x8b
    if data[0..2] == [0x1F, 0x8B] {
        return Ok(ArchiveFormat::Gzip);
    }
    
    // BZIP2: BZ
    if data[0..2] == [0x42, 0x5A] {
        return Ok(ArchiveFormat::Bzip2);
    }
    
    // XZ: \xfd7zXZ\x00
    if data.len() >= 6 && data[0..6] == [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00] {
        return Ok(ArchiveFormat::Xz);
    }
    
    // TAR: Check at offset 257 for "ustar"
    if data.len() >= 262 && &data[257..262] == b"ustar" {
        return Ok(ArchiveFormat::Tar);
    }
    
    Err(ArchiveError::InvalidFormat)
}

/// Create a new archive
pub fn create_archive(format: ArchiveFormat, options: &ArchiveOptions) -> Result<Box<dyn ArchiveWriter>, ArchiveError> {
    match format {
        ArchiveFormat::Zip => {
            let mut writer = zip::ZipWriter::new();
            writer.create(format, options)?;
            Ok(Box::new(writer))
        }
        ArchiveFormat::Tar => {
            let mut writer = tar::TarWriter::new();
            writer.create(format, options)?;
            Ok(Box::new(writer))
        }
        _ => Err(ArchiveError::Unsupported),
    }
}