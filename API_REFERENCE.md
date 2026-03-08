# VantisOS API Reference

## System APIs

### Process Management

```rust
use vantis::process;

// Create new process
let pid = process::spawn("/path/to/binary")?;

// Wait for process
process::wait(pid)?;

// Get process info
let info = process::info(pid)?;
```

### Memory Management

```rust
use vantis::memory;

// Allocate memory
let ptr = memory::allocate(size)?;

// Free memory
memory::deallocate(ptr)?;

// Memory stats
let stats = memory::stats()?;
```

### File System

```rust
use vantis::fs;

// Read file
let content = fs::read("/path/to/file")?;

// Write file
fs::write("/path/to/file", &data)?;

// List directory
let entries = fs::read_dir("/path")?;
```

### Network

```rust
use vantis::net;

// Create socket
let socket = net::Socket::new(net::Domain::IPv4)?;

// Connect
socket.connect("127.0.0.1:8080")?;

// Send data
socket.send(&data)?;
```

## Security APIs

### Cryptography

```rust
use vantis::crypto;

// Hash
let hash = crypto::hash::sha256(data)?;

// Encrypt
let encrypted = crypto::encrypt(&key, &plaintext)?;

// Decrypt
let decrypted = crypto::decrypt(&key, &encrypted)?;
```

### Post-Quantum Cryptography

```rust
use vantis::pqcrypto;

// Kyber key exchange
let (pk, sk) = pqcrypto::kyber::keypair()?;
let (ct, ss) = pqcrypto::kyber::encapsulate(&pk)?;

// Dilithium signatures
let sig = pqcrypto::dilithium::sign(&message, &sk)?;
```

## IPC

```rust
use vantis::ipc;

// Create channel
let (tx, rx) = ipc::channel()?;

// Send message
tx.send(message)?;

// Receive message
let msg = rx.recv()?;
```

## Error Handling

All APIs return `Result<T, Error>` for proper error handling.

```rust
match api_call() {
    Ok(result) => process(result),
    Err(e) => handle_error(e),
}
```