#[cfg(test)]
mod tests {
    use super::super::system_monitor::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_system_monitor_new() {
        let monitor = SystemMonitor::new();
        assert!(monitor.config().show_temperatures);
        assert!(monitor.config().show_network_addresses);
    }

    #[test]
    fn test_cpu_monitor_new() {
        let cpu = CpuMonitor::new(4, 60);
        assert_eq!(cpu.cores.len(), 4);
        assert_eq!(cpu.total_history.capacity(), 60);
    }

    #[test]
    fn test_cpu_average_usage() {
        let mut cpu = CpuMonitor::new(4, 60);
        cpu.cores[0].usage = 25.0;
        cpu.cores[1].usage = 50.0;
        cpu.cores[2].usage = 75.0;
        cpu.cores[3].usage = 100.0;
        assert_eq!(cpu.average_usage(), 62.5);
    }

    #[test]
    fn test_cpu_core_properties() {
        let core = CpuCore {
            id: 0,
            usage: 50.0,
            user: 1000,
            system: 500,
            idle: 1500,
            total: 3000,
        };
        assert_eq!(core.id, 0);
        assert_eq!(core.usage, 50.0);
    }

    #[test]
    fn test_temperature() {
        let temp = Temperature {
            name: "CPU".to_string(),
            celsius: 45.0,
            critical: Some(95.0),
            maximum: Some(85.0),
        };
        assert_eq!(temp.name, "CPU");
        assert_eq!(temp.celsius, 45.0);
        assert!(temp.critical.is_some());
    }

    #[test]
    fn test_memory_monitor_new() {
        let memory = MemoryMonitor::new(60);
        assert_eq!(memory.history.capacity(), 60);
    }

    #[test]
    fn test_memory_usage_percent() {
        let mut memory = MemoryMonitor::new(60);
        memory.total = 16_000_000_000; // 16 GB
        memory.used = 8_000_000_000;   // 8 GB
        assert_eq!(memory.usage_percent(), 50.0);
    }

    #[test]
    fn test_memory_swap_percent() {
        let mut memory = MemoryMonitor::new(60);
        memory.swap_total = 4_000_000_000; // 4 GB
        memory.swap_used = 1_000_000_000;   // 1 GB
        assert_eq!(memory.swap_percent(), 25.0);
    }

    #[test]
    fn test_memory_format_bytes() {
        assert_eq!(MemoryMonitor::format_bytes(1024), "1.00 KiB");
        assert_eq!(MemoryMonitor::format_bytes(1_048_576), "1.00 MiB");
        assert_eq!(MemoryMonitor::format_bytes(1_073_741_824), "1.00 GiB");
        assert_eq!(MemoryMonitor::format_bytes(1_099_511_627_776), "1.00 TiB");
    }

    #[test]
    fn test_disk_monitor_new() {
        let disk_monitor = DiskMonitor::new(60);
        assert_eq!(disk_monitor.io_history.capacity(), 60);
    }

    #[test]
    fn test_disk_usage_percent() {
        let disk = Disk {
            name: "sda1".to_string(),
            mount_point: "/".to_string(),
            fs_type: "ext4".to_string(),
            total: 1_000_000_000_000, // 1 TB
            used: 500_000_000_000,     // 500 GB
            free: 500_000_000_000,     // 500 GB
            read_speed: 0,
            write_speed: 0,
            total_read: 0,
            total_written: 0,
            removable: false,
            system: true,
        };
        assert_eq!(disk.usage_percent(), 50.0);
    }

    #[test]
    fn test_network_monitor_new() {
        let network = NetworkMonitor::new(60);
        assert_eq!(network.download_history.capacity(), 60);
    }

    #[test]
    fn test_network_interface() {
        let interface = NetworkInterface {
            name: "eth0".to_string(),
            mac: "00:11:22:33:44:55".to_string(),
            ipv4: vec!["192.168.1.100".to_string()],
            ipv6: vec![],
            is_up: true,
            is_loopback: false,
            rx_speed: 1000,
            tx_speed: 500,
            total_rx: 1000000,
            total_tx: 500000,
            signal_strength: None,
            ssid: None,
        };
        assert_eq!(interface.name, "eth0");
        assert!(interface.is_up);
    }

    #[test]
    fn test_process_manager_new() {
        let pm = ProcessManager::new();
        assert!(pm.processes.is_empty());
        assert_eq!(pm.sort_by, ProcessSort::Cpu);
        assert!(pm.sort_descending);
    }

    #[test]
    fn test_process_new() {
        let process = Process {
            pid: 1000,
            ppid: 1,
            name: "test".to_string(),
            cmd: "test --arg".to_string(),
            user: "user".to_string(),
            state: ProcessState::Running,
            cpu_usage: 10.0,
            memory: 100_000_000,
            memory_percent: 1.0,
            virtual_memory: 500_000_000,
            shared_memory: 0,
            priority: 0,
            nice: 0,
            threads: 1,
            start_time: Instant::now(),
            cpu_time: Duration::from_secs(10),
        };
        assert_eq!(process.pid, 1000);
        assert_eq!(process.state, ProcessState::Running);
    }

    #[test]
    fn test_process_state_display() {
        assert_eq!(format!("{}", ProcessState::Running), "Running");
        assert_eq!(format!("{}", ProcessState::Sleeping), "Sleeping");
        assert_eq!(format!("{}", ProcessState::Zombie), "Zombie");
    }

    #[test]
    fn test_process_sort_by_cpu() {
        let mut pm = ProcessManager::new();
        pm.sort_by = ProcessSort::Cpu;
        pm.sort_descending = true;
        assert_eq!(pm.sort_by, ProcessSort::Cpu);
    }

    #[test]
    fn test_process_sort_by_memory() {
        let mut pm = ProcessManager::new();
        pm.sort_by = ProcessSort::Memory;
        assert_eq!(pm.sort_by, ProcessSort::Memory);
    }

    #[test]
    fn test_process_sort_by_name() {
        let mut pm = ProcessManager::new();
        pm.sort_by = ProcessSort::Name;
        assert_eq!(pm.sort_by, ProcessSort::Name);
    }

    #[test]
    fn test_kill_process() {
        let mut pm = ProcessManager::new();
        let process = Process {
            pid: 1000,
            ppid: 1,
            name: "test".to_string(),
            cmd: "test".to_string(),
            user: "user".to_string(),
            state: ProcessState::Running,
            cpu_usage: 10.0,
            memory: 100_000_000,
            memory_percent: 1.0,
            virtual_memory: 500_000_000,
            shared_memory: 0,
            priority: 0,
            nice: 0,
            threads: 1,
            start_time: Instant::now(),
            cpu_time: Duration::from_secs(10),
        };
        pm.processes.push(process);
        pm.kill_process(1000, ProcessSignal::Terminate).unwrap();
        assert!(pm.processes.is_empty());
    }

    #[test]
    fn test_kill_process_not_found() {
        let mut pm = ProcessManager::new();
        let result = pm.kill_process(999, ProcessSignal::Terminate);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_priority() {
        let mut pm = ProcessManager::new();
        let process = Process {
            pid: 1000,
            ppid: 1,
            name: "test".to_string(),
            cmd: "test".to_string(),
            user: "user".to_string(),
            state: ProcessState::Running,
            cpu_usage: 10.0,
            memory: 100_000_000,
            memory_percent: 1.0,
            virtual_memory: 500_000_000,
            shared_memory: 0,
            priority: 0,
            nice: 0,
            threads: 1,
            start_time: Instant::now(),
            cpu_time: Duration::from_secs(10),
        };
        pm.processes.push(process);
        pm.set_priority(1000, 10).unwrap();
        assert_eq!(pm.processes[0].priority, 10);
    }

    #[test]
    fn test_monitor_config_default() {
        let config = MonitorConfig::default();
        assert_eq!(config.update_interval_ms, 1000);
        assert_eq!(config.graph_history_size, 60);
        assert!(config.show_temperatures);
    }

    #[test]
    fn test_temperature_unit() {
        assert_eq!(std::mem::discriminant(&TemperatureUnit::Celsius), 
                   std::mem::discriminant(&TemperatureUnit::Fahrenheit));
    }

    #[test]
    fn test_network_unit() {
        assert_eq!(std::mem::discriminant(&NetworkUnit::Bits), 
                   std::mem::discriminant(&NetworkUnit::Bytes));
    }

    #[test]
    fn test_system_monitor_update() {
        let mut monitor = SystemMonitor::new();
        let start_len = monitor.cpu().total_history.len();
        monitor.update();
        // After update, history should have increased or stayed same
        assert!(monitor.cpu().total_history.len() >= start_len);
    }

    #[test]
    fn test_io_stats_default() {
        let stats = IoStats::default();
        assert_eq!(stats.read_speed, 0);
        assert_eq!(stats.write_speed, 0);
        assert_eq!(stats.iops, 0);
    }
}