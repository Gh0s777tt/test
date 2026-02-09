//! # Comprehensive End-to-End Tests for Complete IPC System
//!
//! This test suite validates all 5 verified properties working together:
//! 1. Message Integrity
//! 2. Resource Bounds
//! 3. Information Leakage Prevention
//! 4. Deadlock Freedom
//! 5. Capability Correctness
//!
//! ## Test Categories
//! - Basic functionality tests
//! - Edge case tests
//! - Failure scenario tests
//! - Concurrent operation tests
//! - Resource exhaustion tests
//! - Security tests
//! - Performance tests

#![cfg(test)]

use super::ipc_complete::*;
use std::thread;
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// BASIC FUNCTIONALITY TESTS
// ============================================================================

#[test]
fn test_basic_send_receive() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    let msg = CompleteMessage::new(b"Hello, World!", 1, 2, cap_id, token).unwrap();
    assert!(ipc.send(msg).is_ok());
    
    let received = ipc.receive(2).unwrap();
    assert_eq!(received.data(), b"Hello, World!");
    assert_eq!(received.sender(), 1);
    assert_eq!(received.receiver(), 2);
}

#[test]
fn test_multiple_messages() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Send 10 messages
    for i in 0..10 {
        let data = format!("Message {}", i);
        let msg = CompleteMessage::new(data.as_bytes(), 1, 2, cap_id, token).unwrap();
        assert!(ipc.send(msg).is_ok());
    }
    
    // Receive all 10 messages
    for i in 0..10 {
        let received = ipc.receive(2).unwrap();
        let expected = format!("Message {}", i);
        assert_eq!(received.data(), expected.as_bytes());
    }
}

#[test]
fn test_multiple_processes() {
    let ipc = IpcSystem::new();
    
    // Create capabilities for 3 processes
    let (cap_12, token_12) = ipc.create_capability(1, 2).unwrap();
    let (cap_23, token_23) = ipc.create_capability(2, 3).unwrap();
    let (cap_31, token_31) = ipc.create_capability(3, 1).unwrap();
    
    // Send messages in a ring
    let msg1 = CompleteMessage::new(b"1->2", 1, 2, cap_12, token_12).unwrap();
    let msg2 = CompleteMessage::new(b"2->3", 2, 3, cap_23, token_23).unwrap();
    let msg3 = CompleteMessage::new(b"3->1", 3, 1, cap_31, token_31).unwrap();
    
    assert!(ipc.send(msg1).is_ok());
    assert!(ipc.send(msg2).is_ok());
    assert!(ipc.send(msg3).is_ok());
    
    // Receive in order
    assert_eq!(ipc.receive(2).unwrap().data(), b"1->2");
    assert_eq!(ipc.receive(3).unwrap().data(), b"2->3");
    assert_eq!(ipc.receive(1).unwrap().data(), b"3->1");
}

// ============================================================================
// MESSAGE INTEGRITY TESTS
// ============================================================================

#[test]
fn test_integrity_preserved() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    let original_data = b"Important data that must not be corrupted";
    let msg = CompleteMessage::new(original_data, 1, 2, cap_id, token).unwrap();
    
    // Verify integrity before sending
    assert!(msg.verify_integrity().is_ok());
    
    ipc.send(msg).unwrap();
    let received = ipc.receive(2).unwrap();
    
    // Verify integrity after receiving
    assert!(received.verify_integrity().is_ok());
    assert_eq!(received.data(), original_data);
}

#[test]
fn test_large_message_integrity() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Create 4KB message (maximum size)
    let data = vec![0xAB; MAX_MESSAGE_SIZE];
    let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
    
    assert!(msg.verify_integrity().is_ok());
    ipc.send(msg).unwrap();
    
    let received = ipc.receive(2).unwrap();
    assert!(received.verify_integrity().is_ok());
    assert_eq!(received.data(), &data[..]);
}

// ============================================================================
// RESOURCE BOUNDS TESTS
// ============================================================================

#[test]
fn test_message_size_limit() {
    let data = vec![0u8; MAX_MESSAGE_SIZE + 1];
    let result = CompleteMessage::new(&data, 1, 2, 100, 0xDEADBEEF);
    
    assert_eq!(result, Err(IpcError::MessageTooLarge));
}

#[test]
fn test_queue_size_limit() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Fill queue to maximum
    for i in 0..MAX_QUEUE_SIZE {
        let data = format!("Message {}", i);
        let msg = CompleteMessage::new(data.as_bytes(), 1, 2, cap_id, token).unwrap();
        assert!(ipc.send(msg).is_ok(), "Failed to send message {}", i);
    }
    
    // Next message should fail
    let msg = CompleteMessage::new(b"Overflow", 1, 2, cap_id, token).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::QueueFull));
    
    // After receiving one, should be able to send again
    ipc.receive(2).unwrap();
    let msg = CompleteMessage::new(b"After receive", 1, 2, cap_id, token).unwrap();
    assert!(ipc.send(msg).is_ok());
}

#[test]
fn test_memory_tracking() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    let initial_memory = ipc.memory_usage();
    
    // Send message
    let data = vec![0u8; 1000];
    let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
    ipc.send(msg).unwrap();
    
    // Memory should increase
    assert!(ipc.memory_usage() > initial_memory);
    
    // Receive message
    ipc.receive(2).unwrap();
    
    // Memory should return to initial
    assert_eq!(ipc.memory_usage(), initial_memory);
}

// ============================================================================
// INFORMATION LEAKAGE PREVENTION TESTS
// ============================================================================

#[test]
fn test_capability_required() {
    let ipc = IpcSystem::new();
    
    // Try to send without valid capability
    let msg = CompleteMessage::new(b"Test", 1, 2, 999, 0xBADTOKEN).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::InvalidCapability));
}

#[test]
fn test_wrong_token() {
    let ipc = IpcSystem::new();
    let (cap_id, _) = ipc.create_capability(1, 2).unwrap();
    
    // Use wrong token
    let msg = CompleteMessage::new(b"Test", 1, 2, cap_id, 0xWRONGTOKEN).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::InvalidToken));
}

#[test]
fn test_wrong_sender() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Try to send from wrong process
    let msg = CompleteMessage::new(b"Test", 999, 2, cap_id, token).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::AccessDenied));
}

#[test]
fn test_wrong_receiver() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Try to send to wrong process
    let msg = CompleteMessage::new(b"Test", 1, 999, cap_id, token).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::AccessDenied));
}

#[test]
fn test_capability_revocation() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Send should work initially
    let msg = CompleteMessage::new(b"Before revoke", 1, 2, cap_id, token).unwrap();
    assert!(ipc.send(msg).is_ok());
    
    // Revoke capability
    ipc.revoke_capability(cap_id).unwrap();
    
    // Send should fail after revocation
    let msg = CompleteMessage::new(b"After revoke", 1, 2, cap_id, token).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::CapabilityRevoked));
}

// ============================================================================
// DEADLOCK FREEDOM TESTS
// ============================================================================

#[test]
fn test_simple_deadlock_prevention() {
    let ipc = IpcSystem::new();
    let (cap_12, token_12) = ipc.create_capability(1, 2).unwrap();
    let (cap_21, token_21) = ipc.create_capability(2, 1).unwrap();
    
    // Send from 1 to 2
    let msg1 = CompleteMessage::new(b"1->2", 1, 2, cap_12, token_12).unwrap();
    assert!(ipc.send(msg1).is_ok());
    
    // Try to send from 2 to 1 (would create cycle)
    let msg2 = CompleteMessage::new(b"2->1", 2, 1, cap_21, token_21).unwrap();
    assert_eq!(ipc.send(msg2), Err(IpcError::WouldDeadlock));
}

#[test]
fn test_three_way_deadlock_prevention() {
    let ipc = IpcSystem::new();
    let (cap_12, token_12) = ipc.create_capability(1, 2).unwrap();
    let (cap_23, token_23) = ipc.create_capability(2, 3).unwrap();
    let (cap_31, token_31) = ipc.create_capability(3, 1).unwrap();
    
    // Create chain: 1->2->3
    let msg1 = CompleteMessage::new(b"1->2", 1, 2, cap_12, token_12).unwrap();
    let msg2 = CompleteMessage::new(b"2->3", 2, 3, cap_23, token_23).unwrap();
    
    assert!(ipc.send(msg1).is_ok());
    assert!(ipc.send(msg2).is_ok());
    
    // Try to close the cycle: 3->1 (should fail)
    let msg3 = CompleteMessage::new(b"3->1", 3, 1, cap_31, token_31).unwrap();
    assert_eq!(ipc.send(msg3), Err(IpcError::WouldDeadlock));
}

#[test]
fn test_deadlock_resolution() {
    let ipc = IpcSystem::new();
    let (cap_12, token_12) = ipc.create_capability(1, 2).unwrap();
    let (cap_21, token_21) = ipc.create_capability(2, 1).unwrap();
    
    // Send from 1 to 2
    let msg1 = CompleteMessage::new(b"1->2", 1, 2, cap_12, token_12).unwrap();
    assert!(ipc.send(msg1).is_ok());
    
    // Receive message (breaks the wait)
    ipc.receive(2).unwrap();
    
    // Now 2->1 should work
    let msg2 = CompleteMessage::new(b"2->1", 2, 1, cap_21, token_21).unwrap();
    assert!(ipc.send(msg2).is_ok());
}

// ============================================================================
// CAPABILITY CORRECTNESS TESTS
// ============================================================================

#[test]
fn test_capability_uniqueness() {
    let ipc = IpcSystem::new();
    
    let (cap1, token1) = ipc.create_capability(1, 2).unwrap();
    let (cap2, token2) = ipc.create_capability(1, 2).unwrap();
    
    // Capabilities should be unique
    assert_ne!(cap1, cap2);
    assert_ne!(token1, token2);
}

#[test]
fn test_capability_isolation() {
    let ipc = IpcSystem::new();
    
    let (cap1, token1) = ipc.create_capability(1, 2).unwrap();
    let (cap2, token2) = ipc.create_capability(3, 4).unwrap();
    
    // Can't use cap1's token with cap2
    let msg = CompleteMessage::new(b"Test", 3, 4, cap2, token1).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::InvalidToken));
    
    // Can't use cap2's token with cap1
    let msg = CompleteMessage::new(b"Test", 1, 2, cap1, token2).unwrap();
    assert_eq!(ipc.send(msg), Err(IpcError::InvalidToken));
}

// ============================================================================
// CONCURRENT OPERATION TESTS
// ============================================================================

#[test]
fn test_concurrent_sends() {
    let ipc = Arc::new(IpcSystem::new());
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    let mut handles = vec![];
    
    // Spawn 10 threads sending messages
    for i in 0..10 {
        let ipc_clone = Arc::clone(&ipc);
        let handle = thread::spawn(move || {
            let data = format!("Thread {}", i);
            let msg = CompleteMessage::new(data.as_bytes(), 1, 2, cap_id, token).unwrap();
            ipc_clone.send(msg)
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        assert!(handle.join().unwrap().is_ok());
    }
    
    // Should have received 10 messages
    for _ in 0..10 {
        assert!(ipc.receive(2).is_ok());
    }
}

#[test]
fn test_concurrent_send_receive() {
    let ipc = Arc::new(IpcSystem::new());
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    let ipc_sender = Arc::clone(&ipc);
    let sender = thread::spawn(move || {
        for i in 0..100 {
            let data = format!("Message {}", i);
            let msg = CompleteMessage::new(data.as_bytes(), 1, 2, cap_id, token).unwrap();
            ipc_sender.send(msg).unwrap();
            thread::sleep(Duration::from_micros(100));
        }
    });
    
    let ipc_receiver = Arc::clone(&ipc);
    let receiver = thread::spawn(move || {
        for i in 0..100 {
            let received = ipc_receiver.receive(2).unwrap();
            let expected = format!("Message {}", i);
            assert_eq!(received.data(), expected.as_bytes());
        }
    });
    
    sender.join().unwrap();
    receiver.join().unwrap();
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_empty_message() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    let msg = CompleteMessage::new(b"", 1, 2, cap_id, token).unwrap();
    assert!(ipc.send(msg).is_ok());
    
    let received = ipc.receive(2).unwrap();
    assert_eq!(received.data(), b"");
}

#[test]
fn test_maximum_size_message() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    let data = vec![0xFF; MAX_MESSAGE_SIZE];
    let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
    
    assert!(ipc.send(msg).is_ok());
    
    let received = ipc.receive(2).unwrap();
    assert_eq!(received.data().len(), MAX_MESSAGE_SIZE);
}

#[test]
fn test_receive_timeout() {
    let ipc = IpcSystem::new();
    
    // Try to receive from empty queue (should timeout)
    let start = std::time::Instant::now();
    let result = ipc.receive(999);
    let elapsed = start.elapsed();
    
    assert_eq!(result, Err(IpcError::Timeout));
    assert!(elapsed >= MAX_WAIT_TIME);
}

// ============================================================================
// STATISTICS TESTS
// ============================================================================

#[test]
fn test_statistics_tracking() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Send 5 messages
    for i in 0..5 {
        let data = format!("Message {}", i);
        let msg = CompleteMessage::new(data.as_bytes(), 1, 2, cap_id, token).unwrap();
        ipc.send(msg).unwrap();
    }
    
    // Receive 3 messages
    for _ in 0..3 {
        ipc.receive(2).unwrap();
    }
    
    let stats = ipc.stats();
    assert_eq!(stats.messages_sent, 5);
    assert_eq!(stats.messages_received, 3);
}

#[test]
fn test_error_statistics() {
    let ipc = IpcSystem::new();
    
    // Try invalid operations
    let msg = CompleteMessage::new(b"Test", 1, 2, 999, 0xBAD).unwrap();
    let _ = ipc.send(msg);
    
    let stats = ipc.stats();
    assert!(stats.capability_failures > 0);
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[test]
fn test_high_throughput() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    // Send 1000 messages
    for i in 0..1000 {
        let data = format!("Message {}", i);
        let msg = CompleteMessage::new(data.as_bytes(), 1, 2, cap_id, token).unwrap();
        ipc.send(msg).unwrap();
    }
    
    // Receive all 1000
    for i in 0..1000 {
        let received = ipc.receive(2).unwrap();
        let expected = format!("Message {}", i);
        assert_eq!(received.data(), expected.as_bytes());
    }
}

#[test]
fn test_many_processes() {
    let ipc = IpcSystem::new();
    
    // Create capabilities for 100 processes
    let mut capabilities = vec![];
    for i in 0..100 {
        let (cap_id, token) = ipc.create_capability(i, i + 1).unwrap();
        capabilities.push((cap_id, token));
    }
    
    // Send message from each process
    for (i, (cap_id, token)) in capabilities.iter().enumerate() {
        let data = format!("From process {}", i);
        let msg = CompleteMessage::new(data.as_bytes(), i as u64, (i + 1) as u64, *cap_id, *token).unwrap();
        ipc.send(msg).unwrap();
    }
    
    // Receive all messages
    for i in 0..100 {
        let received = ipc.receive((i + 1) as u64).unwrap();
        let expected = format!("From process {}", i);
        assert_eq!(received.data(), expected.as_bytes());
    }
}

#[test]
fn test_queue_length_tracking() {
    let ipc = IpcSystem::new();
    let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
    
    assert_eq!(ipc.queue_length(2), 0);
    
    // Send 5 messages
    for i in 0..5 {
        let data = format!("Message {}", i);
        let msg = CompleteMessage::new(data.as_bytes(), 1, 2, cap_id, token).unwrap();
        ipc.send(msg).unwrap();
    }
    
    assert_eq!(ipc.queue_length(2), 5);
    
    // Receive 2 messages
    ipc.receive(2).unwrap();
    ipc.receive(2).unwrap();
    
    assert_eq!(ipc.queue_length(2), 3);
}