// Input Event System - VantisOS
//
// This module implements a unified input event system
// for all input devices.

use alloc::vec::Vec;
use alloc::collections::VecDeque;
use spin::Mutex;

/// Input event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InputEventType {
    Key = 0,
    Mouse = 1,
    Touch = 2,
    Joystick = 3,
}

/// Input event
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputEventType,
    pub device_id: u32,
    pub timestamp: u64,
    pub data: InputEventData,
}

/// Input event data
#[derive(Debug, Clone)]
pub enum InputEventData {
    Key {
        scancode: u8,
        pressed: bool,
    },
    Mouse {
        x: i16,
        y: i16,
        left_button: bool,
        right_button: bool,
        middle_button: bool,
    },
    Touch {
        x: u16,
        y: u16,
        pressure: u8,
        id: u8,
        event_type: super::touchscreen::TouchEventType,
    },
    Joystick {
        axis_x: i16,
        axis_y: i16,
        buttons: u16,
    },
}

impl InputEvent {
    /// Create a new input event
    pub fn new(event_type: InputEventType, device_id: u32, timestamp: u64, data: InputEventData) -> Self {
        Self {
            event_type,
            device_id,
            timestamp,
            data,
        }
    }
}

/// Input device
#[derive(Debug, Clone)]
pub struct InputDevice {
    pub id: u32,
    pub name: String,
    pub device_type: InputEventType,
    pub is_enabled: bool,
}

impl InputDevice {
    /// Create a new input device
    pub fn new(id: u32, name: String, device_type: InputEventType) -> Self {
        Self {
            id,
            name,
            device_type,
            is_enabled: false,
        }
    }
}

/// Input event queue
pub struct InputEventQueue {
    events: Mutex<VecDeque<InputEvent>>,
    max_size: usize,
}

impl InputEventQueue {
    /// Create a new input event queue
    pub fn new(max_size: usize) -> Self {
        Self {
            events: Mutex::new(VecDeque::with_capacity(max_size)),
            max_size,
        }
    }
    
    /// Push an event to the queue
    pub fn push(&self, event: InputEvent) -> Result<(), ()> {
        let mut events = self.events.lock();
        
        if events.len() >= self.max_size {
            events.pop_front();
        }
        
        events.push_back(event);
        Ok(())
    }
    
    /// Pop an event from the queue
    pub fn pop(&self) -> Option<InputEvent> {
        let mut events = self.events.lock();
        events.pop_front()
    }
    
    /// Peek at the next event
    pub fn peek(&self) -> Option<InputEvent> {
        let events = self.events.lock();
        events.front().cloned()
    }
    
    /// Get queue size
    pub fn len(&self) -> usize {
        self.events.lock().len()
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.lock().is_empty()
    }
    
    /// Clear the queue
    pub fn clear(&self) {
        self.events.lock().clear();
    }
}

/// Input event manager
pub struct InputEventManager {
    devices: Vec<InputDevice>,
    event_queue: InputEventQueue,
    next_device_id: u32,
}

impl InputEventManager {
    /// Create a new input event manager
    pub fn new(queue_size: usize) -> Self {
        Self {
            devices: Vec::new(),
            event_queue: InputEventQueue::new(queue_size),
            next_device_id: 1,
        }
    }
    
    /// Register a device
    pub fn register_device(&mut self, name: String, device_type: InputEventType) -> u32 {
        let device_id = self.next_device_id;
        self.next_device_id += 1;
        
        let device = InputDevice::new(device_id, name, device_type);
        self.devices.push(device);
        
        device_id
    }
    
    /// Unregister a device
    pub fn unregister_device(&mut self, device_id: u32) -> Result<(), ()> {
        self.devices.retain(|d| d.id != device_id);
        Ok(())
    }
    
    /// Get device by ID
    pub fn get_device(&self, device_id: u32) -> Option<&InputDevice> {
        self.devices.iter().find(|d| d.id == device_id)
    }
    
    /// Get all devices
    pub fn get_devices(&self) -> &[InputDevice] {
        &self.devices
    }
    
    /// Get event queue
    pub fn get_event_queue(&self) -> &InputEventQueue {
        &self.event_queue
    }
    
    /// Push an event
    pub fn push_event(&self, event: InputEvent) -> Result<(), ()> {
        self.event_queue.push(event)
    }
    
    /// Pop an event
    pub fn pop_event(&self) -> Option<InputEvent> {
        self.event_queue.pop()
    }
}

/// Initialize input event system
pub fn init() {
    // TODO: Initialize input event system
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_input_event_creation() {
        let data = InputEventData::Key {
            scancode: 0x01,
            pressed: true,
        };
        
        let event = InputEvent::new(InputEventType::Key, 1, 1000, data);
        
        assert_eq!(event.event_type, InputEventType::Key);
        assert_eq!(event.device_id, 1);
        assert_eq!(event.timestamp, 1000);
    }
    
    #[test]
    fn test_input_device_creation() {
        let device = InputDevice::new(1, "Keyboard".to_string(), InputEventType::Key);
        
        assert_eq!(device.id, 1);
        assert_eq!(device.name, "Keyboard");
        assert_eq!(device.device_type, InputEventType::Key);
    }
    
    #[test]
    fn test_input_event_queue() {
        let queue = InputEventQueue::new(100);
        
        let data = InputEventData::Key {
            scancode: 0x01,
            pressed: true,
        };
        
        let event = InputEvent::new(InputEventType::Key, 1, 1000, data);
        
        assert!(queue.push(event).is_ok());
        assert_eq!(queue.len(), 1);
        
        let popped = queue.pop();
        assert!(popped.is_some());
    }
    
    #[test]
    fn test_input_event_manager() {
        let mut manager = InputEventManager::new(100);
        
        let device_id = manager.register_device("Keyboard".to_string(), InputEventType::Key);
        
        assert!(manager.get_device(device_id).is_some());
        assert_eq!(manager.get_devices().len(), 1);
    }
}