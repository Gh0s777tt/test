//! # Enterprise Integration Module
//! 
//! Ten moduł zawiera funkcje integracji enterprise z zewnętrznymi systemami.

pub mod gateway;
pub mod mesh;
pub mod queue;
pub mod database;
pub mod thirdparty;

pub use gateway::{ApiGateway, ApiRoute, ApiConfig};
pub use mesh::{ServiceMesh, Service, ServiceDiscovery, LoadBalancing};
pub use queue::{MessageQueue, QueueType, Message};
pub use database::{DatabaseConnector, DatabaseType, ConnectionConfig};
pub use thirdparty::{ThirdPartyIntegration, IntegrationType, IntegrationConfig};

use core::sync::atomic::{AtomicU32, Ordering};

/// Stan systemu integracji
static INTEGRATION_STATE: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł integracji
pub fn init() -> Result<(), IntegrationError> {
    // Inicjalizuj API gateway
    gateway::init()?;
    
    // Inicjalizuj service mesh
    mesh::init()?;
    
    // Inicjalizuj message queue
    queue::init()?;
    
    // Inicjalizuj database connectors
    database::init()?;
    
    // Inicjalizuj third-party integrations
    thirdparty::init()?;
    
    INTEGRATION_STATE.store(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca stan systemu integracji
pub fn integration_state() -> bool {
    INTEGRATION_STATE.load(Ordering::Acquire) == 1
}

/// Błędy integracji
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationError {
    /// Błąd API gateway
    GatewayError,
    /// Błąd service mesh
    MeshError,
    /// Błąd message queue
    QueueError,
    /// Błąd database
    DatabaseError,
    /// Błąd third-party integration
    ThirdPartyError,
}

impl core::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IntegrationError::GatewayError => write!(f, "API gateway error"),
            IntegrationError::MeshError => write!(f, "Service mesh error"),
            IntegrationError::QueueError => write!(f, "Message queue error"),
            IntegrationError::DatabaseError => write!(f, "Database error"),
            IntegrationError::ThirdPartyError => write!(f, "Third-party integration error"),
        }
    }
}

impl core::error::Error for IntegrationError {}