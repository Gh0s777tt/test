//! Cryptographic functions

use alloc::vec::Vec;

/// Initialize cryptographic subsystem
pub fn init() {
    // Initialize RNG
    // Initialize crypto library
}

/// Simple hash function (placeholder)
pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    
    // Simple hash (NOT cryptographically secure - just for demonstration)
    let mut state: u64 = 0x1234567890ABCDEF;
    for (i, &byte) in data.iter().enumerate() {
        state = state.wrapping_mul(0x5851F42D4C957F2D);
        state ^= byte as u64;
        state ^= (i as u64).wrapping_mul(0x14057B7EF767814F);
    }
    
    // Expand to 32 bytes
    for i in 0..4 {
        let v = state.wrapping_mul(0x5851F42D4C957F2D).wrapping_add(i as u64);
        result[i * 8..(i + 1) * 8].copy_from_slice(&v.to_le_bytes());
    }
    
    result
}

/// Simple random number generator
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Create a new RNG
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    /// Generate a random u64
    pub fn next_u64(&mut self) -> u64 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    
    /// Generate a random u32
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    
    /// Generate a random byte
    pub fn next_u8(&mut self) -> u8 {
        self.next_u64() as u8
    }
    
    /// Fill a buffer with random bytes
    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        for byte in buf.iter_mut() {
            *byte = self.next_u8();
        }
    }
}

/// Global RNG instance
pub static mut RNG: Option<Rng> = None;

/// Get random bytes
pub fn random_bytes(buf: &mut [u8]) {
    unsafe {
        if let Some(ref mut rng) = RNG {
            rng.fill_bytes(buf);
        }
    }
}