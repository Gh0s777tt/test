//! End-to-End Integration Tests for Verified IPC System
//! 
//! These tests verify that all three properties work together correctly
//! in realistic scenarios.

use vantis_os::verified::ipc_integrated::{
    IntegratedIpcManager, VerifiedMessage, MAX_MESSAGE_SIZE, MAX_QUEUE_SIZE, MAX_IPC_MEMORY,
};
use vantis_os::verified::process::Pid;

// ============================================================================
// BASIC FUNCTIONALITY TESTS
// ============================================================================

#[test]
fn test_basic_send_receive() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    // Setup
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Send
    let data = vec![1, 2, 3, 4, 5];
    let msg_id = manager.send(sender, receiver, data.clone()).unwrap();
    assert!(msg_id > 0);
    
    // Receive
    let msg = manager.receive(receiver).unwrap();
    assert_eq!(msg.read_data(receiver).unwrap(), data.as_slice());
}

#[test]
fn test_multiple_messages() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Send multiple messages
    for i in 0..10 {
        let data = vec![i; 10];
        manager.send(sender, receiver, data).unwrap();
    }
    
    // Receive all messages
    for i in 0..10 {
        let msg = manager.receive(receiver).unwrap();
        assert_eq!(msg.read_data(receiver).unwrap()[0], i);
    }
    
    // Queue should be empty
    assert!(manager.receive(receiver).is_none());
}

#[test]
fn test_bidirectional_communication() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let p1 = Pid::new(1);
    let p2 = Pid::new(2);
    
    manager.register_process(p1);
    manager.register_process(p2);
    manager.grant_send_capability(p1, p2);
    manager.grant_send_capability(p2, p1);
    
    // P1 -> P2
    manager.send(p1, p2, vec![1, 2, 3]).unwrap();
    let msg = manager.receive(p2).unwrap();
    assert_eq!(msg.read_data(p2).unwrap(), &[1, 2, 3]);
    
    // P2 -> P1
    manager.send(p2, p1, vec![4, 5, 6]).unwrap();
    let msg = manager.receive(p1).unwrap();
    assert_eq!(msg.read_data(p1).unwrap(), &[4, 5, 6]);
}

// ============================================================================
// PROPERTY 1: MESSAGE INTEGRITY TESTS
// ============================================================================

#[test]
fn test_integrity_preserved_through_system() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Send message
    let data = vec![1, 2, 3, 4, 5];
    manager.send(sender, receiver, data.clone()).unwrap();
    
    // Receive and verify integrity
    let msg = manager.receive(receiver).unwrap();
    assert!(msg.verify_integrity());
    assert_eq!(msg.read_data(receiver).unwrap(), data.as_slice());
}

#[test]
fn test_integrity_with_large_messages() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Send large message (4KB)
    let data = vec![42u8; MAX_MESSAGE_SIZE];
    manager.send(sender, receiver, data.clone()).unwrap();
    
    // Receive and verify
    let msg = manager.receive(receiver).unwrap();
    assert!(msg.verify_integrity());
    assert_eq!(msg.read_data(receiver).unwrap(), data.as_slice());
}

// ============================================================================
// PROPERTY 2: RESOURCE BOUNDS TESTS
// ============================================================================

#[test]
fn test_message_size_limit() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Try to send oversized message
    let data = vec![0u8; MAX_MESSAGE_SIZE + 1];
    assert!(manager.send(sender, receiver, data).is_err());
    
    // Normal size should work
    let data = vec![0u8; MAX_MESSAGE_SIZE];
    assert!(manager.send(sender, receiver, data).is_ok());
}

#[test]
fn test_queue_size_limit() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Fill queue to limit
    for i in 0..MAX_QUEUE_SIZE {
        let result = manager.send(sender, receiver, vec![i as u8]);
        assert!(result.is_ok(), "Failed at message {}", i);
    }
    
    // Next message should fail (queue full)
    assert!(manager.send(sender, receiver, vec![99]).is_err());
    
    // After receiving one, should be able to send again
    manager.receive(receiver).unwrap();
    assert!(manager.send(sender, receiver, vec![99]).is_ok());
}

#[test]
fn test_total_memory_limit() {
    let mut manager = IntegratedIpcManager::new(1000); // 1KB limit
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Send messages until memory limit
    let mut sent = 0;
    for i in 0..200 {
        let data = vec![i as u8; 10]; // 10 bytes each
        if manager.send(sender, receiver, data).is_ok() {
            sent += 1;
        } else {
            break;
        }
    }
    
    // Should have sent ~100 messages (1000 bytes / 10 bytes)
    assert!(sent >= 90 && sent <= 100, "Sent {} messages", sent);
    
    // Stats should reflect memory usage
    let stats = manager.get_stats();
    assert!(stats.total_memory <= 1000);
}

#[test]
fn test_memory_reclamation() {
    let mut manager = IntegratedIpcManager::new(1000);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Send messages
    for _ in 0..10 {
        manager.send(sender, receiver, vec![0u8; 50]).unwrap();
    }
    
    let stats = manager.get_stats();
    assert_eq!(stats.total_memory, 500);
    
    // Receive all messages
    for _ in 0..10 {
        manager.receive(receiver).unwrap();
    }
    
    // Memory should be reclaimed
    let stats = manager.get_stats();
    assert_eq!(stats.total_memory, 0);
}

// ============================================================================
// PROPERTY 3: INFORMATION LEAKAGE PREVENTION TESTS
// ============================================================================

#[test]
fn test_process_isolation() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    let attacker = Pid::new(3);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.register_process(attacker);
    manager.grant_send_capability(sender, receiver);
    
    // Send message to receiver
    manager.send(sender, receiver, vec![1, 2, 3]).unwrap();
    
    // Attacker cannot receive it
    assert!(manager.receive(attacker).is_none());
    
    // Receiver can receive it
    let msg = manager.receive(receiver).unwrap();
    assert_eq!(msg.read_data(receiver).unwrap(), &[1, 2, 3]);
}

#[test]
fn test_capability_enforcement() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    
    // Without capability, send should fail
    assert!(manager.send(sender, receiver, vec![1, 2, 3]).is_err());
    
    // Grant capability
    manager.grant_send_capability(sender, receiver);
    
    // Now should succeed
    assert!(manager.send(sender, receiver, vec![1, 2, 3]).is_ok());
}

#[test]
fn test_unauthorized_read_attempt() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    let attacker = Pid::new(3);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.register_process(attacker);
    manager.grant_send_capability(sender, receiver);
    
    // Send message
    manager.send(sender, receiver, vec![1, 2, 3]).unwrap();
    
    // Get message (but don't consume it yet)
    let msg = manager.receive(receiver).unwrap();
    
    // Attacker tries to read message data
    assert!(msg.read_data(attacker).is_err());
    
    // Receiver can read it
    assert!(msg.read_data(receiver).is_ok());
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[test]
fn test_high_volume_messaging() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Send 1000 messages
    for i in 0..1000 {
        let data = vec![(i % 256) as u8; 100];
        manager.send(sender, receiver, data).unwrap();
        
        // Receive immediately to avoid queue overflow
        let msg = manager.receive(receiver).unwrap();
        assert!(msg.verify_integrity());
    }
}

#[test]
fn test_many_processes() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    // Create 100 processes
    for i in 1..=100 {
        manager.register_process(Pid::new(i));
    }
    
    // Grant capabilities in a ring
    for i in 1..=100 {
        let next = if i == 100 { 1 } else { i + 1 };
        manager.grant_send_capability(Pid::new(i), Pid::new(next));
    }
    
    // Send messages around the ring
    for i in 1..=100 {
        let next = if i == 100 { 1 } else { i + 1 };
        manager.send(Pid::new(i), Pid::new(next), vec![i as u8]).unwrap();
    }
    
    // Each process should have one message
    for i in 1..=100 {
        let msg = manager.receive(Pid::new(i)).unwrap();
        let prev = if i == 1 { 100 } else { i - 1 };
        assert_eq!(msg.read_data(Pid::new(i)).unwrap()[0], prev as u8);
    }
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_unregistered_process() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    // Try to send without registering
    assert!(manager.send(sender, receiver, vec![1, 2, 3]).is_err());
}

#[test]
fn test_receive_from_empty_queue() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let receiver = Pid::new(1);
    manager.register_process(receiver);
    
    // Receive from empty queue
    assert!(manager.receive(receiver).is_none());
}

#[test]
fn test_stats_accuracy() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let sender = Pid::new(1);
    let receiver = Pid::new(2);
    
    manager.register_process(sender);
    manager.register_process(receiver);
    manager.grant_send_capability(sender, receiver);
    
    // Initial stats
    let stats = manager.get_stats();
    assert_eq!(stats.total_memory, 0);
    assert_eq!(stats.num_queues, 2);
    
    // Send message
    manager.send(sender, receiver, vec![1, 2, 3, 4, 5]).unwrap();
    
    let stats = manager.get_stats();
    assert_eq!(stats.total_memory, 5);
    
    // Receive message
    manager.receive(receiver).unwrap();
    
    let stats = manager.get_stats();
    assert_eq!(stats.total_memory, 0);
}

// ============================================================================
// REAL-WORLD SCENARIO TESTS
// ============================================================================

#[test]
fn test_client_server_pattern() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let client = Pid::new(1);
    let server = Pid::new(2);
    
    manager.register_process(client);
    manager.register_process(server);
    manager.grant_send_capability(client, server);
    manager.grant_send_capability(server, client);
    
    // Client sends request
    manager.send(client, server, b"GET /data".to_vec()).unwrap();
    
    // Server receives request
    let request = manager.receive(server).unwrap();
    assert_eq!(request.read_data(server).unwrap(), b"GET /data");
    
    // Server sends response
    manager.send(server, client, b"200 OK".to_vec()).unwrap();
    
    // Client receives response
    let response = manager.receive(client).unwrap();
    assert_eq!(response.read_data(client).unwrap(), b"200 OK");
}

#[test]
fn test_producer_consumer_pattern() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let producer = Pid::new(1);
    let consumer = Pid::new(2);
    
    manager.register_process(producer);
    manager.register_process(consumer);
    manager.grant_send_capability(producer, consumer);
    
    // Producer sends work items
    for i in 0..10 {
        let work = vec![i; 10];
        manager.send(producer, consumer, work).unwrap();
    }
    
    // Consumer processes work items
    for i in 0..10 {
        let work = manager.receive(consumer).unwrap();
        assert_eq!(work.read_data(consumer).unwrap()[0], i);
    }
}

#[test]
fn test_broadcast_pattern() {
    let mut manager = IntegratedIpcManager::new(MAX_IPC_MEMORY);
    
    let broadcaster = Pid::new(1);
    let listeners = vec![Pid::new(2), Pid::new(3), Pid::new(4)];
    
    manager.register_process(broadcaster);
    for &listener in &listeners {
        manager.register_process(listener);
        manager.grant_send_capability(broadcaster, listener);
    }
    
    // Broadcast message to all listeners
    let message = b"BROADCAST".to_vec();
    for &listener in &listeners {
        manager.send(broadcaster, listener, message.clone()).unwrap();
    }
    
    // All listeners receive the message
    for &listener in &listeners {
        let msg = manager.receive(listener).unwrap();
        assert_eq!(msg.read_data(listener).unwrap(), b"BROADCAST");
    }
}