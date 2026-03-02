//! Power Management Example
//! 
//! This example demonstrates how to use the power management system.

use vantisos::power::management::*;
use vantisos::power::frequency::*;
use vantisos::power::modes::*;
use vantisos::power::monitoring::*;

fn main() {
    // Initialize power management
    vantisos::power::init();
    
    // Create power manager
    let power_config = PowerConfig {
        policy: PowerPolicy::Balanced,
        idle_timeout_ms: 5000,
        sleep_timeout_ms: 30000,
        deep_sleep_timeout_ms: 60000,
    };
    
    let mut power_manager = PowerManager::new(power_config);
    power_manager.init();
    
    // Create frequency scaler
    let freq_config = FrequencyConfig {
        governor: FrequencyGovernor::Ondemand,
        min_frequency: CpuFrequency::MHz400,
        max_frequency: CpuFrequency::MHz2000,
        up_threshold: 80,
        down_threshold: 20,
        sampling_rate_ms: 100,
    };
    
    let mut freq_scaler = FrequencyScaler::new(freq_config);
    freq_scaler.init();
    
    // Create power monitor
    let mut power_monitor = PowerMonitor::new();
    power_monitor.init();
    
    // Create power mode controller
    let mode_config = PowerModeConfig {
        mode: PowerMode::Active,
        wake_up_sources: WakeUpSources {
            gpio: true,
            timer: true,
            uart: false,
            i2c: false,
            spi: false,
            adc: false,
            usb: false,
            ethernet: false,
        },
        retention: true,
    };
    
    let mut mode_controller = PowerModeController::new(mode_config);
    mode_controller.init();
    
    println!("Power management example started");
    
    // Main loop
    loop {
        let current_time = vantisos::time::get_current_time();
        
        // Update power measurement
        power_monitor.update_measurement(current_time);
        
        // Get current power measurement
        let measurement = power_monitor.get_measurement();
        println!("Power: {} mV, {} mA, {} mW", 
                 measurement.voltage_mv, 
                 measurement.current_ma, 
                 measurement.power_mw);
        
        // Update battery information
        power_monitor.update_battery_info();
        let battery_info = power_monitor.get_battery_info();
        println!("Battery: {}%, {} mV, {:?}", 
                 battery_info.level.percentage,
                 battery_info.level.voltage_mv,
                 battery_info.status);
        
        // Update activity
        power_manager.update_activity(current_time);
        
        // Get current power state
        let power_state = power_manager.get_state();
        println!("Power state: {:?}", power_state);
        
        // Get current power policy
        let power_policy = power_manager.get_policy();
        println!("Power policy: {:?}", power_policy);
        
        // Get current CPU frequency
        let cpu_frequency = freq_scaler.get_frequency();
        println!("CPU frequency: {:?}", cpu_frequency);
        
        // Get current load
        let load = freq_scaler.get_load();
        println!("CPU load: {}%", load);
        
        // Update frequency based on load
        freq_scaler.update(current_time, load);
        
        // Sleep for 1 second
        vantisos::time::sleep(1000);
    }
}