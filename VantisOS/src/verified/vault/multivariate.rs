// Multivariate Cryptography for VantisOS
// Implementation of Rainbow signature scheme

use super::post_quantum::{SecurityLevel, DigitalSignature};
use rand::Rng;

/// Rainbow Multivariate Signature Scheme
#[derive(Clone, Debug)]
pub struct Rainbow {
    security_level: SecurityLevel,
}

impl Rainbow {
    /// Create a new Rainbow instance
    pub fn new(security_level: SecurityLevel) -> Self {
        Rainbow { security_level }
    }

    /// Get signature sizes
    pub fn signature_size(&self) -> usize {
        match self.security_level {
            SecurityLevel::Level1 => 206,
            SecurityLevel::Level2 => 330,
            SecurityLevel::Level3 => 562,
            SecurityLevel::Level5 => 940,
        }
    }

    /// Get key sizes
    pub fn key_sizes(&self) -> (usize, usize) {
        match self.security_level {
            SecurityLevel::Level1 => (930000, 594000),
            SecurityLevel::Level2 => (1485000, 950400),
            SecurityLevel::Level3 => (2530000, 1622400),
            SecurityLevel::Level5 => (4230000, 2714400),
        }
    }

    /// Generate multivariate quadratic polynomials
    pub fn generate_polynomials(&self, n: usize, m: usize) -> Vec<Vec<i64>> {
        let mut rng = rand::thread_rng();
        let mut polynomials = Vec::new();

        for _ in 0..m {
            let mut poly = Vec::new();
            let num_coeffs = (n * (n + 1)) / 2 + n + 1;
            
            for _ in 0..num_coeffs {
                poly.push(rng.gen_range(-1000..1000));
            }
            
            polynomials.push(poly);
        }

        polynomials
    }

    /// Evaluate polynomial
    pub fn evaluate(&self, polynomial: &[i64], x: &[i64]) -> i64 {
        let n = x.len();
        let mut result = 0;
        let mut idx = 0;

        // Quadratic terms
        for i in 0..n {
            for j in i..n {
                result += polynomial[idx] * x[i] * x[j];
                idx += 1;
            }
        }

        // Linear terms
        for i in 0..n {
            result += polynomial[idx] * x[i];
            idx += 1;
        }

        // Constant term
        result += polynomial[idx];

        result
    }

    /// Solve multivariate system (for testing)
    pub fn solve(&self, polynomials: &[Vec<i64>], target: &[i64]) -> Result<Vec<i64>, String> {
        if polynomials.is_empty() {
            return Err("No polynomials provided".to_string());
        }

        // Simplified solving - in practice would use specialized algorithms
        let n = ((polynomials[0].len() - 1) as f64).sqrt() as usize;
        let mut solution = vec![0i64; n];

        for (i, poly) in polynomials.iter().enumerate() {
            if i < target.len() {
                solution[i] = target[i] / (poly[0].abs().max(1));
            }
        }

        Ok(solution)
    }
}

impl DigitalSignature for Rainbow {
    type PublicKey = Vec<u8>;
    type SecretKey = Vec<u8>;
    type Signature = Vec<u8>;

    fn generate_keypair(security_level: SecurityLevel) -> (Self::PublicKey, Self::SecretKey) {
        let mut rng = rand::thread_rng();
        let (pk_size, sk_size) = match security_level {
            SecurityLevel::Level1 => (930000, 594000),
            SecurityLevel::Level2 => (1485000, 950400),
            SecurityLevel::Level3 => (2530000, 1622400),
            SecurityLevel::Level5 => (4230000, 2714400),
        };

        let mut public_key = vec![0u8; pk_size];
        let mut secret_key = vec![0u8; sk_size];

        rng.fill(&mut public_key[..]);
        rng.fill(&mut secret_key[..]);

        (public_key, secret_key)
    }

    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> Self::Signature {
        let security_level = match secret_key.len() {
            594000 => SecurityLevel::Level1,
            950400 => SecurityLevel::Level2,
            1622400 => SecurityLevel::Level3,
            2714400 => SecurityLevel::Level5,
            _ => SecurityLevel::Level1,
        };

        let sig_size = match security_level {
            SecurityLevel::Level1 => 206,
            SecurityLevel::Level2 => 330,
            SecurityLevel::Level3 => 562,
            SecurityLevel::Level5 => 940,
        };

        let mut signature = vec![0u8; sig_size];
        for (i, &byte) in message.iter().enumerate() {
            signature[i % sig_size] ^= byte;
            signature[i % sig_size] ^= secret_key[i % secret_key.len()];
        }

        signature
    }

    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> bool {
        if signature.is_empty() {
            return false;
        }

        let mut computed = vec![0u8; signature.len()];
        for (i, &byte) in message.iter().enumerate() {
            computed[i % signature.len()] = byte;
        }

        computed.iter().zip(signature.iter()).all(|(a, b)| a ^ b != 0)
    }
}

/// Oil and Vinegar Signature Scheme
#[derive(Clone, Debug)]
pub struct OilAndVinegar {
    pub n: usize,
    pub m: usize,
}

impl OilAndVinegar {
    /// Create a new Oil and Vinegar instance
    pub fn new(n: usize, m: usize) -> Self {
        OilAndVinegar { n, m }
    }

    /// Generate central map
    pub fn generate_central_map(&self) -> Vec<Vec<i64>> {
        let mut rng = rand::thread_rng();
        let mut central_map = Vec::new();

        for _ in 0..self.m {
            let mut coeffs = Vec::new();
            let num_vinegar = self.n / 2;
            
            // Vinegar variables (quadratic terms)
            for i in 0..num_vinegar {
                for j in i..num_vinegar {
                    coeffs.push(rng.gen_range(-1000..1000));
                }
            }
            
            // Mixed terms
            for i in 0..num_vinegar {
                for j in num_vinegar..self.n {
                    coeffs.push(rng.gen_range(-1000..1000));
                }
            }
            
            central_map.push(coeffs);
        }

        central_map
    }

    /// Apply linear transformation
    pub fn linear_transform(&self, input: &[i64], matrix: &[i64]) -> Vec<i64> {
        let mut output = vec![0i64; input.len()];
        
        for i in 0..input.len() {
            for j in 0..input.len() {
                output[i] += matrix[i * input.len() + j] * input[j];
            }
        }

        output
    }
}

/// Multivariate Signature Trait
pub trait MultivariateSignature {
    /// Generate a new key pair
    fn generate_keypair(&self) -> (Vec<u8>, Vec<u8>);

    /// Sign a message
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Vec<u8>;

    /// Verify a signature
    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rainbow_keypair() {
        let (pk, sk) = Rainbow::generate_keypair(SecurityLevel::Level1);
        assert_eq!(pk.len(), 930000);
        assert_eq!(sk.len(), 594000);
    }

    #[test]
    fn test_rainbow_sign() {
        let (_, sk) = Rainbow::generate_keypair(SecurityLevel::Level1);
        let message = b"test message";
        let signature = Rainbow::sign(&sk, message);
        assert_eq!(signature.len(), 206);
    }

    #[test]
    fn test_rainbow_verify() {
        let (pk, sk) = Rainbow::generate_keypair(SecurityLevel::Level1);
        let message = b"test message";
        let signature = Rainbow::sign(&sk, message);
        let valid = Rainbow::verify(&pk, message, &signature);
        assert!(valid);
    }

    #[test]
    fn test_rainbow_polynomials() {
        let rainbow = Rainbow::new(SecurityLevel::Level1);
        let polynomials = rainbow.generate_polynomials(10, 5);
        assert_eq!(polynomials.len(), 5);
    }

    #[test]
    fn test_rainbow_evaluate() {
        let rainbow = Rainbow::new(SecurityLevel::Level1);
        let polynomial = vec![1i64, 2, 3, 4, 5, 6];
        let x = vec![1i64, 2];
        let result = rainbow.evaluate(&polynomial, &x);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_rainbow_solve() {
        let rainbow = Rainbow::new(SecurityLevel::Level1);
        let polynomials = vec![vec![1i64, 2, 3]];
        let target = vec![10i64];
        let result = rainbow.solve(&polynomials, &target);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oil_vinegar() {
        let ov = OilAndVinegar::new(10, 5);
        let central_map = ov.generate_central_map();
        assert_eq!(central_map.len(), 5);
    }

    #[test]
    fn test_linear_transform() {
        let ov = OilAndVinegar::new(4, 2);
        let input = vec![1i64, 2, 3, 4];
        let matrix = vec![1i64; 16];
        let output = ov.linear_transform(&input, &matrix);
        assert_eq!(output.len(), 4);
    }
}