# VantisOS v0.7.0 IoT Guide

## Overview

VantisOS v0.7.0 "IoT Ready" provides comprehensive support for IoT devices including RISC-V architecture, device drivers, power management, edge computing, file systems, and network protocols.

## Table of Contents

1. [RISC-V Support](#risc-v-support)
2. [IoT Device Drivers](#iot-device-drivers)
3. [Power Management](#power-management)
4. [Edge Computing](#edge-computing)
5. [File Systems](#file-systems)
6. [Network Stack](#network-stack)
7. [Examples](#examples)

---

## RISC-V Support

### Architecture Support

VantisOS v0.7.0 provides full support for RISC-V 64-bit (RV64GC) architecture:

- **Boot Process**: Complete bootloader with stack setup, BSS clearing, and hardware initialization
- **Memory Management**: MMU support with Sv39 addressing, page tables, and virtual memory
- **Interrupt Handling**: Trap vector, exception handling, PLIC, timer interrupts, and system calls
- **Context Switching**: Thread/process switching, FPU state management
- **SBI Interface**: Base, timer, IPI, RFENCE, console, and SRST extensions

### Building for RISC-V

```bash
# Build RISC-V kernel
./build_riscv_kernel.sh

# Run on QEMU
qemu-system-riscv64 -machine virt -kernel kernel.elf -nographic
```

---

## IoT Device Drivers

### GPIO (General Purpose Input/Output)

GPIO driver provides pin control and interrupt support:

```rust
use vantisos::drivers::iot::gpio::*;

// Create GPIO pin
let config = GpioConfig {
    mode: GpioMode::Output,
    pull: GpioPull::None,
    speed: GpioSpeed::Medium,
    alternate: None,
};

let pin = GpioPin::new(0, 0, config);
pin.init();

// Control pin
pin.set_high();
pin.set_low();
pin.toggle();

// Read pin
let value = pin.read();
```

### I2C (Inter-Integrated Circuit)

I2C driver provides master/slave support:

```rust
use vantisos::drivers::iot::i2c::*;

// Create I2C master
let config = I2cConfig {
    speed: I2cSpeed::Standard,
    clock_stretching: true,
    general_call: false,
    slave_address: None,
};

let master = I2cMaster::new(0, config);
master.init();

// Write data
master.write(0x50, &[0x01, 0x02, 0x03]);

// Read data
let mut buffer = [0u8; 3];
master.read(0x50, &mut buffer);

// Scan bus
let devices = master.scan();
```

### SPI (Serial Peripheral Interface)

SPI driver provides master/slave support:

```rust
use vantisos::drivers::iot::spi::*;

// Create SPI master
let config = SpiConfig {
    mode: SpiMode::Mode0,
    bit_order: SpiBitOrder::MsbFirst,
    data_width: SpiDataWidth::Bits8,
    clock_speed: 1000000,
    cs_polarity: false,
};

let master = SpiMaster::new(0, config);
master.init();

// Transfer data
let tx_data = [0x01, 0x02, 0x03];
let mut rx_buffer = [0u8; 3];
master.transfer(&tx_data, &mut rx_buffer);
```

### UART (Universal Asynchronous Receiver-Transmitter)

UART driver provides serial communication:

```rust
use vantisos::drivers::iot::uart::*;

// Create UART device
let config = UartConfig {
    baud_rate: 115200,
    data_bits: UartDataBits::Eight,
    parity: UartParity::None,
    stop_bits: UartStopBits::One,
    flow_control: UartFlowControl::None,
};

let uart = UartDevice::new(0, config);
uart.init();

// Write data
uart.write(b"Hello, World!");

// Read data
let mut buffer = [0u8; 100];
uart.read(&mut buffer);
```

### PWM (Pulse Width Modulation)

PWM driver provides frequency and duty cycle control:

```rust
use vantisos::drivers::iot::pwm::*;

// Create PWM channel
let config = PwmConfig {
    frequency: 1000,
    duty_cycle: 50,
    polarity: PwmPolarity::Normal,
    mode: PwmMode::EdgeAligned,
};

let pwm = PwmChannel::new(0, 0, config);
pwm.init();

// Control PWM
pwm.set_frequency(2000);
pwm.set_duty_cycle(75);
pwm.start();
pwm.stop();
```

### ADC (Analog-to-Digital Converter)

ADC driver provides voltage reading:

```rust
use vantisos::drivers::iot::adc::*;

// Create ADC channel
let config = AdcConfig {
    resolution: AdcResolution::Bits12,
    reference: AdcReference::Internal2_5V,
    sampling_time: AdcSamplingTime::Cycles56,
    continuous: false,
};

let adc = AdcChannel::new(0, 0, config);
adc.init();

// Read voltage
let voltage = adc.read_voltage().unwrap();
println!("Voltage: {:.2} V", voltage);
```

### Sensors

#### Temperature Sensor

```rust
use vantisos::drivers::iot::sensors::temperature::*;

let config = TemperatureSensorConfig {
    sensor_type: TemperatureSensorType::Custom,
    i2c_address: Some(0x48),
    pin: None,
    update_interval_ms: 1000,
};

let mut sensor = TemperatureSensor::new(0, config);
sensor.init();

let temp_c = sensor.read_celsius().unwrap();
let temp_f = sensor.read_fahrenheit().unwrap();
let temp_k = sensor.read_kelvin().unwrap();
```

#### Humidity Sensor

```rust
use vantisos::drivers::iot::sensors::humidity::*;

let config = HumiditySensorConfig {
    sensor_type: HumiditySensorType::Custom,
    i2c_address: Some(0x40),
    pin: None,
    update_interval_ms: 1000,
};

let mut sensor = HumiditySensor::new(0, config);
sensor.init();

let humidity = sensor.read_humidity().unwrap();
```

#### Pressure Sensor

```rust
use vantisos::drivers::iot::sensors::pressure::*;

let config = PressureSensorConfig {
    sensor_type: PressureSensorType::Custom,
    i2c_address: Some(0x76),
    update_interval_ms: 1000,
};

let mut sensor = PressureSensor::new(0, config);
sensor.init();

let pressure = sensor.read_pascals().unwrap();
```

---

## Power Management

### Power States

VantisOS provides 6 power states:

- **Active**: Full power, all peripherals active
- **Idle**: CPU idle, peripherals active
- **Standby**: CPU stopped, some peripherals active
- **Sleep**: CPU stopped, minimal peripherals active
- **DeepSleep**: CPU stopped, most peripherals powered down
- **Off**: Everything powered down

### Power Policies

```rust
use vantisos::power::management::*;

let config = PowerConfig {
    policy: PowerPolicy::Balanced,
    idle_timeout_ms: 5000,
    sleep_timeout_ms: 30000,
    deep_sleep_timeout_ms: 60000,
};

let mut manager = PowerManager::new(config);
manager.init();

// Set power state
manager.set_state(PowerState::Active);

// Set power policy
manager.set_policy(PowerPolicy::PowerSave);
```

### Dynamic Frequency Scaling

```rust
use vantisos::power::frequency::*;

let config = FrequencyConfig {
    governor: FrequencyGovernor::Ondemand,
    min_frequency: CpuFrequency::MHz400,
    max_frequency: CpuFrequency::MHz2000,
    up_threshold: 80,
    down_threshold: 20,
    sampling_rate_ms: 100,
};

let mut scaler = FrequencyScaler::new(config);
scaler.init();

// Update frequency based on load
scaler.update(current_time, load);
```

### Wake-up Mechanisms

```rust
use vantisos::power::wakeup::*;

let mut controller = WakeUpController::new();
controller.init();

// Add wake-up source
let config = WakeUpConfig {
    source: WakeUpSource::Gpio(0),
    trigger: WakeUpTrigger::RisingEdge,
    enabled: true,
};

controller.add_source(config);
```

---

## Edge Computing

### Task Management

```rust
use vantisos::edge::framework::*;

let mut framework = EdgeFramework::new(4);
framework.init();

// Create task
let task_config = TaskConfig {
    priority: TaskPriority::Normal,
    task_type: TaskType::Compute,
    timeout_ms: 5000,
    retry_count: 3,
    cpu_affinity: None,
};

let task_id = framework.create_task("my_task", task_config);
framework.submit_task(task_id);

// Complete task
framework.complete_task(task_id, TaskResult::Success(0));
```

### Local Processing

```rust
use vantisos::edge::processing::*;

let config = ProcessingConfig {
    processing_type: ProcessingType::Filter,
    batch_size: 100,
    timeout_ms: 1000,
    parallel: false,
};

let mut processor = DataProcessor::new(config);
processor.init();

let result = processor.process(data).unwrap();
```

### Cloud Synchronization

```rust
use vantisos::edge::sync::*;

let config = SyncConfig {
    direction: SyncDirection::Bidirectional,
    interval_ms: 60000,
    retry_count: 3,
    conflict_resolution: ConflictResolution::NewestWins,
};

let mut synchronizer = CloudSynchronizer::new(config);
synchronizer.init();

// Synchronize with cloud
let result = synchronizer.sync(current_time).unwrap();
```

### Offline Mode

```rust
use vantisos::edge::offline::*;

let config = OfflineConfig {
    auto_reconnect: true,
    reconnect_interval_ms: 5000,
    max_queue_size: 1000,
    persist_data: true,
};

let mut manager = OfflineManager::new(config);
manager.init();

// Set offline mode
manager.set_offline();

// Add data to queue
manager.add_to_queue(data_size, priority);
```

### Data Aggregation

```rust
use vantisos::edge::aggregation::*;

let mut aggregator = DataAggregator::new(60000);
aggregator.init();

// Add data points
aggregator.add_data_point(25.5, timestamp);
aggregator.add_data_point(26.0, timestamp);

// Aggregate data
let result = aggregator.aggregate(AggregationType::Average).unwrap();
```

---

## File Systems

### ext4

```rust
use vantisos::fs::ext4::*;

let config = Ext4Config {
    block_size: 4096,
    inode_size: 256,
    journaling: true,
};

let mut fs = Ext4FileSystem::new(config);
fs.mount();

// Create file
fs.create_file("/test.txt", 0o644);

// Write file
fs.write_file("/test.txt", b"Hello, World!");

// Read file
let mut buffer = [0u8; 100];
fs.read_file("/test.txt", &mut buffer);

// Delete file
fs.delete_file("/test.txt");

fs.unmount();
```

### FAT32

```rust
use vantisos::fs::fat32::*;

let config = Fat32Config {
    bytes_per_sector: 512,
    sectors_per_cluster: 4,
    reserved_sectors: 32,
    num_fats: 2,
};

let mut fs = Fat32FileSystem::new(config);
fs.mount();

// File operations similar to ext4
fs.create_file("/test.txt");
fs.write_file("/test.txt", b"Hello, World!");

fs.unmount();
```

### exFAT

```rust
use vantisos::fs::exfat::*;

let config = ExFatConfig {
    bytes_per_sector: 512,
    sectors_per_cluster: 4,
    volume_serial: 0x12345678,
};

let mut fs = ExFatFileSystem::new(config);
fs.mount();

// File operations similar to ext4
fs.create_file("/test.txt");
fs.write_file("/test.txt", b"Hello, World!");

fs.unmount();
```

---

## Network Stack

### IPv6

```rust
use vantisos::network::ipv6::*;

let config = Ipv6Config {
    address: Ipv6Address::new([
        0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ]),
    prefix_length: 64,
    gateway: None,
    dns_servers: [None, None],
};

let mut stack = Ipv6Stack::new(config);
stack.init();
stack.enable();
```

### TLS/SSL

```rust
use vantisos::network::tls::*;

let config = TlsConfig {
    version: TlsVersion::Tls1_3,
    cipher_suite: CipherSuite::TlsAes128GcmSha256,
    verify_certificate: true,
    server_name: Some("example.com"),
};

let mut connection = TlsConnection::new(config);
connection.init();
connection.connect();

// Send data
connection.send(b"Hello, TLS!");

// Receive data
let mut buffer = [0u8; 100];
connection.receive(&mut buffer);

connection.disconnect();
```

### VPN

```rust
use vantisos::network::vpn::*;

let config = VpnConfig {
    vpn_type: VpnType::WireGuard,
    server_address: "vpn.example.com",
    server_port: 51820,
    username: None,
    password: None,
    certificate: Some("/path/to/cert.pem"),
};

let mut connection = VpnConnection::new(config);
connection.init();
connection.connect();

// Send data through VPN
connection.send(b"Hello, VPN!");

connection.disconnect();
```

### MQTT

```rust
use vantisos::network::mqtt::*;

let config = MqttConfig {
    version: MqttVersion::Mqtt3_1_1,
    client_id: "vantisos_client",
    broker_address: "broker.example.com",
    broker_port: 1883,
    username: Some("user"),
    password: Some("pass"),
    keep_alive: 60,
    clean_session: true,
};

let mut client = MqttClient::new(config);
client.init();
client.connect();

// Publish message
let message = MqttMessage {
    topic: "sensors/temperature",
    payload: b"25.5",
    qos: MqttQos::AtLeastOnce,
    retain: false,
};

client.publish(message);

// Subscribe to topic
client.subscribe("sensors/#", MqttQos::AtLeastOnce);

client.disconnect();
```

### CoAP

```rust
use vantisos::network::coap::*;

let config = CoapConfig {
    default_message_type: CoapMessageType::Confirmable,
    timeout_ms: 5000,
    max_retransmit: 4,
};

let mut client = CoapClient::new(config);
client.init();

// GET request
let response = client.get("coap://example.com/sensors/temperature").unwrap();

// POST request
let response = client.post("coap://example.com/sensors", b"{&quot;value&quot;: 25.5}").unwrap();

// PUT request
let response = client.put("coap://example.com/config", b"{&quot;interval&quot;: 1000}").unwrap();

// DELETE request
let response = client.delete("coap://example.com/sensors/1").unwrap();
```

---

## Examples

### Complete IoT Device Example

```rust
use vantisos::drivers::iot::*;
use vantisos::power::*;
use vantisos::edge::*;
use vantisos::network::*;

fn main() {
    // Initialize IoT drivers
    vantisos::drivers::iot::init();
    
    // Initialize power management
    vantisos::power::init();
    
    // Initialize edge computing
    vantisos::edge::init();
    
    // Initialize network stack
    vantisos::network::init();
    
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
        client_id: "vantisos_iot_device",
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
    
    // Main loop
    loop {
        // Read temperature
        let temp = temp_sensor.read_celsius().unwrap();
        
        // Publish to MQTT
        let message = MqttMessage {
            topic: "sensors/temperature",
            payload: format!("{:.2}", temp).as_bytes(),
            qos: MqttQos::AtLeastOnce,
            retain: false,
        };
        
        mqtt_client.publish(message);
        
        // Sleep for 1 second
        vantisos::time::sleep(1000);
    }
}
```

---

## Conclusion

VantisOS v0.7.0 "IoT Ready" provides a comprehensive platform for IoT devices with support for RISC-V architecture, extensive device drivers, power management, edge computing, file systems, and network protocols. This guide covers the main features and provides examples to help you get started with IoT development on VantisOS.

For more information, see:
- [Release Notes](../v0.7.0_RELEASE_NOTES.md)
- [API Documentation](../API.md)
- [Examples](../../examples/iot/)