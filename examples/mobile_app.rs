//! Mobile Application Example
//! 
//! This example demonstrates how to create a simple mobile application
//! using VantisOS mobile framework with touch gestures.

use vantis::verified::mobile::ui::MobileUI;
use vantis::verified::mobile::touch::{GestureType, TouchEvent};
use vantis::verified::mobile::battery::BatteryManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("VantisOS Mobile Application Example");
    println!("====================================\n");
    
    // Initialize mobile UI
    let mut ui = MobileUI::new("VantisOS Demo App")?;
    
    // Setup main screen
    ui.create_screen("main")?;
    ui.add_label("Welcome to VantisOS!")?;
    ui.add_button("Start", 100, 200)?;
    ui.add_button("Settings", 100, 300)?;
    
    // Setup settings screen
    ui.create_screen("settings")?;
    ui.add_label("Settings")?;
    ui.add_toggle("Dark Mode", 100, 200)?;
    ui.add_slider("Brightness", 100, 300, 0, 100, 80)?;
    ui.add_button("Back", 100, 400)?;
    
    // Initialize battery manager
    let battery = BatteryManager::new()?;
    let level = battery.get_level()?;
    println!("Battery level: {}%", level);
    
    // Set battery optimization
    battery.enable_power_saving()?;
    println!("Power saving mode enabled");
    
    // Set up gesture handlers
    ui.register_gesture(GestureType::Tap, |event| {
        println!("Tap detected at ({}, {})", event.x, event.y);
        handle_tap(event)
    })?;
    
    ui.register_gesture(GestureType::Swipe, |event| {
        println!("Swipe detected: {:?}", event.direction);
        handle_swipe(event)
    })?;
    
    // Start UI event loop
    println!("Starting mobile application...\n");
    ui.run_event_loop()?;
    
    Ok(())
}

fn handle_tap(event: TouchEvent) -> Result<(), Box<dyn std::error::Error>> {
    // Handle button taps based on coordinates
    if event.y >= 200 && event.y <= 250 {
        println!("Start button tapped!");
        // Start application logic here
    } else if event.y >= 300 && event.y <= 350 {
        println!("Settings button tapped!");
        // Navigate to settings
    } else if event.y >= 400 && event.y <= 450 {
        println!("Back button tapped!");
        // Navigate back
    }
    Ok(())
}

fn handle_swipe(event: TouchEvent) -> Result<(), Box<dyn std::error::Error>> {
    match event.direction {
        Some(direction) => {
            println!("Swipe direction: {:?}", direction);
            // Handle swipe navigation
            Ok(())
        }
        None => {
            println!("Invalid swipe");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mobile_ui_creation() {
        let ui = MobileUI::new("Test App").unwrap();
        assert_eq!(ui.get_title(), "Test App");
    }
    
    #[test]
    fn test_battery_level() {
        let battery = BatteryManager::new().unwrap();
        let level = battery.get_level().unwrap();
        assert!(level >= 0 && level <= 100);
    }
}