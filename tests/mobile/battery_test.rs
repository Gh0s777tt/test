// VantisOS Mobile Battery Management Tests
// Copyright 2025 VantisOS Team
// Licensed under MPL-2.0

use vantis_mobile::battery::*;
use vantis_ui::flux::*;

#[cfg(test)]
mod battery_initialization_tests {
    use super::*;

    #[test]
    fn test_battery_manager_initialization() {
        let manager = BatteryManager::new();
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_battery_detection() {
        let manager = BatteryManager::new();
        let has_battery = manager.has_battery();
        assert!(has_battery.is_ok());
    }

    #[test]
    fn test_battery_type_detection() {
        let manager = BatteryManager::new();
        let battery_type = manager.get_battery_type();
        assert!(battery_type.is_ok());
    }

    #[test]
    fn test_battery_capacity() {
        let manager = BatteryManager::new();
        let capacity = manager.get_battery_capacity();
        assert!(capacity.is_ok());
    }

    #[test]
    fn test_battery_health_detection() {
        let manager = BatteryManager::new();
        let health = manager.get_battery_health();
        assert!(matches!(health, Ok(BatteryHealth::Good) | Ok(BatteryHealth::Fair) | Ok(BatteryHealth::Poor)));
    }

    #[test]
    fn test_battery_technology() {
        let manager = BatteryManager::new();
        let technology = manager.get_battery_technology();
        assert!(technology.is_ok());
    }

    #[test]
    fn test_battery_manufacturer() {
        let manager = BatteryManager::new();
        let manufacturer = manager.get_battery_manufacturer();
        assert!(manufacturer.is_ok());
    }

    #[test]
    fn test_battery_serial_number() {
        let manager = BatteryManager::new();
        let serial = manager.get_battery_serial_number();
        assert!(serial.is_ok());
    }

    #[test]
    fn test_battery_initialization_time() {
        let start = std::time::Instant::now();
        let manager = BatteryManager::new();
        let initialized = manager.is_initialized();
        let duration = start.elapsed();
        
        assert!(initialized);
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_battery_monitor_initialization() {
        let manager = BatteryManager::new();
        let monitor = manager.get_monitor();
        assert!(monitor.is_some());
    }
}

#[cfg(test)]
mod battery_state_tests {
    use super::*;

    #[test]
    fn test_battery_level() {
        let manager = BatteryManager::new();
        let level = manager.get_battery_level();
        assert!(level >= 0 && level <= 100);
    }

    #[test]
    fn test_battery_state() {
        let manager = BatteryManager::new();
        let state = manager.get_battery_state();
        assert!(matches!(state, BatteryState::Charging | BatteryState::Discharging | BatteryState::Full | BatteryState::NotCharging));
    }

    #[test]
    fn test_battery_charging() {
        let manager = BatteryManager::new();
        let is_charging = manager.is_charging();
        assert!(is_charging.is_ok());
    }

    #[test]
    fn test_battery_discharging() {
        let manager = BatteryManager::new();
        let is_discharging = manager.is_discharging();
        assert!(is_discharging.is_ok());
    }

    #[test]
    fn test_battery_full() {
        let manager = BatteryManager::new();
        let is_full = manager.is_full();
        assert!(is_full.is_ok());
    }

    #[test]
    fn test_battery_plugged_in() {
        let manager = BatteryManager::new();
        let plugged_in = manager.is_plugged_in();
        assert!(plugged_in.is_ok());
    }

    #[test]
    fn test_battery_temperature() {
        let manager = BatteryManager::new();
        let temperature = manager.get_battery_temperature();
        assert!(temperature.is_ok());
    }

    #[test]
    fn test_battery_voltage() {
        let manager = BatteryManager::new();
        let voltage = manager.get_battery_voltage();
        assert!(voltage.is_ok());
    }

    #[test]
    fn test_battery_current() {
        let manager = BatteryManager::new();
        let current = manager.get_battery_current();
        assert!(current.is_ok());
    }

    #[test]
    fn test_charge_cycles() {
        let manager = BatteryManager::new();
        let cycles = manager.get_charge_cycles();
        assert!(cycles.is_ok());
    }

    #[test]
    fn test_battery_state_update() {
        let manager = BatteryManager::new();
        let result = manager.update_battery_state();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_level_change() {
        let manager = BatteryManager::new();
        let level1 = manager.get_battery_level();
        std::thread::sleep(std::time::Duration::from_millis(100));
        manager.update_battery_state().ok();
        let level2 = manager.get_battery_level();
        assert!((level1 - level2).abs() <= 1); // Level should be similar
    }

    #[test]
    fn test_battery_state_change_detection() {
        let manager = BatteryManager::new();
        let state_before = manager.get_battery_state();
        let result = manager.listen_to_state_changes();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_estimated_time_remaining() {
        let manager = BatteryManager::new();
        let time = manager.get_estimated_time_remaining();
        assert!(time.is_ok());
    }

    #[test]
    fn test_battery_time_to_full() {
        let manager = BatteryManager::new();
        let time = manager.get_time_to_full();
        if manager.is_charging().unwrap() {
            assert!(time.is_ok());
        }
    }

    #[test]
    fn test_battery_usage_history() {
        let manager = BatteryManager::new();
        let history = manager.get_usage_history(Duration::Hours(24));
        assert!(history.is_ok());
    }

    #[test]
    fn test_battery_drain_rate() {
        let manager = BatteryManager::new();
        let rate = manager.get_drain_rate();
        assert!(rate.is_ok());
    }
}

#[cfg(test)]
mod battery_charging_tests {
    use super::*;

    #[test]
    fn test_charger_detection() {
        let manager = BatteryManager::new();
        let charger = manager.get_charger_type();
        assert!(matches!(charger, Ok(ChargerType::None) | Ok(ChargerType::USB) | Ok(ChargerType::AC) | Ok(ChargerType::Wireless)));
    }

    #[test]
    fn test_charging_speed() {
        let manager = BatteryManager::new();
        let speed = manager.get_charging_speed();
        assert!(speed.is_ok());
    }

    #[test]
    fn test_fast_charging_support() {
        let manager = BatteryManager::new();
        let supported = manager.supports_fast_charging();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_fast_charging_enabled() {
        let manager = BatteryManager::new();
        let enabled = manager.is_fast_charging_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_wireless_charging_support() {
        let manager = BatteryManager::new();
        let supported = manager.supports_wireless_charging();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_wireless_charging_active() {
        let manager = BatteryManager::new();
        let active = manager.is_wireless_charging();
        assert!(active.is_ok());
    }

    #[test]
    fn test_charging_profile_selection() {
        let manager = BatteryManager::new();
        let profile = ChargingProfile::Balanced;
        let result = manager.set_charging_profile(profile);
        assert!(result.is_ok());
    }

    #[test]
    fn test_charging_profile_get() {
        let manager = BatteryManager::new();
        let profile = manager.get_charging_profile();
        assert!(profile.is_ok());
    }

    #[test]
    fn test_charging_profile_performance() {
        let manager = BatteryManager::new();
        let result = manager.set_charging_profile(ChargingProfile::Performance);
        assert!(result.is_ok());
        let profile = manager.get_charging_profile().unwrap();
        assert_eq!(profile, ChargingProfile::Performance);
    }

    #[test]
    fn test_charging_profile_saver() {
        let manager = BatteryManager::new();
        let result = manager.set_charging_profile(ChargingProfile::BatterySaver);
        assert!(result.is_ok());
    }

    #[test]
    fn test_charging_profile_adaptive() {
        let manager = BatteryManager::new();
        let result = manager.set_charging_profile(ChargingProfile::Adaptive);
        assert!(result.is_ok());
    }

    #[test]
    fn test_charge_limit_setting() {
        let manager = BatteryManager::new();
        if manager.supports_charge_limit().unwrap_or(false) {
            let result = manager.set_charge_limit(80);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_charge_limit_get() {
        let manager = BatteryManager::new();
        if manager.supports_charge_limit().unwrap_or(false) {
            let limit = manager.get_charge_limit();
            assert!(limit.is_ok());
        }
    }

    #[test]
    fn test_charge_limit_support() {
        let manager = BatteryManager::new();
        let supported = manager.supports_charge_limit();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_optimized_charging() {
        let manager = BatteryManager::new();
        let result = manager.set_optimized_charging(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimized_charging_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_optimized_charging_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_charging_notification() {
        let manager = BatteryManager::new();
        let result = manager.set_charging_notification(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_charging_complete_notification() {
        let manager = BatteryManager::new();
        let result = manager.set_charging_complete_notification(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_charging_sounds() {
        let manager = BatteryManager::new();
        let result = manager.set_charging_sounds(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_charging_vibration() {
        let manager = BatteryManager::new();
        let result = manager.set_charging_vibration(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reverse_charging_support() {
        let manager = BatteryManager::new();
        let supported = manager.supports_reverse_charging();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_reverse_charging_enabled() {
        let manager = BatteryManager::new();
        if manager.supports_reverse_charging().unwrap_or(false) {
            let result = manager.set_reverse_charging(true);
            assert!(result.is_ok());
        }
    }
}

#[cfg(test)]
mod battery_optimization_tests {
    use super::*;

    #[test]
    fn test_power_saver_mode() {
        let manager = BatteryManager::new();
        let result = manager.set_power_saver_mode(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_power_saver_mode_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_power_saver_mode_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_power_saver_mode_toggle() {
        let manager = BatteryManager::new();
        let before = manager.is_power_saver_mode_enabled().unwrap();
        manager.set_power_saver_mode(!before).ok();
        let after = manager.is_power_saver_mode_enabled().unwrap();
        assert_ne!(before, after);
    }

    #[test]
    fn test_power_saver_mode_threshold() {
        let manager = BatteryManager::new();
        let result = manager.set_power_saver_threshold(20);
        assert!(result.is_ok());
    }

    #[test]
    fn test_power_saver_mode_threshold_get() {
        let manager = BatteryManager::new();
        let threshold = manager.get_power_saver_threshold();
        assert!(threshold.is_ok());
    }

    #[test]
    fn test_power_saver_mode_auto_enable() {
        let manager = BatteryManager::new();
        let result = manager.set_auto_power_saver(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_power_saver_mode_auto_enable_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_auto_power_saver_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_background_activity_restriction() {
        let manager = BatteryManager::new();
        let result = manager.restrict_background_activity(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_background_activity_restriction_status() {
        let manager = BatteryManager::new();
        let restricted = manager.is_background_activity_restricted();
        assert!(restricted.is_ok());
    }

    #[test]
    fn test_app_battery_optimization() {
        let manager = BatteryManager::new();
        let result = manager.optimize_app("com.example.app");
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_battery_optimization_status() {
        let manager = BatteryManager::new();
        let optimized = manager.is_app_optimized("com.example.app");
        assert!(optimized.is_ok());
    }

    #[test]
    fn test_unoptimized_apps_list() {
        let manager = BatteryManager::new();
        let apps = manager.get_unoptimized_apps();
        assert!(apps.is_ok());
    }

    #[test]
    fn test_battery_usage_by_app() {
        let manager = BatteryManager::new();
        let usage = manager.get_battery_usage_by_app(Duration::Hours(24));
        assert!(usage.is_ok());
    }

    #[test]
    fn test_battery_usage_by_system() {
        let manager = BatteryManager::new();
        let usage = manager.get_battery_usage_by_system(Duration::Hours(24));
        assert!(usage.is_ok());
    }

    #[test]
    fn test_cpu_throttling() {
        let manager = BatteryManager::new();
        let result = manager.set_cpu_throttling(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cpu_throttling_status() {
        let manager = BatteryManager::new();
        let throttling = manager.is_cpu_throttling_enabled();
        assert!(throttling.is_ok());
    }

    #[test]
    fn test_brightness_optimization() {
        let manager = BatteryManager::new();
        let result = manager.set_brightness_optimization(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_brightness_optimization_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_brightness_optimization_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_location_optimization() {
        let manager = BatteryManager::new();
        let result = manager.set_location_optimization(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_location_optimization_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_location_optimization_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_sync_optimization() {
        let manager = BatteryManager::new();
        let result = manager.set_sync_optimization(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sync_optimization_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_sync_optimization_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_animation_optimization() {
        let manager = BatteryManager::new();
        let result = manager.set_animation_optimization(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_animation_optimization_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_animation_optimization_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_haptic_optimization() {
        let manager = BatteryManager::new();
        let result = manager.set_haptic_optimization(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_haptic_optimization_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_haptic_optimization_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_network_optimization() {
        let manager = BatteryManager::new();
        let result = manager.set_network_optimization(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_network_optimization_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_network_optimization_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_battery_saver_profile_custom() {
        let manager = BatteryManager::new();
        let profile = BatterySaverProfile::Custom {
            cpu_limit: 80,
            brightness_limit: 70,
            background_apps: false,
        };
        let result = manager.set_battery_saver_profile(profile);
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_saver_profile_get() {
        let manager = BatteryManager::new();
        let profile = manager.get_battery_saver_profile();
        assert!(profile.is_ok());
    }

    #[test]
    fn test_adaptive_battery() {
        let manager = BatteryManager::new();
        let result = manager.set_adaptive_battery(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adaptive_battery_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_adaptive_battery_enabled();
        assert!(enabled.is_ok());
    }

    #[test]
    fn test_smart_battery() {
        let manager = BatteryManager::new();
        let result = manager.set_smart_battery(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_smart_battery_status() {
        let manager = BatteryManager::new();
        let enabled = manager.is_smart_battery_enabled();
        assert!(enabled.is_ok());
    }
}

#[cfg(test)]
mod battery_monitoring_tests {
    use super::*;

    #[test]
    fn test_battery_level_monitoring() {
        let manager = BatteryManager::new();
        let result = manager.start_level_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_level_monitoring_stop() {
        let manager = BatteryManager::new();
        manager.start_level_monitoring().ok();
        let result = manager.stop_level_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_state_monitoring() {
        let manager = BatteryManager::new();
        let result = manager.start_state_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_state_monitoring_stop() {
        let manager = BatteryManager::new();
        manager.start_state_monitoring().ok();
        let result = manager.stop_state_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_temperature_monitoring() {
        let manager = BatteryManager::new();
        let result = manager.start_temperature_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_temperature_monitoring_stop() {
        let manager = BatteryManager::new();
        manager.start_temperature_monitoring().ok();
        let result = manager.stop_temperature_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_monitoring_interval() {
        let manager = BatteryManager::new();
        let result = manager.set_monitoring_interval(30);
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_monitoring_interval_get() {
        let manager = BatteryManager::new();
        let interval = manager.get_monitoring_interval();
        assert!(interval.is_ok());
    }

    #[test]
    fn test_battery_level_callback() {
        let manager = BatteryManager::new();
        manager.on_level_change(|level| {
            assert!(level >= 0 && level <= 100);
        });
        assert!(manager.has_level_callback());
    }

    #[test]
    fn test_battery_state_callback() {
        let manager = BatteryManager::new();
        manager.on_state_change(|state| {
            assert!(matches!(state, BatteryState::Charging | BatteryState::Discharging | BatteryState::Full | BatteryState::NotCharging));
        });
        assert!(manager.has_state_callback());
    }

    #[test]
    fn test_battery_temperature_callback() {
        let manager = BatteryManager::new();
        manager.on_temperature_change(|temp| {
            assert!(temp > -20.0 && temp < 100.0);
        });
        assert!(manager.has_temperature_callback());
    }

    #[test]
    fn test_battery_low_warning() {
        let manager = BatteryManager::new();
        let result = manager.set_low_battery_warning(20);
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_low_warning_get() {
        let manager = BatteryManager::new();
        let warning = manager.get_low_battery_warning();
        assert!(warning.is_ok());
    }

    #[test]
    fn test_battery_critical_warning() {
        let manager = BatteryManager::new();
        let result = manager.set_critical_battery_warning(5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_critical_warning_get() {
        let manager = BatteryManager::new();
        let warning = manager.get_critical_battery_warning();
        assert!(warning.is_ok());
    }

    #[test]
    fn test_battery_warning_callback() {
        let manager = BatteryManager::new();
        manager.on_low_battery(|level| {
            assert!(level >= 0 && level <= 100);
        });
        assert!(manager.has_low_battery_callback());
    }

    #[test]
    fn test_battery_critical_callback() {
        let manager = BatteryManager::new();
        manager.on_critical_battery(|level| {
            assert!(level >= 0 && level <= 100);
        });
        assert!(manager.has_critical_battery_callback());
    }

    #[test]
    fn test_battery_logging() {
        let manager = BatteryManager::new();
        let result = manager.start_battery_logging();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_logging_stop() {
        let manager = BatteryManager::new();
        manager.start_battery_logging().ok();
        let result = manager.stop_battery_logging();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_log_export() {
        let manager = BatteryManager::new();
        let result = manager.export_battery_log("battery_log.csv");
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_log_analytics() {
        let manager = BatteryManager::new();
        let analytics = manager.get_battery_analytics(Duration::Days(7));
        assert!(analytics.is_ok());
    }

    #[test]
    fn test_battery_trends() {
        let manager = BatteryManager::new();
        let trends = manager.get_battery_trends(Duration::Days(30));
        assert!(trends.is_ok());
    }

    #[test]
    fn test_battery_prediction() {
        let manager = BatteryManager::new();
        let prediction = manager.predict_battery_life();
        assert!(prediction.is_ok());
    }

    #[test]
    fn test_battery_health_monitoring() {
        let manager = BatteryManager::new();
        let result = manager.start_health_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_health_monitoring_stop() {
        let manager = BatteryManager::new();
        manager.start_health_monitoring().ok();
        let result = manager.stop_health_monitoring();
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_health_callback() {
        let manager = BatteryManager::new();
        manager.on_health_change(|health| {
            assert!(matches!(health, BatteryHealth::Good | BatteryHealth::Fair | BatteryHealth::Poor));
        });
        assert!(manager.has_health_callback());
    }
}

#[cfg(test)]
mod battery_performance_tests {
    use super::*;

    #[test]
    fn test_battery_reading_performance() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = manager.get_battery_level();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_state_update_performance() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        for _ in 0..100 {
            manager.update_battery_state().ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_monitoring_performance() {
        let manager = BatteryManager::new();
        manager.set_monitoring_interval(1).ok();
        manager.start_level_monitoring().ok();
        std::thread::sleep(std::time::Duration::from_millis(100));
        manager.stop_level_monitoring().ok();
        // Should complete quickly
    }

    #[test]
    fn test_battery_usage_calculation_performance() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        let _ = manager.get_battery_usage_by_app(Duration::Hours(24));
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_battery_prediction_performance() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        let _ = manager.predict_battery_life();
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_battery_logging_performance() {
        let manager = BatteryManager::new();
        manager.start_battery_logging().ok();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let start = std::time::Instant::now();
        manager.stop_battery_logging().ok();
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_battery_log_export_performance() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        let _ = manager.export_battery_log("test_battery_log.csv");
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn test_memory_usage() {
        let manager = BatteryManager::new();
        let memory_before = manager.get_memory_usage();
        
        for _ in 0..1000 {
            let _ = manager.get_battery_level();
            let _ = manager.get_battery_state();
            let _ = manager.get_battery_temperature();
        }
        
        let memory_after = manager.get_memory_usage();
        let increase = memory_after - memory_before;
        assert!(increase < 5_000_000); // Less than 5MB increase
    }

    #[test]
    fn test_concurrent_readings() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        
        let handles: Vec<_> = (0..10).map(|_| {
            std::thread::spawn(move || {
                let local_manager = BatteryManager::new();
                for _ in 0..100 {
                    let _ = local_manager.get_battery_level();
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_callback_overhead() {
        let manager = BatteryManager::new();
        let mut call_count = 0;
        manager.on_level_change(|_| {
            call_count += 1;
        });
        
        let start = std::time::Instant::now();
        for _ in 0..100 {
            manager.simulate_level_change(_ as i32).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_battery_trend_calculation_performance() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        let _ = manager.get_battery_trends(Duration::Days(30));
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn test_optimization_toggle_performance() {
        let manager = BatteryManager::new();
        let start = std::time::Instant::now();
        for _ in 0..100 {
            manager.set_power_saver_mode(true).ok();
            manager.set_power_saver_mode(false).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200);
    }
}

#[cfg(test)]
mod battery_integration_tests {
    use super::*;

    #[test]
    fn test_battery_with_power_saver() {
        let manager = BatteryManager::new();
        manager.set_power_saver_mode(true).ok();
        let level = manager.get_battery_level();
        assert!(level >= 0 && level <= 100);
    }

    #[test]
    fn test_battery_with_monitoring() {
        let manager = BatteryManager::new();
        manager.start_level_monitoring().ok();
        manager.on_level_change(|level| {
            assert!(level >= 0 && level <= 100);
        });
        
        let level = manager.get_battery_level();
        assert!(level >= 0 && level <= 100);
        
        manager.stop_level_monitoring().ok();
    }

    #[test]
    fn test_battery_with_charging_profile() {
        let manager = BatteryManager::new();
        if manager.is_charging().unwrap() {
            manager.set_charging_profile(ChargingProfile::BatterySaver).ok();
            let profile = manager.get_charging_profile().unwrap();
            assert_eq!(profile, ChargingProfile::BatterySaver);
        }
    }

    #[test]
    fn test_battery_with_optimization() {
        let manager = BatteryManager::new();
        manager.set_power_saver_mode(true).ok();
        manager.set_cpu_throttling(true).ok();
        manager.set_brightness_optimization(true).ok();
        
        let level = manager.get_battery_level();
        assert!(level >= 0 && level <= 100);
    }

    #[test]
    fn test_battery_with_warnings() {
        let manager = BatteryManager::new();
        manager.set_low_battery_warning(20).ok();
        manager.set_critical_battery_warning(5).ok();
        
        manager.on_low_battery(|level| {
            assert!(level >= 0 && level <= 100);
        });
        
        manager.on_critical_battery(|level| {
            assert!(level >= 0 && level <= 100);
        });
    }

    #[test]
    fn test_battery_with_logging() {
        let manager = BatteryManager::new();
        manager.start_battery_logging().ok();
        std::thread::sleep(std::time::Duration::from_millis(100));
        manager.stop_battery_logging().ok();
        
        let result = manager.export_battery_log("integration_log.csv");
        assert!(result.is_ok());
    }

    #[test]
    fn test_battery_with_predictions() {
        let manager = BatteryManager::new();
        let prediction = manager.predict_battery_life().unwrap();
        assert!(prediction.estimated_hours > 0);
    }

    #[test]
    fn test_battery_with_analytics() {
        let manager = BatteryManager::new();
        let analytics = manager.get_battery_analytics(Duration::Days(7)).unwrap();
        assert!(analytics.total_drain >= 0);
    }

    #[test]
    fn test_battery_with_trends() {
        let manager = BatteryManager::new();
        let trends = manager.get_battery_trends(Duration::Days(30)).unwrap();
        assert!(!trends.daily_levels.is_empty());
    }

    #[test]
    fn test_battery_with_health_monitoring() {
        let manager = BatteryManager::new();
        manager.start_health_monitoring().ok();
        manager.on_health_change(|health| {
            assert!(matches!(health, BatteryHealth::Good | BatteryHealth::Fair | BatteryHealth::Poor));
        });
        
        let health = manager.get_battery_health().unwrap();
        assert!(matches!(health, BatteryHealth::Good | BatteryHealth::Fair | BatteryHealth::Poor));
        
        manager.stop_health_monitoring().ok();
    }

    #[test]
    fn test_battery_with_charge_limit() {
        let manager = BatteryManager::new();
        if manager.supports_charge_limit().unwrap_or(false) {
            manager.set_charge_limit(80).ok();
            let limit = manager.get_charge_limit().unwrap();
            assert_eq!(limit, 80);
        }
    }

    #[test]
    fn test_battery_with_fast_charging() {
        let manager = BatteryManager::new();
        if manager.supports_fast_charging().unwrap_or(false) {
            let enabled = manager.is_fast_charging_enabled().unwrap();
            assert!(enabled.is_ok());
        }
    }

    #[test]
    fn test_battery_with_wireless_charging() {
        let manager = BatteryManager::new();
        if manager.supports_wireless_charging().unwrap_or(false) {
            let active = manager.is_wireless_charging().unwrap();
            assert!(active.is_ok());
        }
    }

    #[test]
    fn test_battery_with_adaptive_battery() {
        let manager = BatteryManager::new();
        manager.set_adaptive_battery(true).ok();
        let enabled = manager.is_adaptive_battery_enabled().unwrap();
        assert!(enabled);
    }

    #[test]
    fn test_battery_with_smart_battery() {
        let manager = BatteryManager::new();
        manager.set_smart_battery(true).ok();
        let enabled = manager.is_smart_battery_enabled().unwrap();
        assert!(enabled);
    }

    #[test]
    fn test_battery_with_app_optimization() {
        let manager = BatteryManager::new();
        let apps = manager.get_unoptimized_apps().unwrap();
        for app in apps {
            manager.optimize_app(&app).ok();
        }
    }

    #[test]
    fn test_battery_with_usage_tracking() {
        let manager = BatteryManager::new();
        let app_usage = manager.get_battery_usage_by_app(Duration::Hours(24)).unwrap();
        let system_usage = manager.get_battery_usage_by_system(Duration::Hours(24)).unwrap();
        
        assert!(app_usage.total_usage >= 0);
        assert!(system_usage.total_usage >= 0);
    }

    #[test]
    fn test_battery_with_all_callbacks() {
        let manager = BatteryManager::new();
        
        manager.on_level_change(|level| {
            assert!(level >= 0 && level <= 100);
        });
        
        manager.on_state_change(|state| {
            assert!(matches!(state, BatteryState::Charging | BatteryState::Discharging | BatteryState::Full | BatteryState::NotCharging));
        });
        
        manager.on_temperature_change(|temp| {
            assert!(temp > -20.0 && temp < 100.0);
        });
        
        manager.on_health_change(|health| {
            assert!(matches!(health, BatteryHealth::Good | BatteryHealth::Fair | BatteryHealth::Poor));
        });
        
        manager.on_low_battery(|level| {
            assert!(level >= 0 && level <= 100);
        });
        
        manager.on_critical_battery(|level| {
            assert!(level >= 0 && level <= 100);
        });
        
        assert!(manager.has_level_callback());
        assert!(manager.has_state_callback());
        assert!(manager.has_temperature_callback());
        assert!(manager.has_health_callback());
        assert!(manager.has_low_battery_callback());
        assert!(manager.has_critical_battery_callback());
    }

    #[test]
    fn test_battery_with_reverse_charging() {
        let manager = BatteryManager::new();
        if manager.supports_reverse_charging().unwrap_or(false) {
            manager.set_reverse_charging(true).ok();
            std::thread::sleep(std::time::Duration::from_millis(100));
            manager.set_reverse_charging(false).ok();
        }
    }

    #[test]
    fn test_battery_complete_workflow() {
        let manager = BatteryManager::new();
        
        // Initialize monitoring
        manager.start_level_monitoring().ok();
        manager.start_state_monitoring().ok();
        manager.start_temperature_monitoring().ok();
        
        // Set up callbacks
        manager.on_level_change(|level| {
            if level <= 20 {
                // Trigger low battery actions
            }
        });
        
        // Configure power saving
        manager.set_power_saver_threshold(20).ok();
        manager.set_auto_power_saver(true).ok();
        
        // Enable optimizations
        manager.set_cpu_throttling(true).ok();
        manager.set_brightness_optimization(true).ok();
        
        // Start logging
        manager.start_battery_logging().ok();
        
        // Get current state
        let level = manager.get_battery_level();
        let state = manager.get_battery_state();
        let health = manager.get_battery_health();
        
        assert!(level >= 0 && level <= 100);
        assert!(matches!(state, BatteryState::Charging | BatteryState::Discharging | BatteryState::Full | BatteryState::NotCharging));
        assert!(matches!(health, BatteryHealth::Good | BatteryHealth::Fair | BatteryHealth::Poor));
        
        // Stop monitoring
        manager.stop_level_monitoring().ok();
        manager.stop_state_monitoring().ok();
        manager.stop_temperature_monitoring().ok();
        manager.stop_battery_logging().ok();
    }
}