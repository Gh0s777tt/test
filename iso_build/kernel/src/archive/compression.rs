//! Compression Algorithms
//! Implements various compression methods:
//! - DEFLATE (used in ZIP, GZIP)
//! - LZMA (used in 7z, XZ)
//! - BZIP2
//! - Simple RLE for testing

use alloc::vec::Vec;
use alloc::boxed::Box;

/// Compression error type
#[derive(Debug, Clone)]
pub enum CompressionError {
    /// Invalid data
    InvalidData,
    /// Decompression failed
    DecompressionFailed,
    /// Compression failed
    CompressionFailed,
    /// Unsupported method
    UnsupportedMethod,
    /// Out of memory
    OutOfMemory,
}

/// Compression result type
pub type CompressionResult<T> = core::result::Result<T, CompressionError>;

/// Compression method trait
pub trait Compressor {
    /// Compress data
    fn compress(&self, data: &[u8]) -> CompressionResult<Vec<u8>>;
    /// Decompress data
    fn decompress(&self, data: &[u8], expected_size: Option<usize>) -> CompressionResult<Vec<u8>>;
    /// Get compression name
    fn name(&self) -> &'static str;
}

/// No compression (store)
pub struct StoreCompressor;

impl Compressor for StoreCompressor {
    fn compress(&self, data: &[u8]) -> CompressionResult<Vec<u8>> {
        Ok(data.to_vec())
    }
    
    fn decompress(&self, data: &[u8], _expected_size: Option<usize>) -> CompressionResult<Vec<u8>> {
        Ok(data.to_vec())
    }
    
    fn name(&self) -> &'static str {
        "Store"
    }
}

/// Run-Length Encoding compressor (simple, for testing)
pub struct RleCompressor;

impl Compressor for RleCompressor {
    fn compress(&self, data: &[u8]) -> CompressionResult<Vec<u8>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < data.len() {
            let byte = data[i];
            let mut count = 1;
            
            while i + count < data.len() && data[i + count] == byte && count < 255 {
                count += 1;
            }
            
            result.push(count as u8);
            result.push(byte);
            i += count;
        }
        
        Ok(result)
    }
    
    fn decompress(&self, data: &[u8], _expected_size: Option<usize>) -> CompressionResult<Vec<u8>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut result = Vec::new();
        let mut i = 0;
        
        while i + 1 < data.len() {
            let count = data[i] as usize;
            let byte = data[i + 1];
            
            for _ in 0..count {
                result.push(byte);
            }
            
            i += 2;
        }
        
        Ok(result)
    }
    
    fn name(&self) -> &'static str {
        "RLE"
    }
}

/// DEFLATE compressor (simplified implementation)
/// Full implementation would need a proper LZ77 + Huffman coder
pub struct DeflateCompressor {
    /// Compression level (0-9)
    level: u8,
}

impl DeflateCompressor {
    pub fn new(level: u8) -> Self {
        Self { level: level.min(9) }
    }
}

impl Compressor for DeflateCompressor {
    fn compress(&self, data: &[u8]) -> CompressionResult<Vec<u8>> {
        // Simplified DEFLATE: store with minimal header
        // A full implementation would use LZ77 + Huffman coding
        
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        // Create a stored block (no compression)
        // This is valid DEFLATE but not efficient
        let mut result = Vec::new();
        
        // Block header: BFINAL=1, BTYPE=00 (stored)
        result.push(0x01);
        
        // LEN and NLEN
        let len = data.len() as u16;
        result.extend_from_slice(&len.to_le_bytes());
        result.extend_from_slice(&(!len).to_le_bytes());
        
        // Data
        result.extend_from_slice(data);
        
        Ok(result)
    }
    
    fn decompress(&self, data: &[u8], _expected_size: Option<usize>) -> CompressionResult<Vec<u8>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut result = Vec::new();
        let mut pos = 0;
        
        loop {
            if pos >= data.len() {
                break;
            }
            
            let header = data[pos];
            pos += 1;
            
            let bfinal = header & 1;
            let btype = (header >> 1) & 3;
            
            match btype {
                0 => {
                    // Stored block
                    if pos + 4 > data.len() {
                        return Err(CompressionError::InvalidData);
                    }
                    
                    let len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
                    pos += 4; // Skip LEN and NLEN
                    
                    if pos + len > data.len() {
                        return Err(CompressionError::InvalidData);
                    }
                    
                    result.extend_from_slice(&data[pos..pos + len]);
                    pos += len;
                }
                1 | 2 => {
                    // Compressed with Huffman codes
                    // Full implementation would decode dynamic/fixed Huffman
                    // For now, return error
                    return Err(CompressionError::UnsupportedMethod);
                }
                3 => {
                    return Err(CompressionError::InvalidData);
                }
                _ => unreachable!(),
            }
            
            if bfinal == 1 {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn name(&self) -> &'static str {
        "Deflate"
    }
}

/// LZMA compressor (placeholder - would need full implementation)
pub struct LzmaCompressor {
    /// Dictionary size
    dict_size: u32,
    /// Literal context bits
    lc: u8,
    /// Literal position bits
    lp: u8,
    /// Position bits
    pb: u8,
}

impl LzmaCompressor {
    pub fn new() -> Self {
        Self {
            dict_size: 8 * 1024 * 1024, // 8 MB
            lc: 3,
            lp: 0,
            pb: 2,
        }
    }
    
    /// Encode properties byte
    pub fn encode_properties(&self) -> u8 {
        ((self.pb * 5 + self.lp) * 9 + self.lc) as u8
    }
    
    /// Decode properties byte
    pub fn decode_properties(props: u8) -> Self {
        let props = props as usize;
        let lc = props % 9;
        let props = props / 9;
        let lp = props % 5;
        let pb = props / 5;
        
        Self {
            dict_size: 8 * 1024 * 1024,
            lc: lc as u8,
            lp: lp as u8,
            pb: pb as u8,
        }
    }
}

impl Compressor for LzmaCompressor {
    fn compress(&self, _data: &[u8]) -> CompressionResult<Vec<u8>> {
        // LZMA compression is complex - this is a placeholder
        // A full implementation would need range coding + LZ77
        Err(CompressionError::UnsupportedMethod)
    }
    
    fn decompress(&self, _data: &[u8], _expected_size: Option<usize>) -> CompressionResult<Vec<u8>> {
        // LZMA decompression is complex - this is a placeholder
        Err(CompressionError::UnsupportedMethod)
    }
    
    fn name(&self) -> &'static str {
        "LZMA"
    }
}

/// BZIP2 compressor (placeholder)
pub struct Bzip2Compressor {
    /// Block size (1-9)
    block_size: u8,
}

impl Bzip2Compressor {
    pub fn new(block_size: u8) -> Self {
        Self {
            block_size: block_size.min(9).max(1),
        }
    }
}

impl Compressor for Bzip2Compressor {
    fn compress(&self, _data: &[u8]) -> CompressionResult<Vec<u8>> {
        // BZIP2 uses Burrows-Wheeler Transform + Huffman
        // Placeholder implementation
        Err(CompressionError::UnsupportedMethod)
    }
    
    fn decompress(&self, _data: &[u8], _expected_size: Option<usize>) -> CompressionResult<Vec<u8>> {
        Err(CompressionError::UnsupportedMethod)
    }
    
    fn name(&self) -> &'static str {
        "BZip2"
    }
}

/// Zstandard compressor (placeholder)
pub struct ZstdCompressor {
    /// Compression level
    level: i32,
}

impl ZstdCompressor {
    pub fn new(level: i32) -> Self {
        Self {
            level: level.clamp(-5, 22),
        }
    }
}

impl Compressor for ZstdCompressor {
    fn compress(&self, _data: &[u8]) -> CompressionResult<Vec<u8>> {
        Err(CompressionError::UnsupportedMethod)
    }
    
    fn decompress(&self, _data: &[u8], _expected_size: Option<usize>) -> CompressionResult<Vec<u8>> {
        Err(CompressionError::UnsupportedMethod)
    }
    
    fn name(&self) -> &'static str {
        "Zstandard"
    }
}

/// Compression factory
pub struct CompressionFactory;

impl CompressionFactory {
    /// Create a compressor by method name
    pub fn create(method: &str) -> Box<dyn Compressor> {
        match method.to_lowercase().as_str() {
            "store" | "none" => Box::new(StoreCompressor),
            "rle" => Box::new(RleCompressor),
            "deflate" | "zip" | "gzip" => Box::new(DeflateCompressor::new(6)),
            "lzma" | "xz" | "7z" => Box::new(LzmaCompressor::new()),
            "bzip2" | "bz2" => Box::new(Bzip2Compressor::new(9)),
            "zstd" => Box::new(ZstdCompressor::new(3)),
            _ => Box::new(StoreCompressor),
        }
    }
    
    /// Get supported compression methods
    pub fn supported_methods() -> &'static [&'static str] {
        &["store", "deflate", "lzma", "bzip2", "zstd"]
    }
}

/// Utility functions

/// Calculate compression ratio
pub fn compression_ratio(original: usize, compressed: usize) -> f32 {
    if original == 0 {
        return 0.0;
    }
    (compressed as f32 / original as f32) * 100.0
}

/// Estimate compressed size
pub fn estimate_compressed_size(data: &[u8], method: &str) -> usize {
    // Rough estimates based on typical compression ratios
    let ratio = match method {
        "store" => 1.0,
        "deflate" => 0.4,
        "lzma" => 0.35,
        "bzip2" => 0.35,
        "zstd" => 0.4,
        _ => 1.0,
    };
    (data.len() as f32 * ratio) as usize
}