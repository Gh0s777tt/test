# VantisOS v1.4.0 - API Reference

## System Call API

### Process Management

#### exit
```c
void exit(int exit_code);
```
Terminates the current process.

#### fork
```c
pid_t fork(void);
```
Creates a new process.

#### exec
```c
int exec(const char *pathname, char *const argv[]);
```
Executes a new program.

#### wait
```c
pid_t wait(int *wstatus);
```
Waits for process termination.

#### getpid
```c
pid_t getpid(void);
```
Returns the current process ID.

### File Operations

#### open
```c
int open(const char *pathname, int flags, mode_t mode);
```
Opens a file.

#### close
```c
int close(int fd);
```
Closes a file descriptor.

#### read
```c
ssize_t read(int fd, void *buf, size_t count);
```
Reads from a file descriptor.

#### write
```c
ssize_t write(int fd, const void *buf, size_t count);
```
Writes to a file descriptor.

#### stat
```c
int stat(const char *pathname, struct stat *statbuf);
```
Gets file status.

### Memory Management

#### mmap
```c
void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);
```
Maps files or devices into memory.

#### munmap
```c
int munmap(void *addr, size_t length);
```
Unmaps a memory region.

#### brk
```c
int brk(void *addr);
```
Changes data segment size.

#### mprotect
```c
int mprotect(void *addr, size_t len, int prot);
```
Changes memory protection.

### I/O Control

#### ioctl
```c
int ioctl(int fd, unsigned long request, ...);
```
Device-specific operations.

### Network Operations

#### socket
```c
int socket(int domain, int type, int protocol);
```
Creates a socket.

#### bind
```c
int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
```
Binds a socket to an address.

#### listen
```c
int listen(int sockfd, int backlog);
```
Listens for connections.

#### accept
```c
int accept(int sockfd, struct sockaddr *addr, socklen_t *addrlen);
```
Accepts a connection.

#### connect
```c
int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
```
Connects to a socket.

#### send
```c
ssize_t send(int sockfd, const void *buf, size_t len, int flags);
```
Sends data on a socket.

#### recv
```c
ssize_t recv(int sockfd, void *buf, size_t len, int flags);
```
Receives data from a socket.

## Process Management API

### Process Creation
```rust
pub fn create_process(ppid: u64) -> Option<u64>
```
Creates a new process with the specified parent PID.

### Process Termination
```rust
pub fn terminate_process(pid: u64, exit_code: i32) -> bool
```
Terminates a process with the specified PID.

### Process State
```rust
pub fn set_process_state(pid: u64, state: ProcessState) -> bool
```
Sets the state of a process.

### Process Priority
```rust
pub fn set_process_priority(pid: u64, priority: ProcessPriority) -> bool
```
Sets the priority of a process.

### Process Statistics
```rust
pub fn get_process_stats() -> ProcessStats
```
Returns process statistics.

## Thread Management API

### Thread Creation
```rust
pub fn create_thread(pid: u64) -> Option<u64>
```
Creates a new thread in the specified process.

### Thread Termination
```rust
pub fn terminate_thread(tid: u64) -> bool
```
Terminates a thread with the specified TID.

### Thread Scheduling
```rust
pub fn schedule() -> Option<u64>
```
Schedules the next thread to run.

### Thread Yield
```rust
pub fn yield_thread()
```
Yields the current thread.

### Thread Statistics
```rust
pub fn get_thread_stats() -> ThreadStats
```
Returns thread statistics.

## File System API

### File Operations
```rust
pub fn open(path: &str, flags: u32, mode: u32) -> Option<u64>
pub fn close(fd: u64) -> bool
pub fn read(fd: u64, buffer: &mut [u8]) -> isize
pub fn write(fd: u64, buffer: &[u8]) -> isize
pub fn seek(fd: u64, offset: i64, whence: u32) -> i64
```

### File Statistics
```rust
pub fn stat(path: &str) -> Option<FileStats>
pub fn fstat(fd: u64) -> Option<FileStats>
```

### File System Statistics
```rust
pub fn get_fd_count() -> u64
```

## Memory Management API

### Page Allocation
```rust
pub fn allocate_page() -> Option<usize>
pub fn free_page(addr: usize)
```

### Heap Allocation
```rust
pub fn allocate_heap(size: usize, align: usize) -> Option<usize>
pub fn free_heap(addr: usize)
```

### Memory Statistics
```rust
pub fn get_stats() -> MemoryStats
```

## Interrupt Handling API

### Interrupt Initialization
```rust
pub fn init_idt()
pub fn load_idt()
```

### Interrupt Control
```rust
pub fn enable_interrupts()
pub fn disable_interrupts()
pub fn is_interrupts_enabled() -> bool
```

### Interrupt Statistics
```rust
pub fn get_interrupt_stats() -> InterruptStats
```

## Security API

### Stack Canaries
```rust
pub fn get_stack_canary() -> u64
pub fn verify_stack_canary(canary: u64) -> bool
```

### Memory Protection
```rust
pub fn enable_memory_protection()
pub fn disable_memory_protection()
pub fn is_memory_protection_enabled() -> bool
```

### Security Checks
```rust
pub fn security_check(condition: bool, message: &str)
pub fn check_memory_access(address: u64, size: u64, write: bool) -> bool
pub fn validate_pointer<T>(ptr: *const T) -> bool
```

### Security Statistics
```rust
pub fn get_security_stats() -> SecurityStats
```

## Performance API

### Performance Counters
```rust
pub fn get_counters() -> PerformanceCounters
pub fn reset_counters()
```

### Performance Recording
```rust
pub fn record_boot_time(time: u64)
pub fn record_memory_allocation(time: u64)
pub fn record_interrupt(time: u64)
pub fn record_console_output(chars: u64, time: u64)
```

### Performance Measurement
```rust
pub fn rdtsc() -> u64
pub fn cycles_to_us(cycles: u64) -> u64
pub fn cycles_to_ms(cycles: u64) -> u64
```

## Console API

### Console Initialization
```rust
pub fn init()
```

### Console Output
```rust
pub fn write_byte(byte: u8)
pub fn write_string(s: &str)
pub fn write_dec(value: u64)
pub fn write_hex(value: u64)
pub fn write_hex32(value: u32)
pub fn write_bool(value: bool)
```

### Console Control
```rust
pub fn set_color(foreground: VgaColor, background: VgaColor)
pub fn clear_screen()
```

---

**Last Updated**: March 1, 2025
**Version**: 0.5.0