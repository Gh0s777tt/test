//! IoT Temperature Sensor Example
//! 
//! This example demonstrates how to read temperature data from a sensor
//! and process it using VantisOS IoT framework.

use vantis::verified::drivers::iot::sensors::temperature::TemperatureSensor;
use vantis::verified::drivers::iot::gpio::GpioPin;
use vantis::verified::iot::edge_computing::EdgeTask;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("VantisOS IoT Temperature Sensor Example");
    println!("=========================================\n");
    
    // Initialize temperature sensor (I2C based)
    let mut temp_sensor = TemperatureSensor::new(0x48)?; // I2C address 0x48
    
    // Configure sensor
    temp_sensor.configure_continuous()?;
    println!("Temperature sensor initialized");
    
    // Create GPIO pin for LED indicator
    let mut led = GpioPin::new(17)?; // GPIO 17
    led.set_output()?;
    
    println!("Starting temperature monitoring loop...\n");
    
    // Main monitoring loop
    for i in 1..=10 {
        // Read temperature
        let temp_c = temp_sensor.read_temperature()?;
        let temp_f = temp_c * 9.0 / 5.0 + 32.0;
        
        println!("Reading #{}", i);
        println!("  Temperature: {:.2}°C ({:.2}°F)", temp_c, temp_f);
        
        // LED indication based on temperature
        if temp_c > 30.0 {
            led.set_high()?;
            println!("  Status: HIGH - LED ON");
        } else {
            led.set_low()?;
            println!("  Status: NORMAL - LED OFF");
        }
        
        // Create edge computing task
        let task = EdgeTask::new(format!("temp_reading_{}", i));
        task.add_data("temperature_c", temp_c)?;
        task.add_data("temperature_f", temp_f)?;
        task.add_data("timestamp", chrono::Utc::now().timestamp())?;
        
        // Process task locally
        task.process()?;
        
        // Check threshold and trigger alert
        if temp_c > 35.0 {
            println!("  ⚠️  ALERT: High temperature detected!");
            task.set_alert("high_temperature")?;
        }
        
        // Wait 2 seconds before next reading
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    
    println!("\nTemperature monitoring completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_temperature_reading() {
        let mut sensor = TemperatureSensor::new(0x48).unwrap();
        let temp = sensor.read_temperature().unwrap();
        assert!(temp > -50.0 && temp < 100.0, "Invalid temperature reading");
    }
    
    #[test]
    fn test_gpio_control() {
        let mut led = GpioPin::new(17).unwrap();
        led.set_output().unwrap();
        led.set_high().unwrap();
        led.set_low().unwrap();
    }
}