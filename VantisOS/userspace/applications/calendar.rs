//! # Calendar Application
//!
//! A comprehensive calendar application for VantisOS with support for event management,
//! reminders, recurring events, multiple calendar views, and sharing capabilities.

use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};

/// Calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub category: EventCategory,
    pub priority: Priority,
    pub reminder: Option<Reminder>,
    pub recurrence: Option<Recurrence>,
    pub attendees: Vec<String>,
    pub color: Option<String>,
}

/// Event category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventCategory {
    Work,
    Personal,
    Holiday,
    Meeting,
    Birthday,
    Other,
}

/// Event priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

/// Reminder settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub minutes_before: u32,
    pub method: ReminderMethod,
}

/// Reminder method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReminderMethod {
    Notification,
    Email,
    SMS,
}

/// Recurrence pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurrence {
    pub pattern: RecurrencePattern,
    pub interval: u32,
    pub end_date: Option<DateTime<Utc>>,
}

/// Recurrence pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrencePattern {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Calendar view type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewType {
    Day,
    Week,
    Month,
    Year,
    Agenda,
}

/// Calendar application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    pub events: Vec<Event>,
    pub current_date: NaiveDate,
    pub view_type: ViewType,
    pub selected_date: Option<NaiveDate>,
    pub shared_calendars: Vec<SharedCalendar>,
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

impl Calendar {
    /// Create a new calendar instance
    pub fn new() -> Self {
        Calendar {
            events: Vec::new(),
            current_date: Utc::now().date_naive(),
            view_type: ViewType::Month,
            selected_date: None,
            shared_calendars: Vec::new(),
        }
    }

    /// Add a new event
    pub fn add_event(&mut self, event: Event) -> Result<(), String> {
        self.validate_event(&event)?;
        self.events.push(event);
        self.events.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        Ok(())
    }

    /// Update an existing event
    pub fn update_event(&mut self, id: &str, event: Event) -> Result<(), String> {
        self.validate_event(&event)?;
        if let Some(index) = self.events.iter().position(|e| e.id == id) {
            self.events[index] = event;
            self.events.sort_by(|a, b| a.start_time.cmp(&b.start_time));
            Ok(())
        } else {
            Err("Event not found".to_string())
        }
    }

    /// Delete an event
    pub fn delete_event(&mut self, id: &str) -> Result<(), String> {
        if let Some(index) = self.events.iter().position(|e| e.id == id) {
            self.events.remove(index);
            Ok(())
        } else {
            Err("Event not found".to_string())
        }
    }

    /// Get events for a specific date
    pub fn get_events_for_date(&self, date: NaiveDate) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| {
                let event_date = e.start_time.date_naive();
                event_date == date
            })
            .collect()
    }

    /// Get events for a date range
    pub fn get_events_for_range(&self, start: NaiveDate, end: NaiveDate) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| {
                let event_date = e.start_time.date_naive();
                event_date >= start && event_date <= end
            })
            .collect()
    }

    /// Get upcoming events
    pub fn get_upcoming_events(&self, count: usize) -> Vec<&Event> {
        let now = Utc::now();
        self.events
            .iter()
            .filter(|e| e.start_time > now)
            .take(count)
            .collect()
    }

    /// Get events due for reminders
    pub fn get_due_reminders(&self) -> Vec<&Event> {
        let now = Utc::now();
        self.events
            .iter()
            .filter(|e| {
                if let Some(reminder) = &e.reminder {
                    let reminder_time = e.start_time - Duration::minutes(reminder.minutes_before as i64);
                    now >= reminder_time && now < e.start_time
                } else {
                    false
                }
            })
            .collect()
    }

    /// Check for conflicts
    pub fn check_conflicts(&self, event: &Event) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.id != event.id)
            .filter(|e| {
                !(event.end_time <= e.start_time || event.start_time >= e.end_time)
            })
            .collect()
    }

    /// Search events
    pub fn search_events(&self, query: &str) -> Vec<&Event> {
        let query_lower = query.to_lowercase();
        self.events
            .iter()
            .filter(|e| {
                e.title.to_lowercase().contains(&query_lower)
                    || e.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
                    || e.location.as_ref().map_or(false, |l| l.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Generate recurring events
    pub fn generate_recurring_events(&mut self, event: &Event) -> Vec<Event> {
        let mut generated = Vec::new();
        
        if let Some(recurrence) = &event.recurrence {
            let mut current_start = event.start_time.clone();
            let mut current_end = event.end_time.clone();
            
            loop {
                match recurrence.pattern {
                    RecurrencePattern::Daily => {
                        current_start = current_start + Duration::days(recurrence.interval as i64);
                        current_end = current_end + Duration::days(recurrence.interval as i64);
                    }
                    RecurrencePattern::Weekly => {
                        current_start = current_start + Duration::weeks(recurrence.interval as i64);
                        current_end = current_end + Duration::weeks(recurrence.interval as i64);
                    }
                    RecurrencePattern::Monthly => {
                        // Simplified monthly recurrence
                        let mut new_start = current_start + Duration::days(30 * recurrence.interval as i64);
                        let mut new_end = current_end + Duration::days(30 * recurrence.interval as i64);
                        current_start = new_start;
                        current_end = new_end;
                    }
                    RecurrencePattern::Yearly => {
                        current_start = current_start + Duration::days(365 * recurrence.interval as i64);
                        current_end = current_end + Duration::days(365 * recurrence.interval as i64);
                    }
                }
                
                if let Some(end_date) = recurrence.end_date {
                    if current_start > end_date {
                        break;
                    }
                }
                
                // Limit to 100 generated events
                if generated.len() >= 100 {
                    break;
                }
                
                let mut new_event = event.clone();
                new_event.id = format!("{}-{}", event.id, generated.len() + 1);
                new_event.start_time = current_start;
                new_event.end_time = current_end;
                new_event.reminder = None; // Don't repeat reminders
                generated.push(new_event);
            }
        }
        
        generated
    }

    /// Get month view data
    pub fn get_month_view(&self, year: i32, month: u32) -> MonthView {
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let last_day = first_day + Duration::days(32);
        let last_day = NaiveDate::from_ymd_opt(last_day.year(), last_day.month(), 1).unwrap() - Duration::days(1);
        
        let weeks = Vec::new();
        let mut current_week: Vec<Option<DayInfo>> = Vec::new();
        
        // Pad beginning of month
        let weekday = first_day.weekday().num_days_from_sunday();
        for _ in 0..weekday {
            current_week.push(None);
        }
        
        // Add days
        for day in 1..=last_day.day() {
            let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            let events = self.get_events_for_date(date);
            
            current_week.push(Some(DayInfo {
                date,
                day,
                is_today: date == Utc::now().date_naive(),
                is_selected: self.selected_date == Some(date),
                events_count: events.len(),
            }));
            
            if current_week.len() == 7 {
                weeks.push(current_week.clone());
                current_week.clear();
            }
        }
        
        // Pad end of month
        while current_week.len() < 7 {
            current_week.push(None);
        }
        weeks.push(current_week);
        
        MonthView {
            year,
            month,
            weeks,
            selected_date: self.selected_date,
        }
    }

    /// Set current date
    pub fn set_current_date(&mut self, date: NaiveDate) {
        self.current_date = date;
    }

    /// Set view type
    pub fn set_view_type(&mut self, view: ViewType) {
        self.view_type = view;
    }

    /// Set selected date
    pub fn set_selected_date(&mut self, date: Option<NaiveDate>) {
        self.selected_date = date;
    }

    /// Navigate to previous period
    pub fn navigate_previous(&mut self) {
        match self.view_type {
            ViewType::Month => {
                self.current_date = self.current_date - Duration::days(32);
                let first = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), 1).unwrap();
                self.current_date = first;
            }
            ViewType::Week => {
                self.current_date = self.current_date - Duration::weeks(1);
            }
            ViewType::Day => {
                self.current_date = self.current_date - Duration::days(1);
            }
            _ => {}
        }
    }

    /// Navigate to next period
    pub fn navigate_next(&mut self) {
        match self.view_type {
            ViewType::Month => {
                let last = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month() + 1, 1).unwrap();
                self.current_date = last + Duration::days(32);
                let first = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), 1).unwrap();
                self.current_date = first;
            }
            ViewType::Week => {
                self.current_date = self.current_date + Duration::weeks(1);
            }
            ViewType::Day => {
                self.current_date = self.current_date + Duration::days(1);
            }
            _ => {}
        }
    }

    /// Validate event
    fn validate_event(&self, event: &Event) -> Result<(), String> {
        if event.title.trim().is_empty() {
            return Err("Event title cannot be empty".to_string());
        }
        
        if event.start_time >= event.end_time {
            return Err("End time must be after start time".to_string());
        }
        
        // Check for conflicts
        let conflicts = self.check_conflicts(event);
        if !conflicts.is_empty() {
            return Err(format!("Event conflicts with {} other event(s)", conflicts.len()));
        }
        
        Ok(())
    }
}

/// Month view data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthView {
    pub year: i32,
    pub month: u32,
    pub weeks: Vec<Vec<Option<DayInfo>>>,
    pub selected_date: Option<NaiveDate>,
}

/// Day information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayInfo {
    pub date: NaiveDate,
    pub day: u32,
    pub is_today: bool,
    pub is_selected: bool,
    pub events_count: usize,
}

/// Shared calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedCalendar {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub participants: Vec<String>,
    pub color: String,
}

/// Generate unique event ID
pub fn generate_event_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("evt-{}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_creation() {
        let calendar = Calendar::new();
        assert_eq!(calendar.events.len(), 0);
    }

    #[test]
    fn test_add_event() {
        let mut calendar = Calendar::new();
        let event = Event {
            id: generate_event_id(),
            title: "Test Event".to_string(),
            description: None,
            start_time: Utc::now(),
            end_time: Utc::now() + Duration::hours(1),
            location: None,
            category: EventCategory::Work,
            priority: Priority::Medium,
            reminder: None,
            recurrence: None,
            attendees: Vec::new(),
            color: None,
        };
        
        assert!(calendar.add_event(event).is_ok());
        assert_eq!(calendar.events.len(), 1);
    }

    #[test]
    fn test_event_validation_empty_title() {
        let calendar = Calendar::new();
        let event = Event {
            id: generate_event_id(),
            title: "".to_string(),
            description: None,
            start_time: Utc::now(),
            end_time: Utc::now() + Duration::hours(1),
            location: None,
            category: EventCategory::Work,
            priority: Priority::Medium,
            reminder: None,
            recurrence: None,
            attendees: Vec::new(),
            color: None,
        };
        
        assert!(calendar.add_event(event).is_err());
    }

    #[test]
    fn test_delete_event() {
        let mut calendar = Calendar::new();
        let id = generate_event_id();
        let event = Event {
            id: id.clone(),
            title: "Test Event".to_string(),
            description: None,
            start_time: Utc::now(),
            end_time: Utc::now() + Duration::hours(1),
            location: None,
            category: EventCategory::Work,
            priority: Priority::Medium,
            reminder: None,
            recurrence: None,
            attendees: Vec::new(),
            color: None,
        };
        
        calendar.add_event(event).unwrap();
        assert_eq!(calendar.events.len(), 1);
        
        calendar.delete_event(&id).unwrap();
        assert_eq!(calendar.events.len(), 0);
    }
}