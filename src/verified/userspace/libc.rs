// Standard C Library (libc)
// String functions, memory functions, I/O functions, math functions

use alloc::vec::Vec;
use alloc::string::String;
use core::mem;

// ============================================================================
// String Functions
// ============================================================================

/// strlen - calculate string length
pub fn strlen(s: *const u8) -> usize {
    if s.is_null() {
        return 0;
    }

    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 {
            len += 1;
        }
    }
    len
}

/// strcpy - copy string
pub fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }

    unsafe {
        let mut i = 0;
        loop {
            let c = *src.add(i);
            *dest.add(i) = c;
            if c == 0 {
                break;
            }
            i += 1;
        }
    }
    dest
}

/// strncpy - copy string with limit
pub fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }

    unsafe {
        for i in 0..n {
            let c = *src.add(i);
            *dest.add(i) = c;
            if c == 0 {
                // Fill remaining with zeros
                for j in (i + 1)..n {
                    *dest.add(j) = 0;
                }
                break;
            }
        }
    }
    dest
}

/// strcat - concatenate strings
pub fn strcat(dest: *mut u8, src: *const u8) -> *mut u8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }

    let dest_len = strlen(dest);
    strcpy(unsafe { dest.add(dest_len) }, src);
    dest
}

/// strcmp - compare strings
pub fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    if s1.is_null() || s2.is_null() {
        return if s1.is_null() && s2.is_null() { 0 } else if s1.is_null() { -1 } else { 1 };
    }

    unsafe {
        let mut i = 0;
        loop {
            let c1 = *s1.add(i);
            let c2 = *s2.add(i);
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
            if c1 == 0 {
                return 0;
            }
            i += 1;
        }
    }
}

/// strncmp - compare strings with limit
pub fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    if s1.is_null() || s2.is_null() || n == 0 {
        return 0;
    }

    unsafe {
        for i in 0..n {
            let c1 = *s1.add(i);
            let c2 = *s2.add(i);
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
            if c1 == 0 {
                return 0;
            }
        }
    }
    0
}

/// strchr - find character in string
pub fn strchr(s: *const u8, c: i32) -> *const u8 {
    if s.is_null() {
        return core::ptr::null();
    }

    unsafe {
        let mut i = 0;
        loop {
            let sc = *s.add(i);
            if sc == (c as u8) {
                return s.add(i);
            }
            if sc == 0 {
                break;
            }
            i += 1;
        }
    }
    core::ptr::null()
}

/// strstr - find substring in string
pub fn strstr(haystack: *const u8, needle: *const u8) -> *const u8 {
    if haystack.is_null() || needle.is_null() {
        return core::ptr::null();
    }

    let needle_len = strlen(needle);
    if needle_len == 0 {
        return haystack;
    }

    let haystack_len = strlen(haystack);
    if needle_len > haystack_len {
        return core::ptr::null();
    }

    unsafe {
        for i in 0..=(haystack_len - needle_len) {
            let mut found = true;
            for j in 0..needle_len {
                if *haystack.add(i + j) != *needle.add(j) {
                    found = false;
                    break;
                }
            }
            if found {
                return haystack.add(i);
            }
        }
    }
    core::ptr::null()
}

// ============================================================================
// Memory Functions
// ============================================================================

/// memcpy - copy memory
pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }

    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

/// memmove - copy memory with overlap handling
pub fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }

    // Check for overlap
    let dest_addr = dest as usize;
    let src_addr = src as usize;

    if dest_addr < src_addr && dest_addr + n > src_addr {
        // Overlap: copy backwards
        unsafe {
            for i in (0..n).rev() {
                *dest.add(i) = *src.add(i);
            }
        }
    } else {
        // No overlap or safe to copy forwards
        memcpy(dest, src, n);
    }
    dest
}

/// memset - set memory
pub fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    if s.is_null() || n == 0 {
        return s;
    }

    unsafe {
        for i in 0..n {
            *s.add(i) = c as u8;
        }
    }
    s
}

/// memcmp - compare memory
pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    if s1.is_null() || s2.is_null() || n == 0 {
        return 0;
    }

    unsafe {
        for i in 0..n {
            let c1 = *s1.add(i);
            let c2 = *s2.add(i);
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
        }
    }
    0
}

// ============================================================================
// I/O Functions
// ============================================================================

/// printf - formatted output (simplified)
pub fn printf(format: *const u8, args: &[u64]) -> i32 {
    if format.is_null() {
        return -1;
    }

    let mut written = 0;
    let mut arg_idx = 0;
    let mut i = 0;

    unsafe {
        loop {
            let c = *format.add(i);
            if c == 0 {
                break;
            }

            if c == b'%' {
                i += 1;
                let spec = *format.add(i);
                match spec {
                    b'd' | b'i' => {
                        // Integer
                        if arg_idx < args.len() {
                            let val = args[arg_idx] as i32;
                            written += print_integer(val);
                            arg_idx += 1;
                        }
                    }
                    b'u' => {
                        // Unsigned integer
                        if arg_idx < args.len() {
                            let val = args[arg_idx] as u32;
                            written += print_unsigned(val);
                            arg_idx += 1;
                        }
                    }
                    b'x' => {
                        // Hexadecimal
                        if arg_idx < args.len() {
                            let val = args[arg_idx] as u32;
                            written += print_hex(val);
                            arg_idx += 1;
                        }
                    }
                    b's' => {
                        // String
                        if arg_idx < args.len() {
                            let s = args[arg_idx] as *const u8;
                            written += print_string(s);
                            arg_idx += 1;
                        }
                    }
                    b'c' => {
                        // Character
                        if arg_idx < args.len() {
                            let c = args[arg_idx] as u8;
                            print_char(c);
                            written += 1;
                            arg_idx += 1;
                        }
                    }
                    b'%' => {
                        // Literal %
                        print_char(b'%');
                        written += 1;
                    }
                    _ => {
                        // Unknown format specifier
                        print_char(b'%');
                        print_char(spec);
                        written += 2;
                    }
                }
            } else {
                print_char(c);
                written += 1;
            }
            i += 1;
        }
    }

    written
}

/// Helper function to print integer
fn print_integer(val: i32) -> usize {
    if val < 0 {
        print_char(b'-');
        print_unsigned((-val) as u32) + 1
    } else {
        print_unsigned(val as u32)
    }
}

/// Helper function to print unsigned integer
fn print_unsigned(mut val: u32) -> usize {
    let mut buffer = [0u8; 20];
    let mut len = 0;

    if val == 0 {
        buffer[len] = b'0';
        len += 1;
    } else {
        while val > 0 {
            buffer[len] = (val % 10) as u8 + b'0';
            val /= 10;
            len += 1;
        }
    }

    // Reverse and print
    for i in (0..len).rev() {
        print_char(buffer[i]);
    }

    len
}

/// Helper function to print hexadecimal
fn print_hex(mut val: u32) -> usize {
    let mut buffer = [0u8; 8];
    let mut len = 0;

    if val == 0 {
        buffer[len] = b'0';
        len += 1;
    } else {
        while val > 0 {
            let digit = (val & 0xF) as u8;
            buffer[len] = if digit < 10 {
                digit + b'0'
            } else {
                digit - 10 + b'a'
            };
            val >>= 4;
            len += 1;
        }
    }

    // Reverse and print
    for i in (0..len).rev() {
        print_char(buffer[i]);
    }

    len
}

/// Helper function to print string
fn print_string(s: *const u8) -> usize {
    if s.is_null() {
        print_string(b"(null)\0" as *const u8);
        return 6;
    }

    let mut len = 0;
    unsafe {
        loop {
            let c = *s.add(len);
            if c == 0 {
                break;
            }
            print_char(c);
            len += 1;
        }
    }
    len
}

/// Helper function to print character
fn print_char(c: u8) {
    // In real implementation, this would write to stdout
    // For now, this is a placeholder
    // TODO: Implement actual character output
}

// ============================================================================
// Math Functions
// ============================================================================

/// abs - absolute value
pub fn abs(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

/// labs - absolute value of long
pub fn labs(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

/// min - minimum of two integers
pub fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

/// max - maximum of two integers
pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// div - division with remainder
pub fn div(numer: i32, denom: i32) -> (i32, i32) {
    (numer / denom, numer % denom)
}

/// ldiv - long division with remainder
pub fn ldiv(numer: i64, denom: i64) -> (i64, i64) {
    (numer / denom, numer % denom)
}

// ============================================================================
// Conversion Functions
// ============================================================================

/// atoi - convert string to integer
pub fn atoi(s: *const u8) -> i32 {
    if s.is_null() {
        return 0;
    }

    let mut result: i32 = 0;
    let mut sign: i32 = 1;
    let mut i = 0;

    unsafe {
        // Skip whitespace
        while *s.add(i) == b' ' || *s.add(i) == b'\t' || *s.add(i) == b'\n' {
            i += 1;
        }

        // Check sign
        if *s.add(i) == b'-' {
            sign = -1;
            i += 1;
        } else if *s.add(i) == b'+' {
            i += 1;
        }

        // Parse digits
        while *s.add(i) >= b'0' && *s.add(i) <= b'9' {
            result = result * 10 + (*s.add(i) - b'0') as i32;
            i += 1;
        }
    }

    sign * result
}

/// atol - convert string to long
pub fn atol(s: *const u8) -> i64 {
    if s.is_null() {
        return 0;
    }

    let mut result: i64 = 0;
    let mut sign: i64 = 1;
    let mut i = 0;

    unsafe {
        // Skip whitespace
        while *s.add(i) == b' ' || *s.add(i) == b'\t' || *s.add(i) == b'\n' {
            i += 1;
        }

        // Check sign
        if *s.add(i) == b'-' {
            sign = -1;
            i += 1;
        } else if *s.add(i) == b'+' {
            i += 1;
        }

        // Parse digits
        while *s.add(i) >= b'0' && *s.add(i) <= b'9' {
            result = result * 10 + (*s.add(i) - b'0') as i64;
            i += 1;
        }
    }

    sign * result
}

/// itoa - convert integer to string
pub fn itoa(value: i32, str: *mut u8, radix: i32) -> *mut u8 {
    if str.is_null() || radix < 2 || radix > 36 {
        return str;
    }

    let mut value = value;
    let mut i = 0;
    let mut is_negative = false;

    unsafe {
        if value < 0 && radix == 10 {
            is_negative = true;
            value = -value;
        }

        // Handle zero
        if value == 0 {
            *str.add(i) = b'0';
            i += 1;
        } else {
            // Convert to string
            while value != 0 {
                let digit = (value % radix) as u8;
                *str.add(i) = if digit < 10 {
                    digit + b'0'
                } else {
                    digit - 10 + b'a'
                };
                value /= radix;
                i += 1;
            }
        }

        // Add sign
        if is_negative {
            *str.add(i) = b'-';
            i += 1;
        }

        // Null-terminate
        *str.add(i) = 0;

        // Reverse string
        let len = i;
        for j in 0..(len / 2) {
            let temp = *str.add(j);
            *str.add(j) = *str.add(len - 1 - j);
            *str.add(len - 1 - j) = temp;
        }
    }

    str
}

// ============================================================================
// Utility Functions
// ============================================================================

/// exit - terminate process
pub fn exit(status: i32) -> ! {
    // In real implementation, this would call the exit system call
    loop {}
}

/// abort - abort process
pub fn abort() -> ! {
    // In real implementation, this would raise SIGABRT
    loop {}
}

/// assert - assertion macro (function version)
pub fn assert(condition: bool) {
    if !condition {
        abort();
    }
}

/// getenv - get environment variable
pub fn getenv(name: *const u8) -> *const u8 {
    // In real implementation, this would search environment variables
    core::ptr::null()
}

/// setenv - set environment variable
pub fn setenv(name: *const u8, value: *const u8, overwrite: i32) -> i32 {
    // In real implementation, this would set environment variable
    0
}

/// unsetenv - unset environment variable
pub fn unsetenv(name: *const u8) -> i32 {
    // In real implementation, this would unset environment variable
    0
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strlen() {
        let s = b"Hello\0";
        assert_eq!(strlen(s.as_ptr()), 5);
    }

    #[test]
    fn test_strcmp() {
        let s1 = b"Hello\0";
        let s2 = b"Hello\0";
        let s3 = b"World\0";
        assert_eq!(strcmp(s1.as_ptr(), s2.as_ptr()), 0);
        assert!(strcmp(s1.as_ptr(), s3.as_ptr()) < 0);
    }

    #[test]
    fn test_memcpy() {
        let mut dest = [0u8; 10];
        let src = [1u8, 2, 3, 4, 5];
        memcpy(dest.as_mut_ptr(), src.as_ptr(), 5);
        assert_eq!(&dest[..5], &src[..5]);
    }

    #[test]
    fn test_memset() {
        let mut buf = [0u8; 10];
        memset(buf.as_mut_ptr(), 0xFF, 10);
        assert_eq!(buf, [0xFFu8; 10]);
    }

    #[test]
    fn test_atoi() {
        assert_eq!(atoi(b"123\0".as_ptr()), 123);
        assert_eq!(atoi(b"-456\0".as_ptr()), -456);
        assert_eq!(atoi(b"  789\0".as_ptr()), 789);
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs(42), 42);
        assert_eq!(abs(-42), 42);
    }
}