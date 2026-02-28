# VantisOS User Space Guide

**Version**: 0.4.1  
**Date**: February 28, 2025  
**Status**: Production Ready

---

## Table of Contents

1. [Overview](#overview)
2. [User Space Architecture](#user-space-architecture)
3. [Memory Layout](#memory-layout)
4. [Standard Libraries](#standard-libraries)
5. [Applications](#applications)
6. [Testing](#testing)
7. [API Reference](#api-reference)
8. [Examples](#examples)
9. [Troubleshooting](#troubleshooting)

---

## Overview

VantisOS user space provides a complete Unix-like environment with standard libraries, applications, and utilities. The user space is designed to be POSIX-compatible and provides a familiar interface for developers and users.

### Key Features

- **Standard C Library (libc)**: Complete string, memory, I/O, and conversion functions
- **Math Library (libm)**: Comprehensive mathematical functions
- **Thread Library (libpthread)**: POSIX-compliant threading support
- **Dynamic Linker (ld.so)**: ELF64 dynamic linking and symbol resolution
- **Shell Application**: Interactive shell with 14 built-in commands
- **File Utilities**: 10 standard Unix file utilities
- **Network Utilities**: 9 standard Unix network utilities
- **Comprehensive Testing**: Integration, E2E, performance, and stress tests

---

## User Space Architecture

### Components

```
User Space
├── Memory Layout
│   ├── Code Segment (0x400000 - 0x500000)
│   ├── Data Segment (0x500000 - 0x600000)
│   ├── Heap Segment (0x600000 - dynamic)
│   └── Stack Segment (0x7ffff0000000 - 8MB)
├── Process Management
│   ├── User Space Process
│   ├── Process States (Created, Loading, Ready, Running, Blocked, Terminated)
│   └── Process Loader
├── System Call Interface
│   ├── System Call Numbers (50+ calls)
│   ├── System Call Dispatcher
│   └── System Call Validator
└── Libraries
    ├── libc (Standard C Library)
    ├── libm (Math Library)
    ├── libpthread (Thread Library)
    └── ldso (Dynamic Linker)
```

### Process States

1. **Created**: Process created but not loaded
2. **Loading**: Executable being loaded into memory
3. **Ready**: Process ready to run
4. **Running**: Process currently executing
5. **Blocked**: Process blocked (waiting for I/O, etc.)
6. **Terminated**: Process has terminated

---

## Memory Layout

### Default Memory Layout

```
+------------------+ 0x7ffff0000000
| Stack (8MB)      |
+------------------+
|                 |
|                 |
+------------------+
| Heap (dynamic)   |
+------------------+ 0x600000
| Data (1MB)       |
+------------------+ 0x500000
| Code (1MB)       |
+------------------+ 0x400000
```

### Memory Regions

| Region | Start Address | End Address | Size |
|--------|---------------|-------------|------|
| Code | 0x400000 | 0x500000 | 1MB |
| Data | 0x500000 | 0x600000 | 1MB |
| Heap | 0x600000 | Dynamic | Dynamic |
| Stack | 0x7ffff0000000 | 0x7ffff0000000 + 8MB | 8MB |

---

## Standard Libraries

### libc - Standard C Library

#### String Functions

```c
size_t strlen(const char *s);
char *strcpy(char *dest, const char *src);
char *strncpy(char *dest, const char *src, size_t n);
char *strcat(char *dest, const char *src);
int strcmp(const char *s1, const char *s2);
int strncmp(const char *s1, const char *s2, size_t n);
char *strchr(const char *s, int c);
char *strstr(const char *haystack, const char *needle);
```

#### Memory Functions

```c
void *memcpy(void *dest, const void *src, size_t n);
void *memmove(void *dest, const void *src, size_t n);
void *memset(void *s, int c, size_t n);
int memcmp(const void *s1, const void *s2, size_t n);
```

#### I/O Functions

```c
int printf(const char *format, ...);
```

**Format Specifiers**:
- `%d`, `%i`: Signed integer
- `%u`: Unsigned integer
- `%x`: Hexadecimal
- `%s`: String
- `%c`: Character
- `%%`: Literal %

#### Math Functions

```c
int abs(int x);
long labs(long x);
int min(int a, int b);
int max(int a, int b);
```

#### Conversion Functions

```c
int atoi(const char *s);
long atol(const char *s);
char *itoa(int value, char *str, int radix);
```

#### Utility Functions

```c
void exit(int status);
void abort(void);
char *getenv(const char *name);
int setenv(const char *name, const char *value, int overwrite);
int unsetenv(const char *name);
```

### libm - Math Library

#### Trigonometric Functions

```c
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);
```

#### Hyperbolic Functions

```c
double sinh(double x);
double cosh(double x);
double tanh(double x);
double asinh(double x);
double acosh(double x);
double atanh(double x);
```

#### Exponential and Logarithmic Functions

```c
double exp(double x);
double log(double x);
double log10(double x);
double log2(double x);
double pow(double x, double y);
double sqrt(double x);
double cbrt(double x);
double hypot(double x, double y);
```

#### Rounding Functions

```c
double ceil(double x);
double floor(double x);
double round(double x);
double trunc(double x);
double fmod(double x, double y);
double remainder(double x, double y);
```

#### Other Functions

```c
double fabs(double x);
double fmin(double x, double y);
double fmax(double x, double y);
double fdim(double x, double y);
double copysign(double x, double y);
int signbit(double x);
int isnan(double x);
int isinf(double x);
int isfinite(double x);
```

#### Constants

```c
#define M_PI        3.14159265358979323846
#define M_PI_2      1.57079632679489661923
#define M_PI_4      0.78539816339744830962
#define M_2PI       6.28318530717958647693
#define M_E         2.71828182845904523536
#define M_LN2       0.69314718055994530942
#define M_LN10      2.30258509299404568402
#define M_LOG10E    0.43429448190325182765
#define M_LOG2E     1.44269504088896340736
#define M_SQRT2     1.41421356237309504880
#define M_SQRT1_2   0.70710678118654752440
```

### libpthread - Thread Library

#### Thread Types

```c
typedef unsigned long pthread_t;
typedef struct pthread_attr_t pthread_attr_t;
typedef void *(*ThreadFunc)(void *arg);
```

#### Thread Functions

```c
int pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                   ThreadFunc func, void *arg);
void pthread_exit(void *retval);
int pthread_join(pthread_t thread, void **retval);
int pthread_detach(pthread_t thread);
int pthread_cancel(pthread_t thread);
pthread_t pthread_self(void);
int pthread_equal(pthread_t t1, pthread_t t2);
```

#### Mutex Functions

```c
int pthread_mutex_init(pthread_mutex_t *mutex, const pthread_mutexattr_t *attr);
int pthread_mutex_destroy(pthread_mutex_t *mutex);
int pthread_mutex_lock(pthread_mutex_t *mutex);
int pthread_mutex_trylock(pthread_mutex_t *mutex);
int pthread_mutex_unlock(pthread_mutex_t *mutex);
```

#### Condition Variable Functions

```c
int pthread_cond_init(pthread_cond_t *cond, const pthread_condattr_t *attr);
int pthread_cond_destroy(pthread_cond_t *cond);
int pthread_cond_wait(pthread_cond_t *cond, pthread_mutex_t *mutex);
int pthread_cond_signal(pthread_cond_t *cond);
int pthread_cond_broadcast(pthread_cond_t *cond);
```

### ldso - Dynamic Linker

#### ELF Types

```c
typedef enum {
    ELF32 = 1,
    ELF64 = 2
} ElfClass;

typedef enum {
    Relocatable = 1,
    Executable = 2,
    SharedObject = 3,
    Core = 4
} ElfType;

typedef enum {
    X86 = 3,
    X86_64 = 62,
    ARM = 40,
    AArch64 = 183
} ElfMachine;
```

#### Dynamic Linker Functions

```c
DynamicLinker *new_dynamic_linker(void);
LoadedLibrary *load_elf(DynamicLinker *linker, const uint8_t *data, size_t size);
uint64_t resolve_symbol(DynamicLinker *linker, const char *name);
int apply_relocations(DynamicLinker *linker, LoadedLibrary *library);
```

---

## Applications

### Shell Application

#### Built-in Commands

| Command | Description |
|---------|-------------|
| `exit [code]` | Exit shell with optional exit code |
| `cd [dir]` | Change directory |
| `pwd` | Print working directory |
| `ls [dir]` | List directory contents |
| `cat [file]` | Print file contents |
| `echo [args]` | Print arguments |
| `mkdir [dir]` | Create directory |
| `rm [file]` | Remove file |
| `cp [src] [dst]` | Copy file |
| `mv [src] [dst]` | Move/rename file |
| `env` | Print environment variables |
| `export VAR=value` | Set environment variable |
| `unset VAR` | Unset environment variable |
| `help` | Print help |

#### Command Features

- **Pipes**: `ls | grep test | sort`
- **Input Redirection**: `cat < input.txt`
- **Output Redirection**: `ls > output.txt`
- **Background Execution**: `sleep 10 &`

### File Utilities

| Utility | Description |
|---------|-------------|
| `wc [file]` | Word count (lines, words, bytes) |
| `head [-n] [file]` | Print first N lines |
| `tail [-n] [file]` | Print last N lines |
| `grep [pattern] [file]` | Search for pattern |
| `find [path] [name]` | Find files |
| `sort [file]` | Sort lines |
| `uniq [file]` | Print unique lines |
| `diff [file1] [file2]` | Compare files |
| `chmod [mode] [file]` | Change permissions |
| `chown [owner] [file]` | Change owner |

### Network Utilities

| Utility | Description |
|---------|-------------|
| `ping [host]` | Ping host |
| `ifconfig [interface]` | Configure network interface |
| `netstat` | Print network statistics |
| `ssh [user@host]` | Secure shell |
| `scp [src] [dst]` | Secure copy |
| `wget [url]` | Download file |
| `curl [url]` | Transfer data |
| `nc [host] [port]` | Netcat |
| `telnet [host] [port]` | Telnet client |

---

## Testing

### Test Suites

1. **Integration Tests** (6 tests)
   - libc + libm integration
   - libc + libpthread integration
   - libm + libpthread integration
   - ldso + libc integration
   - shell + libc integration
   - shell + apps integration

2. **End-to-End Tests** (4 tests)
   - Shell workflow
   - Command pipeline
   - File operations
   - Network operations

3. **Performance Tests** (4 tests)
   - String operations performance
   - Math operations performance
   - Memory operations performance
   - Command parsing performance

4. **Stress Tests** (4 tests)
   - Large string operations (100,000 bytes)
   - Many threads (1,000 threads)
   - Many ELF files (100 files)
   - Complex command pipelines (5 complex pipelines)

### Running Tests

```rust
use vantisos::userspace::testing::*;

fn main() {
    let suites = run_all_tests();
    print_test_summary(&suites);
}
```

---

## API Reference

### User Space Memory Layout

```rust
pub struct UserSpaceLayout {
    pub code_start: u64,
    pub code_end: u64,
    pub data_start: u64,
    pub data_end: u64,
    pub heap_start: u64,
    pub heap_end: u64,
    pub stack_start: u64,
    pub stack_end: u64,
    pub stack_size: u64,
}

impl UserSpaceLayout {
    pub fn new() -> Self;
    pub fn get_code_size(&self) -> u64;
    pub fn get_data_size(&self) -> u64;
    pub fn get_heap_size(&self) -> u64;
    pub fn get_stack_size(&self) -> u64;
    pub fn get_total_size(&self) -> u64;
}
```

### User Space Process

```rust
pub struct UserSpaceProcess {
    pub pid: u64,
    pub layout: UserSpaceLayout,
    pub entry_point: u64,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
    pub state: UserSpaceProcessState,
}

pub enum UserSpaceProcessState {
    Created,
    Loading,
    Ready,
    Running,
    Blocked,
    Terminated,
}

impl UserSpaceProcess {
    pub fn new(pid: u64, entry_point: u64) -> Self;
    pub fn set_argv(&mut self, argv: Vec<String>);
    pub fn set_envp(&mut self, envp: Vec<String>);
    pub fn set_state(&mut self, state: UserSpaceProcessState);
    pub fn get_stack_pointer(&self) -> u64;
    pub fn get_heap_pointer(&self) -> u64;
}
```

### User Space Loader

```rust
pub struct UserSpaceLoader {
    processes: BTreeMap<u64, UserSpaceProcess>,
    next_pid: AtomicU64,
}

impl UserSpaceLoader {
    pub fn new() -> Self;
    pub fn allocate_pid(&self) -> u64;
    pub fn create_process(&mut self, entry_point: u64) -> Result<u64, &'static str>;
    pub fn get_process(&self, pid: u64) -> Option<&UserSpaceProcess>;
    pub fn get_process_mut(&mut self, pid: u64) -> Option<&mut UserSpaceProcess>;
    pub fn load_executable(&mut self, pid: u64, data: &[u8]) -> Result<(), &'static str>;
    pub fn start_process(&mut self, pid: u64) -> Result<(), &'static str>;
    pub fn get_stats(&self) -> LoaderStats;
}
```

---

## Examples

### Example 1: Hello World

```c
#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    return 0;
}
```

### Example 2: String Operations

```c
#include <stdio.h>
#include <string.h>

int main() {
    char str1[20] = "Hello";
    char str2[] = "World";
    
    strcat(str1, ", ");
    strcat(str1, str2);
    
    printf("%s\n", str1);
    printf("Length: %zu\n", strlen(str1));
    
    return 0;
}
```

### Example 3: Math Operations

```c
#include <stdio.h>
#include <math.h>

int main() {
    double x = 3.14159;
    
    printf("sin(%.5f) = %.5f\n", x, sin(x));
    printf("cos(%.5f) = %.5f\n", x, cos(x));
    printf("sqrt(%.5f) = %.5f\n", x, sqrt(x));
    printf("pow(%.5f, 2) = %.5f\n", x, pow(x, 2));
    
    return 0;
}
```

### Example 4: Thread Creation

```c
#include <stdio.h>
#include <pthread.h>

void *thread_func(void *arg) {
    printf("Thread running\n");
    return NULL;
}

int main() {
    pthread_t thread;
    
    if (pthread_create(&thread, NULL, thread_func, NULL) != 0) {
        printf("Failed to create thread\n");
        return 1;
    }
    
    pthread_join(thread, NULL);
    printf("Thread finished\n");
    
    return 0;
}
```

---

## Troubleshooting

### Common Issues

#### Issue 1: Memory Allocation Failed

**Symptoms**: Program crashes with segmentation fault

**Solutions**:
- Check heap size
- Verify memory layout
- Use proper memory management

#### Issue 2: Thread Creation Failed

**Symptoms**: pthread_create returns error

**Solutions**:
- Check thread stack size
- Verify thread function signature
- Ensure proper thread cleanup

#### Issue 3: Symbol Resolution Failed

**Symptoms**: Dynamic linker cannot find symbol

**Solutions**:
- Verify ELF file format
- Check symbol table
- Ensure proper library linking

#### Issue 4: Command Parsing Failed

**Symptoms**: Shell cannot parse command

**Solutions**:
- Check command syntax
- Verify pipe and redirection syntax
- Ensure proper quoting

---

## Performance Characteristics

### Library Performance

| Library | Operation | Performance |
|---------|-----------|-------------|
| libc | strlen | O(n) |
| libc | strcmp | O(n) |
| libc | memcpy | O(n) |
| libm | sin | O(1) (simplified) |
| libm | sqrt | O(1) |
| libpthread | pthread_create | O(1) |
| ldso | load_elf | O(n) |

### System Call Performance

| System Call | Latency |
|-------------|---------|
| getpid | < 1μs |
| read/write | < 10μs |
| open/close | < 5μs |
| fork | < 100μs |
| exec | < 1ms |

---

## Security Considerations

### Memory Safety

- All user space code is written in Rust
- No buffer overflows
- No use-after-free
- No null pointer dereferences

### Process Isolation

- Each process has isolated memory space
- System call validation
- Capability-based security

### Thread Safety

- Mutex protection for shared data
- Condition variables for synchronization
- Atomic operations for simple cases

---

## Future Enhancements

### Planned Features

1. **Additional libc Functions**: More POSIX-compliant functions
2. **Advanced libm Functions**: More accurate math algorithms
3. **Extended libpthread**: More synchronization primitives
4. **Enhanced ldso**: Support for more relocation types
5. **More Shell Commands**: Additional built-in commands
6. **More Utilities**: Additional file and network utilities

### Performance Improvements

1. **Optimized String Functions**: SIMD optimizations
2. **Faster Math Functions**: Hardware acceleration
3. **Efficient Memory Management**: Custom allocators
4. **Improved System Calls**: Faster system call path

---

## References

- POSIX.1-2017: IEEE Standard for Information Technology
- ELF Specification: Tool Interface Standard
- C11 Standard: ISO/IEC 9899:2011
- POSIX Threads: IEEE Std 1003.1c-2017

---

## Support

For issues, questions, or contributions, please visit:
- GitHub: https://github.com/vantisCorp/VantisOS
- Documentation: https://docs.vantisos.ai
- Community: https://community.vantisos.ai

---

**Document Version**: 1.0  
**Last Updated**: February 28, 2025  
**Maintainer**: VantisOS Team