//! IoT Integration Tests
//! 
//! This module contains integration tests for IoT functionality.

#![cfg(test)]

use vantisos::drivers::iot::*;
use vantisos::power::*;
use vantisos::edge::*;
use vantisos::fs::*;
use vantisos::network::*;

#[test]
fn test_gpio_integration() {
    // Test GPIO pin creation and initialization
    let config = GpioConfig {
        mode: GpioMode::Output,
        pull: GpioPull::None,
        speed: GpioSpeed::Medium,
        alternate: None,
    };
    
    let pin = GpioPin::new(0, 0, config);
    pin.init();
    
    // Test pin operations
    pin.set_high();
    assert_eq!(pin.read(), GpioValue::High);
    
    pin.set_low();
    assert_eq!(pin.read(), GpioValue::Low);
    
    pin.toggle();
    assert_eq!(pin.read(), GpioValue::High);
}

#[test]
fn test_i2c_integration() {
    // Test I2C master creation and initialization
    let config = I2cConfig {
        speed: I2cSpeed::Standard,
        clock_stretching: true,
        general_call: false,
        slave_address: None,
    };
    
    let master = I2cMaster::new(0, config);
    master.init();
    
    // Test I2C operations
    let data = [0x01, 0x02, 0x03];
    assert!(master.write(0x50, &data).is_ok());
    
    let mut buffer = [0u8; 3];
    assert!(master.read(0x50, &mut buffer).is_ok());
}

#[test]
fn test_spi_integration() {
    // Test SPI master creation and initialization
    let config = SpiConfig {
        mode: SpiMode::Mode0,
        bit_order: SpiBitOrder::MsbFirst,
        data_width: SpiDataWidth::Bits8,
        clock_speed: 1000000,
        cs_polarity: false,
    };
    
    let master = SpiMaster::new(0, config);
    master.init();
    
    // Test SPI operations
    let tx_data = [0x01, 0x02, 0x03];
    let mut rx_buffer = [0u8; 3];
    assert!(master.transfer(&tx_data, &mut rx_buffer).is_ok());
}

#[test]
fn test_uart_integration() {
    // Test UART device creation and initialization
    let config = UartConfig {
        baud_rate: 115200,
        data_bits: UartDataBits::Eight,
        parity: UartParity::None,
        stop_bits: UartStopBits::One,
        flow_control: UartFlowControl::None,
    };
    
    let uart = UartDevice::new(0, config);
    uart.init();
    
    // Test UART operations
    assert!(uart.write_byte(0x55).is_ok());
    
    let mut buffer = [0u8; 1];
    assert!(uart.read(&mut buffer).is_ok());
}

#[test]
fn test_pwm_integration() {
    // Test PWM channel creation and initialization
    let config = PwmConfig {
        frequency: 1000,
        duty_cycle: 50,
        polarity: PwmPolarity::Normal,
        mode: PwmMode::EdgeAligned,
    };
    
    let pwm = PwmChannel::new(0, 0, config);
    pwm.init();
    
    // Test PWM operations
    assert!(pwm.set_frequency(2000).is_ok());
    assert!(pwm.set_duty_cycle(75).is_ok());
    
    pwm.start();
    pwm.stop();
}

#[test]
fn test_adc_integration() {
    // Test ADC channel creation and initialization
    let config = AdcConfig {
        resolution: AdcResolution::Bits12,
        reference: AdcReference::Internal2_5V,
        sampling_time: AdcSamplingTime::Cycles56,
        continuous: false,
    };
    
    let adc = AdcChannel::new(0, 0, config);
    adc.init();
    
    // Test ADC operations
    assert!(adc.read_raw().is_ok());
    assert!(adc.read_voltage().is_ok());
}

#[test]
fn test_temperature_sensor_integration() {
    // Test temperature sensor creation and initialization
    let config = TemperatureSensorConfig {
        sensor_type: TemperatureSensorType::Custom,
        i2c_address: Some(0x48),
        pin: None,
        update_interval_ms: 1000,
    };
    
    let mut sensor = TemperatureSensor::new(0, config);
    sensor.init();
    
    // Test temperature reading
    assert!(sensor.read_celsius().is_ok());
    assert!(sensor.read_fahrenheit().is_ok());
    assert!(sensor.read_kelvin().is_ok());
}

#[test]
fn test_humidity_sensor_integration() {
    // Test humidity sensor creation and initialization
    let config = HumiditySensorConfig {
        sensor_type: HumiditySensorType::Custom,
        i2c_address: Some(0x40),
        pin: None,
        update_interval_ms: 1000,
    };
    
    let mut sensor = HumiditySensor::new(0, config);
    sensor.init();
    
    // Test humidity reading
    assert!(sensor.read_humidity().is_ok());
}

#[test]
fn test_pressure_sensor_integration() {
    // Test pressure sensor creation and initialization
    let config = PressureSensorConfig {
        sensor_type: PressureSensorType::Custom,
        i2c_address: Some(0x76),
        update_interval_ms: 1000,
    };
    
    let mut sensor = PressureSensor::new(0, config);
    sensor.init();
    
    // Test pressure reading
    assert!(sensor.read_pascals().is_ok());
    assert!(sensor.read_hectopascals().is_ok());
    assert!(sensor.read_millibars().is_ok());
}

#[test]
fn test_power_management_integration() {
    // Test power manager creation and initialization
    let config = PowerConfig {
        policy: PowerPolicy::Balanced,
        idle_timeout_ms: 5000,
        sleep_timeout_ms: 30000,
        deep_sleep_timeout_ms: 60000,
    };
    
    let mut manager = PowerManager::new(config);
    manager.init();
    
    // Test power state management
    manager.set_state(PowerState::Active);
    assert_eq!(manager.get_state(), PowerState::Active);
    
    manager.set_state(PowerState::Idle);
    assert_eq!(manager.get_state(), PowerState::Idle);
    
    // Test power policy management
    manager.set_policy(PowerPolicy::Performance);
    assert_eq!(manager.get_policy(), PowerPolicy::Performance);
}

#[test]
fn test_edge_computing_integration() {
    // Test edge framework creation and initialization
    let mut framework = EdgeFramework::new(4);
    framework.init();
    
    // Test task creation and submission
    let task_config = TaskConfig {
        priority: TaskPriority::Normal,
        task_type: TaskType::Compute,
        timeout_ms: 5000,
        retry_count: 3,
        cpu_affinity: None,
    };
    
    let task_id = framework.create_task("test_task", task_config);
    assert!(framework.submit_task(task_id).is_ok());
    
    // Test task completion
    framework.complete_task(task_id, TaskResult::Success(0));
    let task = framework.get_task(task_id).unwrap();
    assert_eq!(task.status, TaskStatus::Completed);
}

#[test]
fn test_file_system_integration() {
    // Test ext4 file system creation and initialization
    let config = Ext4Config {
        block_size: 4096,
        inode_size: 256,
        journaling: true,
    };
    
    let mut fs = Ext4FileSystem::new(config);
    fs.init();
    
    // Test file operations
    assert!(fs.create_file("/test.txt", 0o644).is_ok());
    
    let mut buffer = [0u8; 100];
    assert!(fs.read_file("/test.txt", &mut buffer).is_ok());
    
    let data = b"Hello, World!";
    assert!(fs.write_file("/test.txt", data).is_ok());
    
    assert!(fs.delete_file("/test.txt").is_ok());
}

#[test]
fn test_ipv6_integration() {
    // Test IPv6 stack creation and initialization
    let config = Ipv6Config {
        address: Ipv6Address::unspecified(),
        prefix_length: 64,
        gateway: None,
        dns_servers: [None, None],
    };
    
    let mut stack = Ipv6Stack::new(config);
    stack.init();
    
    // Test IPv6 operations
    stack.enable();
    assert!(stack.is_enabled());
    
    let address = Ipv6Address::loopback();
    assert!(address.is_loopback());
}

#[test]
fn test_mqtt_integration() {
    // Test MQTT client creation and initialization
    let config = MqttConfig {
        version: MqttVersion::Mqtt3_1_1,
        client_id: "vantisos_client",
        broker_address: "localhost",
        broker_port: 1883,
        username: None,
        password: None,
        keep_alive: 60,
        clean_session: true,
    };
    
    let mut client = MqttClient::new(config);
    client.init();
    
    // Test MQTT operations
    let message = MqttMessage {
        topic: "test/topic",
        payload: b"Hello, MQTT!",
        qos: MqttQos::AtLeastOnce,
        retain: false,
    };
    
    // Note: Actual connection would require a broker
    // This test verifies the API structure
    assert_eq!(client.get_state(), MqttState::Disconnected);
}

#[test]
fn test_coap_integration() {
    // Test CoAP client creation and initialization
    let config = CoapConfig {
        default_message_type: CoapMessageType::Confirmable,
        timeout_ms: 5000,
        max_retransmit: 4,
    };
    
    let mut client = CoapClient::new(config);
    client.init();
    
    // Test CoAP operations
    // Note: Actual requests would require a server
    // This test verifies the API structure
    assert!(client.get("coap://localhost/test").is_ok());
}