//! Temperature Sensor Example
//! 
//! This example demonstrates how to use the temperature sensor driver.

use vantisos::drivers::iot::sensors::temperature::*;
use vantisos::drivers::iot::i2c::*;
use vantisos::network::mqtt::*;

fn main() {
    // Initialize I2C
    let i2c_config = I2cConfig {
        speed: I2cSpeed::Standard,
        clock_stretching: true,
        general_call: false,
        slave_address: None,
    };
    
    let i2c_master = I2cMaster::new(0, i2c_config);
    i2c_master.init();
    
    // Create temperature sensor
    let temp_config = TemperatureSensorConfig {
        sensor_type: TemperatureSensorType::Custom,
        i2c_address: Some(0x48),
        pin: None,
        update_interval_ms: 1000,
    };
    
    let mut temp_sensor = TemperatureSensor::new(0, temp_config);
    temp_sensor.init();
    
    // Create MQTT client
    let mqtt_config = MqttConfig {
        version: MqttVersion::Mqtt3_1_1,
        client_id: "vantisos_temp_sensor",
        broker_address: "broker.example.com",
        broker_port: 1883,
        username: Some("user"),
        password: Some("pass"),
        keep_alive: 60,
        clean_session: true,
    };
    
    let mut mqtt_client = MqttClient::new(mqtt_config);
    mqtt_client.init();
    mqtt_client.connect();
    
    // Subscribe to commands
    mqtt_client.subscribe("commands/temperature", MqttQos::AtLeastOnce);
    
    println!("Temperature sensor example started");
    
    // Main loop
    loop {
        // Read temperature
        match temp_sensor.read_celsius() {
            Ok(temp_c) => {
                let temp_f = temp_c * 9.0 / 5.0 + 32.0;
                let temp_k = temp_c + 273.15;
                
                println!("Temperature: {:.2}°C ({:.2}°F, {:.2}K)", temp_c, temp_f, temp_k);
                
                // Publish to MQTT
                let payload = format!("{{&quot;celsius&quot;: {:.2}, &quot;fahrenheit&quot;: {:.2}, &quot;kelvin&quot;: {:.2}}}", 
                                     temp_c, temp_f, temp_k);
                
                let message = MqttMessage {
                    topic: "sensors/temperature",
                    payload: payload.as_bytes(),
                    qos: MqttQos::AtLeastOnce,
                    retain: false,
                };
                
                mqtt_client.publish(message);
            }
            Err(e) => {
                eprintln!("Error reading temperature: {:?}", e);
            }
        }
        
        // Sleep for 1 second
        vantisos::time::sleep(1000);
    }
}