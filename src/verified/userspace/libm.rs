// Math Library (libm)
// Mathematical functions

// ============================================================================
// Trigonometric Functions
// ============================================================================

/// sin - sine function (simplified)
pub fn sin(x: f64) -> f64 {
    // Simplified implementation using Taylor series
    // In real implementation, this would use more accurate algorithms
    let mut result = 0.0;
    let mut term = x;
    let mut sign = 1.0;
    
    for i in 1..=10 {
        result += sign * term;
        sign *= -1.0;
        term *= x * x / ((2 * i) * (2 * i + 1)) as f64;
    }
    
    result
}

/// cos - cosine function (simplified)
pub fn cos(x: f64) -> f64 {
    // Simplified implementation using Taylor series
    let mut result = 0.0;
    let mut term = 1.0;
    let mut sign = 1.0;
    
    for i in 0..=10 {
        result += sign * term;
        sign *= -1.0;
        term *= x * x / ((2 * i + 1) * (2 * i + 2)) as f64;
    }
    
    result
}

/// tan - tangent function
pub fn tan(x: f64) -> f64 {
    sin(x) / cos(x)
}

/// asin - arcsine function
pub fn asin(x: f64) -> f64 {
    // Simplified implementation
    // In real implementation, this would use more accurate algorithms
    if x < -1.0 || x > 1.0 {
        return f64::NAN;
    }
    
    // Newton-Raphson iteration
    let mut result = x;
    for _ in 0..10 {
        result = result - (sin(result) - x) / cos(result);
    }
    
    result
}

/// acos - arccosine function
pub fn acos(x: f64) -> f64 {
    std::f64::consts::FRAC_PI_2 - asin(x)
}

/// atan - arctangent function
pub fn atan(x: f64) -> f64 {
    // Simplified implementation
    // In real implementation, this would use more accurate algorithms
    if x.abs() < 1.0 {
        let mut result = x;
        for _ in 0..10 {
            result = result - (tan(result) - x) / (1.0 + tan(result) * tan(result));
        }
        result
    } else if x > 0.0 {
        std::f64::consts::FRAC_PI_2 - atan(1.0 / x)
    } else {
        -std::f64::consts::FRAC_PI_2 - atan(1.0 / x)
    }
}

/// atan2 - arctangent of y/x
pub fn atan2(y: f64, x: f64) -> f64 {
    if x > 0.0 {
        atan(y / x)
    } else if x < 0.0 {
        if y >= 0.0 {
            atan(y / x) + std::f64::consts::PI
        } else {
            atan(y / x) - std::f64::consts::PI
        }
    } else {
        if y > 0.0 {
            std::f64::consts::FRAC_PI_2
        } else if y < 0.0 {
            -std::f64::consts::FRAC_PI_2
        } else {
            f64::NAN
        }
    }
}

// ============================================================================
// Hyperbolic Functions
// ============================================================================

/// sinh - hyperbolic sine
pub fn sinh(x: f64) -> f64 {
    (x.exp() - (-x).exp()) / 2.0
}

/// cosh - hyperbolic cosine
pub fn cosh(x: f64) -> f64 {
    (x.exp() + (-x).exp()) / 2.0
}

/// tanh - hyperbolic tangent
pub fn tanh(x: f64) -> f64 {
    sinh(x) / cosh(x)
}

/// asinh - inverse hyperbolic sine
pub fn asinh(x: f64) -> f64 {
    (x + (x * x + 1.0).sqrt()).ln()
}

/// acosh - inverse hyperbolic cosine
pub fn acosh(x: f64) -> f64 {
    if x < 1.0 {
        return f64::NAN;
    }
    (x + (x * x - 1.0).sqrt()).ln()
}

/// atanh - inverse hyperbolic tangent
pub fn atanh(x: f64) -> f64 {
    if x.abs() >= 1.0 {
        return f64::NAN;
    }
    0.5 * ((1.0 + x) / (1.0 - x)).ln()
}

// ============================================================================
// Exponential and Logarithmic Functions
// ============================================================================

/// exp - exponential function
pub fn exp(x: f64) -> f64 {
    x.exp()
}

/// log - natural logarithm
pub fn log(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    x.ln()
}

/// log10 - base-10 logarithm
pub fn log10(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    x.log10()
}

/// log2 - base-2 logarithm
pub fn log2(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    x.log2()
}

/// pow - power function
pub fn pow(x: f64, y: f64) -> f64 {
    if x < 0.0 && y.fract() != 0.0 {
        return f64::NAN;
    }
    x.powf(y)
}

/// sqrt - square root
pub fn sqrt(x: f64) -> f64 {
    if x < 0.0 {
        return f64::NAN;
    }
    x.sqrt()
}

/// cbrt - cube root
pub fn cbrt(x: f64) -> f64 {
    if x >= 0.0 {
        x.powf(1.0 / 3.0)
    } else {
        -(-x).powf(1.0 / 3.0)
    }
}

/// hypot - hypotenuse
pub fn hypot(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

// ============================================================================
// Rounding Functions
// ============================================================================

/// ceil - ceiling function
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

/// floor - floor function
pub fn floor(x: f64) -> f64 {
    x.floor()
}

/// round - round to nearest integer
pub fn round(x: f64) -> f64 {
    x.round()
}

/// trunc - truncate toward zero
pub fn trunc(x: f64) -> f64 {
    x.trunc()
}

/// fmod - floating-point remainder
pub fn fmod(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        return f64::NAN;
    }
    x % y
}

/// remainder - IEEE remainder
pub fn remainder(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        return f64::NAN;
    }
    x - y * (x / y).round()
}

// ============================================================================
// Other Functions
// ============================================================================

/// fabs - absolute value
pub fn fabs(x: f64) -> f64 {
    x.abs()
}

/// fmin - minimum of two floats
pub fn fmin(x: f64, y: f64) -> f64 {
    x.min(y)
}

/// fmax - maximum of two floats
pub fn fmax(x: f64, y: f64) -> f64 {
    x.max(y)
}

/// fdim - positive difference
pub fn fdim(x: f64, y: f64) -> f64 {
    if x > y {
        x - y
    } else {
        0.0
    }
}

/// copysign - copy sign
pub fn copysign(x: f64, y: f64) -> f64 {
    if y.is_sign_negative() {
        -x.abs()
    } else {
        x.abs()
    }
}

/// signbit - sign bit
pub fn signbit(x: f64) -> bool {
    x.is_sign_negative()
}

/// isnan - check for NaN
pub fn isnan(x: f64) -> bool {
    x.is_nan()
}

/// isinf - check for infinity
pub fn isinf(x: f64) -> bool {
    x.is_infinite()
}

/// isfinite - check for finite
pub fn isfinite(x: f64) -> bool {
    x.is_finite()
}

// ============================================================================
// Constants
// ============================================================================

/// Pi constant
pub const M_PI: f64 = std::f64::consts::PI;

/// Pi/2 constant
pub const M_PI_2: f64 = std::f64::consts::FRAC_PI_2;

/// Pi/4 constant
pub const M_PI_4: f64 = std::f64::consts::FRAC_PI_4;

/// 2*Pi constant
pub const M_2PI: f64 = 2.0 * std::f64::consts::PI;

/// e constant
pub const M_E: f64 = std::f64::consts::E;

/// ln(2) constant
pub const M_LN2: f64 = std::f64::consts::LN_2;

/// ln(10) constant
pub const M_LN10: f64 = std::f64::consts::LN_10;

/// log10(e) constant
pub const M_LOG10E: f64 = std::f64::consts::LOG10_E;

/// log2(e) constant
pub const M_LOG2E: f64 = std::f64::consts::LOG2_E;

/// sqrt(2) constant
pub const M_SQRT2: f64 = std::f64::consts::SQRT_2;

/// sqrt(1/2) constant
pub const M_SQRT1_2: f64 = std::f64::consts::FRAC_1_SQRT_2;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sin() {
        assert!((sin(0.0) - 0.0).abs() < 1e-10);
        assert!((sin(std::f64::consts::PI / 2.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cos() {
        assert!((cos(0.0) - 1.0).abs() < 1e-10);
        assert!((cos(std::f64::consts::PI) - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt() {
        assert!((sqrt(4.0) - 2.0).abs() < 1e-10);
        assert!((sqrt(9.0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_pow() {
        assert!((pow(2.0, 3.0) - 8.0).abs() < 1e-10);
        assert!((pow(3.0, 2.0) - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_fabs() {
        assert!((fabs(3.14) - 3.14).abs() < 1e-10);
        assert!((fabs(-3.14) - 3.14).abs() < 1e-10);
    }
}