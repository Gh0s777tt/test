//! Notification System for VantisOS
//!
//! This module provides a comprehensive notification system:
//! - Multi-channel notifications (email, SMS, webhook, Slack)
//! - Notification templates
//! - Routing rules
//! - Delivery tracking and retry logic
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::notification::{NotificationManager, NotificationChannel, NotificationMessage};
//!
//! let mut manager = NotificationManager::new();
//! let message = NotificationMessage::new("Alert".to_string(), "System overload detected".to_string());
//! manager.send(message, NotificationChannel::Email);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::{HashMap, VecDeque};

/// Notification channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotificationChannel {
    Email,
    SMS,
    Webhook,
    Slack,
    Teams,
    Discord,
    Push,
    Log,
}

impl NotificationChannel {
    pub fn to_string(&self) -> &str {
        match self {
            NotificationChannel::Email => "email",
            NotificationChannel::SMS => "sms",
            NotificationChannel::Webhook => "webhook",
            NotificationChannel::Slack => "slack",
            NotificationChannel::Teams => "teams",
            NotificationChannel::Discord => "discord",
            NotificationChannel::Push => "push",
            NotificationChannel::Log => "log",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "email" => Some(NotificationChannel::Email),
            "sms" => Some(NotificationChannel::SMS),
            "webhook" => Some(NotificationChannel::Webhook),
            "slack" => Some(NotificationChannel::Slack),
            "teams" => Some(NotificationChannel::Teams),
            "discord" => Some(NotificationChannel::Discord),
            "push" => Some(NotificationChannel::Push),
            "log" => Some(NotificationChannel::Log),
            _ => None,
        }
    }
}

/// Notification message
#[derive(Debug, Clone)]
pub struct NotificationMessage {
    pub id: String,
    pub title: String,
    pub body: String,
    pub recipient: String,
    pub sender: String,
    pub metadata: HashMap<String, String>,
    pub created_at: u64,
}

impl NotificationMessage {
    pub fn new(title: String, body: String) -> Self {
        let id = format!("notif_{}_{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        );

        Self {
            id,
            title,
            body,
            recipient: String::new(),
            sender: "system".to_string(),
            metadata: HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn with_recipient(mut self, recipient: String) -> Self {
        self.recipient = recipient;
        self
    }

    pub fn with_sender(mut self, sender: String) -> Self {
        self.sender = sender;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }
}

/// Delivery status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Retrying,
}

/// Delivery record
#[derive(Debug, Clone)]
pub struct DeliveryRecord {
    pub notification_id: String,
    pub channel: NotificationChannel,
    pub status: DeliveryStatus,
    pub attempts: usize,
    pub max_attempts: usize,
    pub last_attempt: u64,
    pub error_message: Option<String>,
    pub delivered_at: Option<u64>,
}

impl DeliveryRecord {
    pub fn new(notification_id: String, channel: NotificationChannel) -> Self {
        Self {
            notification_id,
            channel,
            status: DeliveryStatus::Pending,
            attempts: 0,
            max_attempts: 3,
            last_attempt: 0,
            error_message: None,
            delivered_at: None,
        }
    }

    pub fn mark_sent(&mut self) {
        self.status = DeliveryStatus::Sent;
        self.last_attempt = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn mark_delivered(&mut self) {
        self.status = DeliveryStatus::Delivered;
        self.delivered_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }

    pub fn mark_failed(&mut self, error: String) {
        self.attempts += 1;
        if self.attempts >= self.max_attempts {
            self.status = DeliveryStatus::Failed;
        } else {
            self.status = DeliveryStatus::Retrying;
        }
        self.error_message = Some(error);
        self.last_attempt = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn can_retry(&self) -> bool {
        matches!(self.status, DeliveryStatus::Pending | DeliveryStatus::Retrying)
            && self.attempts < self.max_attempts
    }
}

/// Notification template
#[derive(Debug, Clone)]
pub struct NotificationTemplate {
    pub name: String,
    pub subject_template: String,
    pub body_template: String,
    pub channels: Vec<NotificationChannel>,
}

impl NotificationTemplate {
    pub fn new(name: String, subject_template: String, body_template: String) -> Self {
        Self {
            name,
            subject_template,
            body_template,
            channels: Vec::new(),
        }
    }

    pub fn with_channels(mut self, channels: Vec<NotificationChannel>) -> Self {
        self.channels = channels;
        self
    }

    pub fn render(&self, variables: &HashMap<String, String>) -> (String, String) {
        let mut subject = self.subject_template.clone();
        let mut body = self.body_template.clone();

        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            subject = subject.replace(&placeholder, value);
            body = body.replace(&placeholder, value);
        }

        (subject, body)
    }
}

/// Routing rule
#[derive(Debug, Clone)]
pub struct RoutingRule {
    pub name: String,
    pub conditions: Vec<RoutingCondition>,
    pub channels: Vec<NotificationChannel>,
    pub recipients: Vec<String>,
    pub priority: u8,
}

#[derive(Debug, Clone)]
pub struct RoutingCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    GreaterThan,
    LessThan,
}

impl RoutingCondition {
    pub fn evaluate(&self, message: &NotificationMessage) -> bool {
        let field_value = match self.field.as_str() {
            "title" => &message.title,
            "body" => &message.body,
            "sender" => &message.sender,
            "recipient" => &message.recipient,
            _ => return false,
        };

        match self.operator {
            ConditionOperator::Equals => field_value == &self.value,
            ConditionOperator::NotEquals => field_value != &self.value,
            ConditionOperator::Contains => field_value.contains(&self.value),
            ConditionOperator::GreaterThan | ConditionOperator::LessThan => {
                if let (Ok(field_num), Ok(value_num)) = (field_value.parse::<f64>(), self.value.parse::<f64>()) {
                    if self.operator == ConditionOperator::GreaterThan {
                        field_num > value_num
                    } else {
                        field_num < value_num
                    }
                } else {
                    false
                }
            }
        }
    }
}

/// Notification configuration
#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub max_queue_size: usize,
    pub max_history_size: usize,
    pub retry_interval: u64,
    pub default_max_attempts: usize,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 10000,
            max_history_size: 100000,
            retry_interval: 60,
            default_max_attempts: 3,
        }
    }
}

/// Notification statistics
#[derive(Debug, Clone, Default)]
pub struct NotificationStats {
    pub total_sent: usize,
    pub total_delivered: usize,
    pub total_failed: usize,
    pub by_channel: HashMap<NotificationChannel, ChannelStats>,
}

#[derive(Debug, Clone, Default)]
pub struct ChannelStats {
    pub sent: usize,
    pub delivered: usize,
    pub failed: usize,
}

/// Notification manager
pub struct NotificationManager {
    pub config: NotificationConfig,
    pub queue: VecDeque<(NotificationMessage, NotificationChannel)>,
    pub delivery_records: HashMap<String, DeliveryRecord>,
    pub templates: HashMap<String, NotificationTemplate>,
    pub routing_rules: Vec<RoutingRule>,
    pub stats: NotificationStats,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            config: NotificationConfig::default(),
            queue: VecDeque::new(),
            delivery_records: HashMap::new(),
            templates: HashMap::new(),
            routing_rules: Vec::new(),
            stats: NotificationStats::default(),
        }
    }

    /// Send a notification
    pub fn send(&mut self, message: NotificationMessage, channel: NotificationChannel) -> Result<()> {
        if self.queue.len() >= self.config.max_queue_size {
            return Err(AIServiceError::QueueFull);
        }

        let notification_id = message.id.clone();
        let record = DeliveryRecord::new(notification_id.clone(), channel);
        self.delivery_records.insert(notification_id, record);

        self.queue.push_back((message, channel));
        Ok(())
    }

    /// Send via multiple channels
    pub fn send_multi(&mut self, message: NotificationMessage, channels: Vec<NotificationChannel>) -> Result<()> {
        for channel in channels {
            let msg = message.clone();
            self.send(msg, channel)?;
        }
        Ok(())
    }

    /// Send using a template
    pub fn send_template(
        &mut self,
        template_name: &str,
        variables: &HashMap<String, String>,
        recipient: String,
    ) -> Result<()> {
        let template = self.templates.get(template_name)
            .ok_or_else(|| AIServiceError::TemplateNotFound(template_name.to_string()))?
            .clone();

        let (subject, body) = template.render(variables);
        let message = NotificationMessage::new(subject, body)
            .with_recipient(recipient);

        for channel in &template.channels {
            self.send(message.clone(), *channel)?;
        }

        Ok(())
    }

    /// Process queue (simulate sending)
    pub fn process_queue(&mut self) -> usize {
        let mut processed = 0;

        while let Some((message, channel)) = self.queue.pop_front() {
            let record = self.delivery_records.get_mut(&message.id);
            
            if let Some(record) = record {
                // Simulate sending (in real implementation, this would call external services)
                let success = self.send_to_channel(&message, channel);

                if success {
                    record.mark_sent();
                    record.mark_delivered();
                    self.stats.total_sent += 1;
                    self.stats.total_delivered += 1;

                    let channel_stats = self.stats.by_channel.entry(channel).or_default();
                    channel_stats.sent += 1;
                    channel_stats.delivered += 1;
                } else {
                    record.mark_failed("Channel unavailable".to_string());
                    self.stats.total_failed += 1;

                    let channel_stats = self.stats.by_channel.entry(channel).or_default();
                    channel_stats.failed += 1;

                    // Re-queue if can retry
                    if record.can_retry() {
                        self.queue.push_back((message, channel));
                    }
                }
            }

            processed += 1;
        }

        processed
    }

    /// Simulate sending to channel
    fn send_to_channel(&self, message: &NotificationMessage, channel: NotificationChannel) -> bool {
        match channel {
            NotificationChannel::Log => {
                println!("[NOTIFICATION] {} - {}", message.title, message.body);
                true
            }
            _ => {
                // In real implementation, this would call external APIs
                true
            }
        }
    }

    /// Apply routing rules
    pub fn apply_routing(&self, message: &NotificationMessage) -> Vec<(NotificationChannel, String)> {
        let mut routes = Vec::new();

        for rule in &self.routing_rules {
            let conditions_met = rule.conditions.iter().all(|c| c.evaluate(message));

            if conditions_met {
                for channel in &rule.channels {
                    for recipient in &rule.recipients {
                        routes.push((*channel, recipient.clone()));
                    }
                }
            }
        }

        routes
    }

    /// Add template
    pub fn add_template(&mut self, template: NotificationTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// Add routing rule
    pub fn add_routing_rule(&mut self, rule: RoutingRule) {
        self.routing_rules.push(rule);
    }

    /// Get delivery status
    pub fn get_delivery_status(&self, notification_id: &str) -> Option<&DeliveryRecord> {
        self.delivery_records.get(notification_id)
    }

    /// Get statistics
    pub fn get_stats(&self) -> NotificationStats {
        self.stats.clone()
    }

    /// Reset manager
    pub fn reset(&mut self) {
        self.queue.clear();
        self.delivery_records.clear();
        self.stats = NotificationStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_message() {
        let message = NotificationMessage::new("Test".to_string(), "Body".to_string())
            .with_recipient("user@example.com".to_string());

        assert_eq!(message.title, "Test");
        assert_eq!(message.recipient, "user@example.com");
    }

    #[test]
    fn test_delivery_record() {
        let mut record = DeliveryRecord::new("test_id".to_string(), NotificationChannel::Email);
        
        record.mark_sent();
        assert_eq!(record.status, DeliveryStatus::Sent);
        
        record.mark_delivered();
        assert_eq!(record.status, DeliveryStatus::Delivered);
        assert!(record.delivered_at.is_some());
    }

    #[test]
    fn test_delivery_retry() {
        let mut record = DeliveryRecord::new("test_id".to_string(), NotificationChannel::Email);
        record.max_attempts = 3;
        
        record.mark_failed("Error 1".to_string());
        assert!(record.can_retry());
        assert_eq!(record.attempts, 1);
        
        record.mark_failed("Error 2".to_string());
        assert!(record.can_retry());
        
        record.mark_failed("Error 3".to_string());
        assert!(!record.can_retry());
        assert_eq!(record.status, DeliveryStatus::Failed);
    }

    #[test]
    fn test_template() {
        let template = NotificationTemplate::new(
            "alert".to_string(),
            "Alert: {{alert_name}}".to_string(),
            "Alert {{alert_name}} triggered at {{time}}".to_string(),
        );

        let mut vars = HashMap::new();
        vars.insert("alert_name".to_string(), "High CPU".to_string());
        vars.insert("time".to_string(), "12:00".to_string());

        let (subject, body) = template.render(&vars);
        
        assert_eq!(subject, "Alert: High CPU");
        assert_eq!(body, "Alert High CPU triggered at 12:00");
    }

    #[test]
    fn test_routing_condition() {
        let message = NotificationMessage::new("Critical Alert".to_string(), "Body".to_string());

        let condition = RoutingCondition {
            field: "title".to_string(),
            operator: ConditionOperator::Contains,
            value: "Critical".to_string(),
        };

        assert!(condition.evaluate(&message));
    }

    #[test]
    fn test_notification_manager() {
        let mut manager = NotificationManager::new();
        let message = NotificationMessage::new("Test".to_string(), "Body".to_string());

        assert!(manager.send(message, NotificationChannel::Log).is_ok());
        assert_eq!(manager.queue.len(), 1);
    }

    #[test]
    fn test_process_queue() {
        let mut manager = NotificationManager::new();
        let message = NotificationMessage::new("Test".to_string(), "Body".to_string());
        
        manager.send(message, NotificationChannel::Log).unwrap();
        let processed = manager.process_queue();
        
        assert_eq!(processed, 1);
        assert_eq!(manager.stats.total_delivered, 1);
    }

    #[test]
    fn test_send_template() {
        let mut manager = NotificationManager::new();
        
        let template = NotificationTemplate::new(
            "alert".to_string(),
            "Alert: {{name}}".to_string(),
            "Alert {{name}} detected".to_string(),
        ).with_channels(vec![NotificationChannel::Log]);
        
        manager.add_template(template);

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Test Alert".to_string());

        assert!(manager.send_template("alert", &vars, "user@example.com".to_string()).is_ok());
    }

    #[test]
    fn test_routing_rules() {
        let mut manager = NotificationManager::new();
        
        let rule = RoutingRule {
            name: "critical_alerts".to_string(),
            conditions: vec![RoutingCondition {
                field: "title".to_string(),
                operator: ConditionOperator::Contains,
                value: "Critical".to_string(),
            }],
            channels: vec![NotificationChannel::Email, NotificationChannel::SMS],
            recipients: vec!["admin@example.com".to_string()],
            priority: 1,
        };
        
        manager.add_routing_rule(rule);

        let message = NotificationMessage::new("Critical Alert".to_string(), "Body".to_string());
        let routes = manager.apply_routing(&message);
        
        assert_eq!(routes.len(), 2);
    }
}