// Privacy Module - Right to be Forgotten Implementation
// VantisOS - Secure Microkernel Operating System

use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Status żądania usunięcia danych
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeletionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Typ danych
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    UserProfile,
    LoginData,
    AppData,
    SystemData,
    BackupData,
    LogData,
}

/// Polityka retencji danych
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub login_data: Duration,
    pub user_data: Duration,
    pub app_data: Duration,
    pub system_data: Duration,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            login_data: Duration::from_secs(30 * 24 * 60 * 60), // 30 dni
            user_data: Duration::from_secs(365 * 24 * 60 * 60), // 365 dni
            app_data: Duration::from_secs(90 * 24 * 60 * 60),   // 90 dni
            system_data: Duration::from_secs(7 * 24 * 60 * 60),  // 7 dni
        }
    }
}

/// Rekord danych
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    pub record_id: Uuid,
    pub user_id: Uuid,
    pub data_type: DataType,
    pub location: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Żądanie usunięcia danych
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionRequest {
    pub request_id: Uuid,
    pub user_id: Uuid,
    pub requested_at: DateTime<Utc>,
    pub status: DeletionStatus,
    pub confirmation_token: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

/// Wynik usunięcia danych
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionResult {
    pub user_id: Uuid,
    pub records_deleted: usize,
    pub bytes_deleted: u64,
    pub duration_seconds: f64,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Menedżer prywatności
pub struct PrivacyManager {
    deletion_requests: HashMap<Uuid, DeletionRequest>,
    data_records: HashMap<Uuid, Vec<DataRecord>>,
    retention_policy: RetentionPolicy,
}

impl PrivacyManager {
    /// Tworzy nowy menedżer prywatności
    pub fn new() -> Self {
        Self {
            deletion_requests: HashMap::new(),
            data_records: HashMap::new(),
            retention_policy: RetentionPolicy::default(),
        }
    }

    /// Złóż żądanie usunięcia danych
    pub async fn request_deletion(&mut self, user_id: Uuid) -> Result<DeletionRequest, String> {
        let request_id = Uuid::new_v4();
        let confirmation_token = Self::generate_confirmation_token();
        
        let request = DeletionRequest {
            request_id,
            user_id,
            requested_at: Utc::now(),
            status: DeletionStatus::Pending,
            confirmation_token,
            completed_at: None,
            error_message: None,
        };
        
        self.deletion_requests.insert(request_id, request.clone());
        Ok(request)
    }

    /// Potwierdź żądanie usunięcia
    pub async fn confirm_deletion(&mut self, token: String) -> Result<Uuid, String> {
        let request = self.deletion_requests.values()
            .find(|r| r.confirmation_token == token && r.status == DeletionStatus::Pending)
            .ok_or("Invalid or expired token")?;
        
        let request_id = request.request_id;
        if let Some(req) = self.deletion_requests.get_mut(&request_id) {
            req.status = DeletionStatus::Processing;
        }
        
        Ok(request_id)
    }

    /// Usuń dane użytkownika
    pub async fn delete_user_data(&mut self, user_id: Uuid) -> Result<DeletionResult, String> {
        let start_time = std::time::Instant::now();
        
        // Znajdź wszystkie dane użytkownika
        let records = self.data_records.get(&user_id)
            .ok_or("No data found for user")?
            .clone();
        
        let mut records_deleted = 0;
        let mut bytes_deleted = 0u64;
        
        // Usuń każdy rekord
        for record in &records {
            match self.delete_record(record).await {
                Ok(bytes) => {
                    records_deleted += 1;
                    bytes_deleted += bytes;
                }
                Err(e) => {
                    log::error!("Failed to delete record {}: {}", record.record_id, e);
                }
            }
        }
        
        // Usuń z pamięci
        self.data_records.remove(&user_id);
        
        let duration = start_time.elapsed().as_secs_f64();
        
        Ok(DeletionResult {
            user_id,
            records_deleted,
            bytes_deleted,
            duration_seconds: duration,
            success: true,
            error_message: None,
        })
    }

    /// Automatyczne usuwanie wygasłych danych
    pub async fn delete_expired_data(&mut self) -> Result<Vec<DeletionResult>, String> {
        let mut results = Vec::new();
        let now = Utc::now();
        
        for (user_id, records) in self.data_records.clone().iter() {
            let expired_records: Vec<_> = records.iter()
                .filter(|r| {
                    if let Some(expires_at) = r.expires_at {
                        expires_at < now
                    } else {
                        false
                    }
                })
                .collect();
            
            if !expired_records.is_empty() {
                let start_time = std::time::Instant::now();
                let mut records_deleted = 0;
                let mut bytes_deleted = 0u64;
                
                for record in expired_records {
                    match self.delete_record(record).await {
                        Ok(bytes) => {
                            records_deleted += 1;
                            bytes_deleted += bytes;
                        }
                        Err(e) => {
                            log::error!("Failed to delete expired record {}: {}", record.record_id, e);
                        }
                    }
                }
                
                let duration = start_time.elapsed().as_secs_f64();
                
                results.push(DeletionResult {
                    user_id: *user_id,
                    records_deleted,
                    bytes_deleted,
                    duration_seconds: duration,
                    success: true,
                    error_message: None,
                });
            }
        }
        
        Ok(results)
    }

    /// Sprawdź status żądania usunięcia
    pub fn get_deletion_status(&self, request_id: Uuid) -> Result<DeletionStatus, String> {
        self.deletion_requests.get(&request_id)
            .map(|r| r.status.clone())
            .ok_or("Request not found")
    }

    /// Pobierz politykę retencji
    pub fn get_retention_policy(&self) -> RetentionPolicy {
        self.retention_policy.clone()
    }

    /// Ustaw politykę retencji
    pub fn set_retention_policy(&mut self, policy: RetentionPolicy) -> Result<(), String> {
        self.retention_policy = policy;
        Ok(())
    }

    /// Dodaj rekord danych
    pub fn add_data_record(&mut self, record: DataRecord) {
        self.data_records
            .entry(record.user_id)
            .or_insert_with(Vec::new)
            .push(record);
    }

    /// Usuń pojedynczy rekord
    async fn delete_record(&self, record: &DataRecord) -> Result<u64, String> {
        // Symulacja bezpiecznego usuwania (nadpisywanie 3x)
        let file_size = self.get_file_size(&record.location)?;
        
        // Nadpisanie danych 3x
        for _ in 0..3 {
            self.overwrite_data(&record.location)?;
        }
        
        // Usunięcie pliku
        std::fs::remove_file(&record.location)
            .map_err(|e| format!("Failed to remove file: {}", e))?;
        
        Ok(file_size)
    }

    /// Nadpisuje dane losowymi wartościami
    fn overwrite_data(&self, location: &str) -> Result<(), String> {
        // W rzeczywistej implementacji, nadpisywałbyśmy dane
        // Tutaj symulujemy operację
        Ok(())
    }

    /// Pobiera rozmiar pliku
    fn get_file_size(&self, location: &str) -> Result<u64, String> {
        std::fs::metadata(location)
            .map(|m| m.len())
            .map_err(|e| format!("Failed to get file size: {}", e))
    }

    /// Generuje token potwierdzenia
    fn generate_confirmation_token() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        
        (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_deletion() {
        let mut manager = PrivacyManager::new();
        let user_id = Uuid::new_v4();
        
        let request = manager.request_deletion(user_id).await.unwrap();
        assert_eq!(request.status, DeletionStatus::Pending);
        assert_eq!(request.user_id, user_id);
    }

    #[tokio::test]
    async fn test_confirm_deletion() {
        let mut manager = PrivacyManager::new();
        let user_id = Uuid::new_v4();
        
        let request = manager.request_deletion(user_id).await.unwrap();
        let request_id = manager.confirm_deletion(request.confirmation_token).await.unwrap();
        
        assert_eq!(request_id, request.request_id);
        assert_eq!(manager.get_deletion_status(request_id).unwrap(), DeletionStatus::Processing);
    }

    #[tokio::test]
    async fn test_retention_policy() {
        let manager = PrivacyManager::new();
        let policy = manager.get_retention_policy();
        
        assert_eq!(policy.login_data.as_secs(), 30 * 24 * 60 * 60);
        assert_eq!(policy.user_data.as_secs(), 365 * 24 * 60 * 60);
    }

    #[tokio::test]
    async fn test_add_data_record() {
        let mut manager = PrivacyManager::new();
        let user_id = Uuid::new_v4();
        
        let record = DataRecord {
            record_id: Uuid::new_v4(),
            user_id,
            data_type: DataType::UserProfile,
            location: "/tmp/test.dat".to_string(),
            created_at: Utc::now(),
            expires_at: None,
        };
        
        manager.add_data_record(record);
        assert!(manager.data_records.contains_key(&user_id));
    }
}