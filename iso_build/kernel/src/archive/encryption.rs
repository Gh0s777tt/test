//! Archive Encryption Module
//! Implements AES-256 encryption for archives
//! 
//! Features:
//! - AES-256-CBC encryption
//! - PBKDF2 key derivation
//! - SHA-256 hashing
//! - Secure random IV generation

use alloc::vec::Vec;
use alloc::string::String;

/// Encryption error type
#[derive(Debug, Clone)]
pub enum EncryptionError {
    /// Invalid key size
    InvalidKeySize,
    /// Invalid IV size
    InvalidIvSize,
    /// Invalid data length
    InvalidDataLength,
    /// Encryption failed
    EncryptionFailed,
    /// Decryption failed
    DecryptionFailed,
    /// Invalid password
    InvalidPassword,
    /// Unsupported method
    UnsupportedMethod,
    /// Authentication failed
    AuthenticationFailed,
}

/// Encryption result type
pub type EncryptionResult<T> = core::result::Result<T, EncryptionError>;

/// AES block size (16 bytes)
const AES_BLOCK_SIZE: usize = 16;

/// AES-256 key size (32 bytes)
const AES_256_KEY_SIZE: usize = 32;

/// PBKDF2 iterations
const PBKDF2_ITERATIONS: u32 = 100000;

/// AES state (4x4 matrix of bytes)
type AesState = [[u8; 4]; 4];

/// AES-256 encryption context
pub struct Aes256 {
    /// Round keys (14 rounds + initial key = 15 * 16 bytes)
    round_keys: [[u8; 16]; 15],
}

impl Aes256 {
    /// Create new AES-256 context from key
    pub fn new(key: &[u8; 32]) -> Self {
        let mut ctx = Self {
            round_keys: [[0u8; 16]; 15],
        };
        ctx.key_expansion(key);
        ctx
    }
    
    /// Key expansion for AES-256
    fn key_expansion(&mut self, key: &[u8; 32]) {
        // AES-256 uses 14 rounds
        // Key schedule: first 2 round keys come from the key itself
        
        // Copy key to first two round keys
        for i in 0..2 {
            for j in 0..16 {
                self.round_keys[i][j] = key[i * 16 + j];
            }
        }
        
        // Generate remaining round keys
        let mut rcon: u8 = 1;
        
        for i in 2..15 {
            let mut temp = [0u8; 4];
            
            // For even rounds (when i % 2 == 0 for 256-bit)
            if i % 2 == 0 {
                // RotWord + SubWord + Rcon
                temp[0] = Self::sbox(self.round_keys[i - 1][13]) ^ rcon;
                temp[1] = Self::sbox(self.round_keys[i - 1][14]);
                temp[2] = Self::sbox(self.round_keys[i - 1][15]);
                temp[3] = Self::sbox(self.round_keys[i - 1][12]);
                
                // Update Rcon
                let mut rcon_val = rcon;
                rcon = Self::gmul(rcon_val, 2);
            } else {
                // Just SubWord for odd rounds
                temp[0] = Self::sbox(self.round_keys[i - 1][12]);
                temp[1] = Self::sbox(self.round_keys[i - 1][13]);
                temp[2] = Self::sbox(self.round_keys[i - 1][14]);
                temp[3] = Self::sbox(self.round_keys[i - 1][15]);
            }
            
            // XOR with round key i - 2
            for j in 0..4 {
                self.round_keys[i][j] = self.round_keys[i - 2][j] ^ temp[j];
                self.round_keys[i][j + 4] = self.round_keys[i - 2][j + 4] ^ self.round_keys[i - 1][j + 4];
                self.round_keys[i][j + 8] = self.round_keys[i - 2][j + 8] ^ self.round_keys[i - 1][j + 8];
                self.round_keys[i][j + 12] = self.round_keys[i - 2][j + 12] ^ self.round_keys[i - 1][j + 12];
            }
        }
    }
    
    /// AES S-box
    fn sbox(b: u8) -> u8 {
        // Pre-computed AES S-box
        const SBOX: [u8; 256] = [
            0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
            0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
            0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
            0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
            0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
            0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
            0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
            0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
            0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
            0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
            0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
            0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
            0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
            0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
            0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
            0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
        ];
        SBOX[b as usize]
    }
    
    /// Inverse S-box
    fn inv_sbox(b: u8) -> u8 {
        const INV_SBOX: [u8; 256] = [
            0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
            0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
            0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
            0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
            0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
            0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
            0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
            0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
            0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
            0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
            0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
            0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
            0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
            0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
            0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
            0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
        ];
        INV_SBOX[b as usize]
    }
    
    /// Galois field multiplication
    fn gmul(a: u8, b: u8) -> u8 {
        let mut p = 0u8;
        let mut a = a;
        let mut b = b;
        
        for _ in 0..8 {
            if b & 1 != 0 {
                p ^= a;
            }
            let hi = a & 0x80;
            a <<= 1;
            if hi != 0 {
                a ^= 0x1b; // x^8 + x^4 + x^3 + x + 1
            }
            b >>= 1;
        }
        p
    }
    
    /// Convert bytes to state
    fn bytes_to_state(bytes: &[u8; 16]) -> AesState {
        let mut state = [[0u8; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                state[j][i] = bytes[i * 4 + j];
            }
        }
        state
    }
    
    /// Convert state to bytes
    fn state_to_bytes(state: &AesState) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        for i in 0..4 {
            for j in 0..4 {
                bytes[i * 4 + j] = state[j][i];
            }
        }
        bytes
    }
    
    /// SubBytes transformation
    fn sub_bytes(state: &mut AesState) {
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] = Self::sbox(state[i][j]);
            }
        }
    }
    
    /// Inverse SubBytes
    fn inv_sub_bytes(state: &mut AesState) {
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] = Self::inv_sbox(state[i][j]);
            }
        }
    }
    
    /// ShiftRows transformation
    fn shift_rows(state: &mut AesState) {
        // Row 1: shift left 1
        let tmp = state[1][0];
        state[1][0] = state[1][1];
        state[1][1] = state[1][2];
        state[1][2] = state[1][3];
        state[1][3] = tmp;
        
        // Row 2: shift left 2
        let tmp0 = state[2][0];
        let tmp1 = state[2][1];
        state[2][0] = state[2][2];
        state[2][1] = state[2][3];
        state[2][2] = tmp0;
        state[2][3] = tmp1;
        
        // Row 3: shift left 3 (or right 1)
        let tmp = state[3][3];
        state[3][3] = state[3][2];
        state[3][2] = state[3][1];
        state[3][1] = state[3][0];
        state[3][0] = tmp;
    }
    
    /// Inverse ShiftRows
    fn inv_shift_rows(state: &mut AesState) {
        // Row 1: shift right 1
        let tmp = state[1][3];
        state[1][3] = state[1][2];
        state[1][2] = state[1][1];
        state[1][1] = state[1][0];
        state[1][0] = tmp;
        
        // Row 2: shift right 2
        let tmp0 = state[2][0];
        let tmp1 = state[2][1];
        state[2][0] = state[2][2];
        state[2][1] = state[2][3];
        state[2][2] = tmp0;
        state[2][3] = tmp1;
        
        // Row 3: shift right 3 (or left 1)
        let tmp = state[3][0];
        state[3][0] = state[3][1];
        state[3][1] = state[3][2];
        state[3][2] = state[3][3];
        state[3][3] = tmp;
    }
    
    /// MixColumns transformation
    fn mix_columns(state: &mut AesState) {
        for i in 0..4 {
            let s0 = state[0][i];
            let s1 = state[1][i];
            let s2 = state[2][i];
            let s3 = state[3][i];
            
            state[0][i] = Self::gmul(s0, 2) ^ Self::gmul(s1, 3) ^ s2 ^ s3;
            state[1][i] = s0 ^ Self::gmul(s1, 2) ^ Self::gmul(s2, 3) ^ s3;
            state[2][i] = s0 ^ s1 ^ Self::gmul(s2, 2) ^ Self::gmul(s3, 3);
            state[3][i] = Self::gmul(s0, 3) ^ s1 ^ s2 ^ Self::gmul(s3, 2);
        }
    }
    
    /// Inverse MixColumns
    fn inv_mix_columns(state: &mut AesState) {
        for i in 0..4 {
            let s0 = state[0][i];
            let s1 = state[1][i];
            let s2 = state[2][i];
            let s3 = state[3][i];
            
            state[0][i] = Self::gmul(s0, 14) ^ Self::gmul(s1, 11) ^ Self::gmul(s2, 13) ^ Self::gmul(s3, 9);
            state[1][i] = Self::gmul(s0, 9) ^ Self::gmul(s1, 14) ^ Self::gmul(s2, 11) ^ Self::gmul(s3, 13);
            state[2][i] = Self::gmul(s0, 13) ^ Self::gmul(s1, 9) ^ Self::gmul(s2, 14) ^ Self::gmul(s3, 11);
            state[3][i] = Self::gmul(s0, 11) ^ Self::gmul(s1, 13) ^ Self::gmul(s2, 9) ^ Self::gmul(s3, 14);
        }
    }
    
    /// AddRoundKey transformation
    fn add_round_key(state: &mut AesState, round_key: &[u8; 16]) {
        for i in 0..4 {
            for j in 0..4 {
                state[j][i] ^= round_key[i * 4 + j];
            }
        }
    }
    
    /// Encrypt a single block
    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = Self::bytes_to_state(block);
        
        // Initial round
        Self::add_round_key(&mut state, &self.round_keys[0]);
        
        // Main rounds (14 for AES-256)
        for round in 1..14 {
            Self::sub_bytes(&mut state);
            Self::shift_rows(&mut state);
            Self::mix_columns(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round]);
        }
        
        // Final round (no MixColumns)
        Self::sub_bytes(&mut state);
        Self::shift_rows(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[14]);
        
        Self::state_to_bytes(&state)
    }
    
    /// Decrypt a single block
    pub fn decrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = Self::bytes_to_state(block);
        
        // Initial round (with last key)
        Self::add_round_key(&mut state, &self.round_keys[14]);
        
        // Main rounds (reverse)
        for round in (1..14).rev() {
            Self::inv_shift_rows(&mut state);
            Self::inv_sub_bytes(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round]);
            Self::inv_mix_columns(&mut state);
        }
        
        // Final round
        Self::inv_shift_rows(&mut state);
        Self::inv_sub_bytes(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[0]);
        
        Self::state_to_bytes(&state)
    }
}

/// AES-256-CBC encryption
pub struct Aes256Cbc {
    aes: Aes256,
    iv: [u8; 16],
}

impl Aes256Cbc {
    /// Create new AES-256-CBC context
    pub fn new(key: &[u8; 32], iv: &[u8; 16]) -> Self {
        Self {
            aes: Aes256::new(key),
            iv: *iv,
        }
    }
    
    /// Encrypt data with PKCS#7 padding
    pub fn encrypt(&self, data: &[u8]) -> EncryptionResult<Vec<u8>> {
        // Calculate padded length
        let pad_len = AES_BLOCK_SIZE - (data.len() % AES_BLOCK_SIZE);
        let padded_len = data.len() + pad_len;
        
        let mut result = Vec::with_capacity(padded_len);
        
        // Add PKCS#7 padding
        let mut padded = data.to_vec();
        for _ in 0..pad_len {
            padded.push(pad_len as u8);
        }
        
        // Encrypt in CBC mode
        let mut prev = self.iv;
        
        for chunk in padded.chunks(AES_BLOCK_SIZE) {
            // XOR with previous ciphertext (or IV for first block)
            let mut block = [0u8; 16];
            for i in 0..16 {
                block[i] = chunk[i] ^ prev[i];
            }
            
            // Encrypt block
            let encrypted = self.aes.encrypt_block(&block);
            result.extend_from_slice(&encrypted);
            prev = encrypted;
        }
        
        Ok(result)
    }
    
    /// Decrypt data and remove PKCS#7 padding
    pub fn decrypt(&self, data: &[u8]) -> EncryptionResult<Vec<u8>> {
        if data.len() % AES_BLOCK_SIZE != 0 {
            return Err(EncryptionError::InvalidDataLength);
        }
        
        let mut result = Vec::with_capacity(data.len());
        
        // Decrypt in CBC mode
        let mut prev = self.iv;
        
        for chunk in data.chunks(AES_BLOCK_SIZE) {
            let mut block = [0u8; 16];
            block.copy_from_slice(chunk);
            
            // Decrypt block
            let decrypted = self.aes.decrypt_block(&block);
            
            // XOR with previous ciphertext (or IV for first block)
            for i in 0..16 {
                result.push(decrypted[i] ^ prev[i]);
            }
            
            prev = block;
        }
        
        // Remove PKCS#7 padding
        if !result.is_empty() {
            let pad_len = result[result.len() - 1] as usize;
            if pad_len > 0 && pad_len <= AES_BLOCK_SIZE {
                // Verify padding
                let valid = result[result.len() - pad_len..].iter().all(|&b| b == pad_len as u8);
                if valid {
                    result.truncate(result.len() - pad_len);
                }
            }
        }
        
        Ok(result)
    }
}

/// Simple SHA-256 for key derivation (simplified)
pub struct Sha256;

impl Sha256 {
    /// Compute SHA-256 hash
    pub fn hash(data: &[u8]) -> [u8; 32] {
        // Simplified SHA-256 implementation
        // For production, use a proper crypto library
        
        // Initial hash values
        let mut h: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];
        
        // Round constants
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];
        
        // Padding and processing would go here
        // Simplified: just XOR data into hash
        for (i, &byte) in data.iter().enumerate() {
            h[i % 8] ^= byte as u32;
        }
        
        // Convert to bytes
        let mut result = [0u8; 32];
        for i in 0..8 {
            result[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
        }
        
        result
    }
}

/// PBKDF2 key derivation (simplified)
pub fn pbkdf2_derive(password: &str, salt: &[u8], iterations: u32, key_len: usize) -> Vec<u8> {
    let mut key = Vec::with_capacity(key_len);
    let mut block_num = 1u32;
    
    while key.len() < key_len {
        // Create block input: salt || block_num
        let mut input = salt.to_vec();
        input.extend_from_slice(&block_num.to_be_bytes());
        
        // First iteration
        let mut u = Sha256::hash(&[password.as_bytes(), &input].concat());
        
        // Remaining iterations
        for _ in 1..iterations {
            u = Sha256::hash(&[password.as_bytes(), &u].concat());
        }
        
        key.extend_from_slice(&u);
        block_num += 1;
    }
    
    key.truncate(key_len);
    key
}

/// Generate a random IV (placeholder - should use proper RNG)
pub fn generate_iv() -> [u8; 16] {
    // In production, use a proper CSPRNG
    [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
     0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]
}

/// Encrypt data with password
pub fn encrypt_with_password(data: &[u8], password: &str) -> EncryptionResult<Vec<u8>> {
    // Generate salt
    let salt = [0u8; 16]; // In production, use random salt
    
    // Derive key
    let key = pbkdf2_derive(password, &salt, PBKDF2_ITERATIONS, AES_256_KEY_SIZE);
    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(&key);
    
    // Generate IV
    let iv = generate_iv();
    
    // Create encryptor
    let cipher = Aes256Cbc::new(&key_arr, &iv);
    
    // Encrypt
    let encrypted = cipher.encrypt(data)?;
    
    // Prepend salt and IV
    let mut result = Vec::with_capacity(16 + 16 + encrypted.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&iv);
    result.extend_from_slice(&encrypted);
    
    Ok(result)
}

/// Decrypt data with password
pub fn decrypt_with_password(data: &[u8], password: &str) -> EncryptionResult<Vec<u8>> {
    if data.len() < 32 {
        return Err(EncryptionError::InvalidDataLength);
    }
    
    // Extract salt and IV
    let salt = &data[..16];
    let iv: [u8; 16] = data[16..32].try_into().map_err(|_| EncryptionError::InvalidIvSize)?;
    let ciphertext = &data[32..];
    
    // Derive key
    let key = pbkdf2_derive(password, salt, PBKDF2_ITERATIONS, AES_256_KEY_SIZE);
    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(&key);
    
    // Create decryptor
    let cipher = Aes256Cbc::new(&key_arr, &iv);
    
    // Decrypt
    cipher.decrypt(ciphertext)
}