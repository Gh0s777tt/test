//! Edge Computing Example
//! 
//! This example demonstrates how to use the edge computing framework.

use vantisos::edge::framework::*;
use vantisos::edge::processing::*;
use vantisos::edge::sync::*;
use vantisos::edge::offline::*;
use vantisos::edge::aggregation::*;
use vantisos::drivers::iot::sensors::temperature::*;

fn main() {
    // Initialize edge computing
    vantisos::edge::init();
    
    // Create edge framework
    let mut framework = EdgeFramework::new(4);
    framework.init();
    
    // Create data processor
    let processing_config = ProcessingConfig {
        processing_type: ProcessingType::Aggregate,
        batch_size: 10,
        timeout_ms: 1000,
        parallel: false,
    };
    
    let mut processor = DataProcessor::new(processing_config);
    processor.init();
    
    // Create cloud synchronizer
    let sync_config = SyncConfig {
        direction: SyncDirection::Bidirectional,
        interval_ms: 60000,
        retry_count: 3,
        conflict_resolution: ConflictResolution::NewestWins,
    };
    
    let mut synchronizer = CloudSynchronizer::new(sync_config);
    synchronizer.init();
    
    // Create offline manager
    let offline_config = OfflineConfig {
        auto_reconnect: true,
        reconnect_interval_ms: 5000,
        max_queue_size: 1000,
        persist_data: true,
    };
    
    let mut offline_manager = OfflineManager::new(offline_config);
    offline_manager.init();
    
    // Create data aggregator
    let mut aggregator = DataAggregator::new(60000);
    aggregator.init();
    
    // Create temperature sensor
    let temp_config = TemperatureSensorConfig {
        sensor_type: TemperatureSensorType::Custom,
        i2c_address: Some(0x48),
        pin: None,
        update_interval_ms: 1000,
    };
    
    let mut temp_sensor = TemperatureSensor::new(0, temp_config);
    temp_sensor.init();
    
    println!("Edge computing example started");
    
    // Main loop
    loop {
        let current_time = vantisos::time::get_current_time();
        
        // Read temperature
        match temp_sensor.read_celsius() {
            Ok(temp) => {
                println!("Temperature: {:.2}°C", temp);
                
                // Add data point to aggregator
                aggregator.add_data_point(temp, current_time);
                
                // Process data
                let data = format!("{:.2}", temp).as_bytes().to_vec();
                match processor.process(&data) {
                    Ok(result) => {
                        println!("Processed: {} items, {} success, {} failed, {} ms",
                                 result.processed_count,
                                 result.success_count,
                                 result.failure_count,
                                 result.processing_time_ms);
                    }
                    Err(e) => {
                        eprintln!("Processing error: {:?}", e);
                    }
                }
                
                // Create task for edge processing
                let task_config = TaskConfig {
                    priority: TaskPriority::Normal,
                    task_type: TaskType::Compute,
                    timeout_ms: 5000,
                    retry_count: 3,
                    cpu_affinity: None,
                };
                
                let task_id = framework.create_task("process_temperature", task_config);
                framework.submit_task(task_id);
                
                // Complete task
                framework.complete_task(task_id, TaskResult::Success(0));
                
                // Add to offline queue if needed
                if offline_manager.is_offline() {
                    match offline_manager.add_to_queue(data.len() as u32, 1) {
                        Ok(item_id) => {
                            println!("Added to offline queue: {}", item_id);
                        }
                        Err(e) => {
                            eprintln!("Queue error: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading temperature: {:?}", e);
            }
        }
        
        // Aggregate data every 10 seconds
        if current_time % 10000 == 0 {
            match aggregator.aggregate(AggregationType::Average) {
                Ok(result) => {
                    println!("Aggregated: {:.2} ({} samples)", result.value, result.count);
                }
                Err(e) => {
                    eprintln!("Aggregation error: {:?}", e);
                }
            }
        }
        
        // Synchronize with cloud every 60 seconds
        if current_time % 60000 == 0 {
            match synchronizer.sync(current_time) {
                Ok(result) => {
                    println!("Synced: {} uploaded, {} downloaded, {} conflicts, {} ms",
                             result.uploaded_count,
                             result.downloaded_count,
                             result.conflict_count,
                             result.sync_time_ms);
                }
                Err(e) => {
                    eprintln!("Sync error: {:?}", e);
                }
            }
        }
        
        // Process offline queue if online
        if offline_manager.is_online() && offline_manager.get_queue_size() > 0 {
            match offline_manager.process_queue() {
                Ok(count) => {
                    println!("Processed {} items from offline queue", count);
                }
                Err(e) => {
                    eprintln!("Queue processing error: {:?}", e);
                }
            }
        }
        
        // Sleep for 1 second
        vantisos::time::sleep(1000);
    }
}