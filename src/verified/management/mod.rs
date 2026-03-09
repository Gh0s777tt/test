//! # Management Tools Module
//! 
//! Ten moduł zawiera narzędzia zarządzania dla systemu enterprise.

pub mod console;
pub mod cli;
pub mod dashboard;
pub mod alerting;
pub mod logging;
pub mod metrics;

pub use console::{WebConsole, ConsoleConfig, ConsoleSession};
pub use cli::{CliManager, CliCommand, CliResult};
pub use dashboard::{Dashboard, DashboardWidget, DashboardConfig};
pub use alerting::{AlertManager, Alert, AlertRule, AlertSeverity};
pub use logging::{LogAggregator, LogEntry, LogLevel};
pub use metrics::{MetricsCollector, Metric, MetricType};

use core::sync::atomic::{AtomicU32, Ordering};

/// Stan systemu zarządzania
static MANAGEMENT_STATE: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł zarządzania
pub fn init() -> Result<(), ManagementError> {
    // Inicjalizuj web console
    console::init()?;
    
    // Inicjalizuj CLI
    cli::init()?;
    
    // Inicjalizuj dashboard
    dashboard::init()?;
    
    // Inicjalizuj alerting
    alerting::init()?;
    
    // Inicjalizuj log aggregation
    logging::init()?;
    
    // Inicjalizuj metrics collection
    metrics::init()?;
    
    MANAGEMENT_STATE.store(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca stan systemu zarządzania
pub fn management_state() -> bool {
    MANAGEMENT_STATE.load(Ordering::Acquire) == 1
}

/// Błędy zarządzania
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagementError {
    /// Błąd konsoli
    ConsoleError,
    /// Błąd CLI
    CliError,
    /// Błąd dashboard
    DashboardError,
    /// Błąd alerting
    AlertingError,
    /// Błąd logowania
    LoggingError,
    /// Błąd metryk
    MetricsError,
}

impl core::fmt::Display for ManagementError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ManagementError::ConsoleError => write!(f, "Console error"),
            ManagementError::CliError => write!(f, "CLI error"),
            ManagementError::DashboardError => write!(f, "Dashboard error"),
            ManagementError::AlertingError => write!(f, "Alerting error"),
            ManagementError::LoggingError => write!(f, "Logging error"),
            ManagementError::MetricsError => write!(f, "Metrics error"),
        }
    }
}

impl core::error::Error for ManagementError {}