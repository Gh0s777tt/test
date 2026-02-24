// VantisOS Grand Premiere - Launch Event Management System
// Copyright (c) 2025 VantisOS Foundation
// SPDX-License-Identifier: MIT

//! # Grand Premiere
//!
//! Complete launch event management system for VantisOS V1.0, including
//! virtual premiere, press events, marketing campaigns, and media relations.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{NexusError};
use super::nexus_storage::NexusStorage;
use super::nexus_engine::NexusEngine;

/// Event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// Virtual premiere
    VirtualPremiere,
    /// Press event
    PressEvent,
    /// Webinar
    Webinar,
    /// Conference talk
    ConferenceTalk,
    /// Meetup
    Meetup,
    /// Custom event
    Custom,
}

/// Event status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventStatus {
    /// Event is planned
    Planned,
    /// Event is being prepared
    InPreparation,
    /// Event is live
    Live,
    /// Event is completed
    Completed,
    /// Event is cancelled
    Cancelled,
    /// Event is postponed
    Postponed,
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event ID
    pub event_id: Uuid,
    
    /// Event name
    pub name: String,
    
    /// Event description
    pub description: String,
    
    /// Event type
    pub event_type: EventType,
    
    /// Event status
    pub status: EventStatus,
    
    /// Event start time
    pub start_time: u64,
    
    /// Event end time
    pub end_time: u64,
    
    /// Event timezone
    pub timezone: String,
    
    /// Event location (virtual or physical)
    pub location: EventLocation,
    
    /// Expected attendees
    pub expected_attendees: u32,
    
    /// Actual attendees
    pub actual_attendees: u32,
    
    /// Registration URL
    pub registration_url: Option<String>,
    
    /// Stream URL
    pub stream_url: Option<String>,
    
    /// Event agenda
    pub agenda: Vec<AgendaItem>,
    
    /// Speakers
    pub speakers: Vec<Speaker>,
    
    /// Sponsors
    pub sponsors: Vec<Sponsor>,
    
    /// Media partners
    pub media_partners: Vec<MediaPartner>,
    
    /// Event metrics
    pub metrics: EventMetrics,
    
    /// Created at timestamp
    pub created_at: u64,
    
    /// Updated at timestamp
    pub updated_at: u64,
}

/// Event location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLocation {
    /// Location type
    pub location_type: LocationType,
    
    /// Location name
    pub name: String,
    
    /// Address (for physical events)
    pub address: Option<String>,
    
    /// City
    pub city: Option<String>,
    
    /// Country
    pub country: Option<String>,
    
    /// Virtual platform (for virtual events)
    pub virtual_platform: Option<String>,
    
    /// Virtual URL
    pub virtual_url: Option<String>,
}

/// Location type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    /// Virtual event
    Virtual,
    /// Physical event
    Physical,
    /// Hybrid event
    Hybrid,
}

/// Agenda item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaItem {
    /// Item ID
    pub item_id: Uuid,
    
    /// Item title
    pub title: String,
    
    /// Item description
    pub description: String,
    
    /// Start time
    pub start_time: u64,
    
    /// End time
    pub end_time: u64,
    
    /// Speaker ID
    pub speaker_id: Option<Uuid>,
    
    /// Item type
    pub item_type: AgendaItemType,
}

/// Agenda item type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgendaItemType {
    /// Keynote
    Keynote,
    /// Presentation
    Presentation,
    /// Panel discussion
    PanelDiscussion,
    /// Demo
    Demo,
    /// Q&A session
    QASession,
    /// Break
    Break,
    /// Networking
    Networking,
    /// Custom item
    Custom,
}

/// Speaker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speaker {
    /// Speaker ID
    pub speaker_id: Uuid,
    
    /// Speaker name
    pub name: String,
    
    /// Speaker title
    pub title: String,
    
    /// Speaker company
    pub company: String,
    
    /// Speaker bio
    pub bio: String,
    
    /// Speaker photo URL
    pub photo_url: Option<String>,
    
    /// Speaker social media
    pub social_media: HashMap<String, String>,
    
    /// Speaker topics
    pub topics: Vec<String>,
}

/// Sponsor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sponsor {
    /// Sponsor ID
    pub sponsor_id: Uuid,
    
    /// Sponsor name
    pub name: String,
    
    /// Sponsor description
    pub description: String,
    
    /// Sponsor logo URL
    pub logo_url: Option<String>,
    
    /// Sponsor website
    pub website: String,
    
    /// Sponsor tier
    pub tier: SponsorTier,
}

/// Sponsor tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum SponsorTier {
    /// Platinum sponsor
    Platinum = 4,
    /// Gold sponsor
    Gold = 3,
    /// Silver sponsor
    Silver = 2,
    /// Bronze sponsor
    Bronze = 1,
}

/// Media partner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaPartner {
    /// Partner ID
    pub partner_id: Uuid,
    
    /// Partner name
    pub name: String,
    
    /// Partner description
    pub description: String,
    
    /// Partner logo URL
    pub logo_url: Option<String>,
    
    /// Partner website
    pub website: String,
    
    /// Partner type
    pub partner_type: MediaPartnerType,
}

/// Media partner type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaPartnerType {
    /// Tech media
    TechMedia,
    /// Business media
    BusinessMedia,
    /// Security media
    SecurityMedia,
    /// Open source media
    OpenSourceMedia,
    /// Podcast
    Podcast,
    /// YouTube channel
    YouTubeChannel,
}

/// Event metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetrics {
    /// Total registrations
    pub total_registrations: u32,
    
    /// Total attendees
    pub total_attendees: u32,
    
    /// Peak concurrent viewers
    pub peak_concurrent_viewers: u32,
    
    /// Average watch time in minutes
    pub average_watch_time_minutes: u32,
    
    /// Social media impressions
    pub social_media_impressions: u64,
    
    /// Social media engagements
    pub social_media_engagements: u64,
    
    /// Press coverage
    pub press_coverage: u32,
    
    /// Community sign-ups
    pub community_signups: u32,
    
    /// Enterprise inquiries
    pub enterprise_inquiries: u32,
    
    /// GitHub stars
    pub github_stars: u32,
    
    /// GitHub forks
    pub github_forks: u32,
    
    /// Website visits
    pub website_visits: u64,
}

/// Marketing campaign
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingCampaign {
    /// Campaign ID
    pub campaign_id: Uuid,
    
    /// Campaign name
    pub name: String,
    
    /// Campaign description
    pub description: String,
    
    /// Campaign type
    pub campaign_type: CampaignType,
    
    /// Campaign status
    pub status: CampaignStatus,
    
    /// Start date
    pub start_date: u64,
    
    /// End date
    pub end_date: u64,
    
    /// Budget in USD
    pub budget_usd: u64,
    
    /// Actual spend in USD
    pub actual_spend_usd: u64,
    
    /// Target audience
    pub target_audience: Vec<String>,
    
    /// Campaign channels
    pub channels: Vec<CampaignChannel>,
    
    /// Campaign content
    pub content: Vec<CampaignContent>,
    
    /// Campaign metrics
    pub metrics: CampaignMetrics,
    
    /// Created at timestamp
    pub created_at: u64,
    
    /// Updated at timestamp
    pub updated_at: u64,
}

/// Campaign type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignType {
    /// Brand awareness campaign
    BrandAwareness,
    /// Product launch campaign
    ProductLaunch,
    /// Community building campaign
    CommunityBuilding,
    /// Lead generation campaign
    LeadGeneration,
    /// Event promotion campaign
    EventPromotion,
    /// Custom campaign
    Custom,
}

/// Campaign status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignStatus {
    /// Campaign is planned
    Planned,
    /// Campaign is active
    Active,
    /// Campaign is paused
    Paused,
    /// Campaign is completed
    Completed,
    /// Campaign is cancelled
    Cancelled,
}

/// Campaign channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignChannel {
    /// Channel ID
    pub channel_id: Uuid,
    
    /// Channel type
    pub channel_type: ChannelType,
    
    /// Channel name
    pub name: String,
    
    /// Channel URL
    pub url: Option<String>,
    
    /// Budget allocation in USD
    pub budget_usd: u64,
    
    /// Channel status
    pub status: ChannelStatus,
}

/// Channel type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelType {
    /// Social media
    SocialMedia,
    /// Email marketing
    EmailMarketing,
    /// Paid advertising
    PaidAdvertising,
    /// Content marketing
    ContentMarketing,
    /// PR and media
    PRMedia,
    /// Influencer marketing
    InfluencerMarketing,
    /// Community marketing
    CommunityMarketing,
}

/// Channel status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelStatus {
    /// Channel is planned
    Planned,
    /// Channel is active
    Active,
    /// Channel is paused
    Paused,
    /// Channel is completed
    Completed,
}

/// Campaign content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignContent {
    /// Content ID
    pub content_id: Uuid,
    
    /// Content type
    pub content_type: ContentType,
    
    /// Content title
    pub title: String,
    
    /// Content description
    pub description: String,
    
    /// Content URL
    pub url: Option<String>,
    
    /// Content text
    pub text: Option<String>,
    
    /// Content image URL
    pub image_url: Option<String>,
    
    /// Content video URL
    pub video_url: Option<String>,
    
    /// Published at timestamp
    pub published_at: Option<u64>,
    
    /// Content metrics
    pub metrics: ContentMetrics,
}

/// Content type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    /// Blog post
    BlogPost,
    /// Social media post
    SocialMediaPost,
    /// Video
    Video,
    /// Infographic
    Infographic,
    /// Press release
    PressRelease,
    /// Email
    Email,
    /// Landing page
    LandingPage,
    /// Custom content
    Custom,
}

/// Content metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetrics {
    /// Impressions
    pub impressions: u64,
    
    /// Clicks
    pub clicks: u64,
    
    /// Shares
    pub shares: u64,
    
    /// Likes
    pub likes: u64,
    
    /// Comments
    pub comments: u64,
    
    /// Conversions
    pub conversions: u64,
    
    /// Click-through rate (0-100)
    pub ctr: f64,
    
    /// Engagement rate (0-100)
    pub engagement_rate: f64,
}

/// Campaign metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignMetrics {
    /// Total impressions
    pub total_impressions: u64,
    
    /// Total clicks
    pub total_clicks: u64,
    
    /// Total conversions
    pub total_conversions: u64,
    
    /// Total spend in USD
    pub total_spend_usd: u64,
    
    /// Cost per impression in USD
    pub cpi_usd: f64,
    
    /// Cost per click in USD
    pub cpc_usd: f64,
    
    /// Cost per acquisition in USD
    pub cpa_usd: f64,
    
    /// Return on ad spend (0-100)
    pub roas: f64,
}

/// Press release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressRelease {
    /// Press release ID
    pub release_id: Uuid,
    
    /// Release title
    pub title: String,
    
    /// Release summary
    pub summary: String,
    
    /// Release body
    pub body: String,
    
    /// Release date
    pub release_date: u64,
    
    /// Embargo date (if applicable)
    pub embargo_date: Option<u64>,
    
    /// Contact person
    pub contact_person: String,
    
    /// Contact email
    pub contact_email: String,
    
    /// Contact phone
    pub contact_phone: Option<String>,
    
    /// Media contacts
    pub media_contacts: Vec<MediaContact>,
    
    /// Release status
    pub status: PressReleaseStatus,
    
    /// Distribution channels
    pub distribution_channels: Vec<String>,
    
    /// Published at timestamp
    pub published_at: Option<u64>,
    
    /// Release metrics
    pub metrics: PressReleaseMetrics,
}

/// Press release status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressReleaseStatus {
    /// Release is drafted
    Draft,
    /// Release is under review
    UnderReview,
    /// Release is approved
    Approved,
    /// Release is published
    Published,
    /// Release is withdrawn
    Withdrawn,
}

/// Media contact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaContact {
    /// Contact ID
    pub contact_id: Uuid,
    
    /// Contact name
    pub name: String,
    
    /// Contact organization
    pub organization: String,
    
    /// Contact email
    pub email: String,
    
    /// Contact phone
    pub phone: Option<String>,
    
    /// Contact type
    pub contact_type: MediaContactType,
}

/// Media contact type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaContactType {
    /// Journalist
    Journalist,
    /// Editor
    Editor,
    /// Blogger
    Blogger,
    /// Influencer
    Influencer,
    /// Analyst
    Analyst,
}

/// Press release metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressReleaseMetrics {
    /// Total pickups
    pub total_pickups: u32,
    
    /// Total reach
    pub total_reach: u64,
    
    /// Total impressions
    pub total_impressions: u64,
    
    /// Social shares
    pub social_shares: u32,
    
    /// Media mentions
    pub media_mentions: u32,
    
    /// Backlinks
    pub backlinks: u32,
}

/// Grand Premiere Manager
pub struct GrandPremiereManager {
    /// Storage backend
    storage: Arc<NexusStorage>,
    
    /// Core engine
    engine: Arc<NexusEngine>,
    
    /// Events
    events: Arc<RwLock<HashMap<Uuid, Event>>>,
    
    /// Marketing campaigns
    campaigns: Arc<RwLock<HashMap<Uuid, MarketingCampaign>>>,
    
    /// Press releases
    press_releases: Arc<RwLock<HashMap<Uuid, PressRelease>>>,
    
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl GrandPremiereManager {
    /// Create a new Grand Premiere Manager instance
    pub fn new(
        storage: Arc<NexusStorage>,
        engine: Arc<NexusEngine>,
    ) -> Result<Self, NexusError> {
        Ok(Self {
            storage,
            engine,
            events: Arc::new(RwLock::new(HashMap::new())),
            campaigns: Arc::new(RwLock::new(HashMap::new())),
            press_releases: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Create a new event
    pub async fn create_event(
        &self,
        name: String,
        description: String,
        event_type: EventType,
        start_time: u64,
        end_time: u64,
        location: EventLocation,
        expected_attendees: u32,
    ) -> Result<Uuid, NexusError> {
        let event_id = Uuid::new_v4();
        
        let event = Event {
            event_id,
            name,
            description,
            event_type,
            status: EventStatus::Planned,
            start_time,
            end_time,
            timezone: "UTC".to_string(),
            location,
            expected_attendees,
            actual_attendees: 0,
            registration_url: None,
            stream_url: None,
            agenda: Vec::new(),
            speakers: Vec::new(),
            sponsors: Vec::new(),
            media_partners: Vec::new(),
            metrics: EventMetrics {
                total_registrations: 0,
                total_attendees: 0,
                peak_concurrent_viewers: 0,
                average_watch_time_minutes: 0,
                social_media_impressions: 0,
                social_media_engagements: 0,
                press_coverage: 0,
                community_signups: 0,
                enterprise_inquiries: 0,
                github_stars: 0,
                github_forks: 0,
                website_visits: 0,
            },
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let mut events = self.events.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        events.insert(event_id, event);
        
        log::info!("Created event: {} ({})", event_id, name);
        
        Ok(event_id)
    }
    
    /// Get event by ID
    pub fn get_event(&self, event_id: Uuid) -> Option<Event> {
        let events = self.events.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        events.get(&event_id).cloned()
    }
    
    /// Get all events
    pub fn get_all_events(&self) -> Vec<Event> {
        let events = self.events.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        events.values().cloned().collect()
    }
    
    /// Update event status
    pub async fn update_event_status(
        &self,
        event_id: Uuid,
        status: EventStatus,
    ) -> Result<(), NexusError> {
        let mut events = self.events.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let event = events.get_mut(&event_id)
            .ok_or(NexusError::NotFound)?;
        
        event.status = status;
        event.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        log::info!("Updated event {} status to {:?}", event_id, status);
        
        Ok(())
    }
    
    /// Add agenda item to event
    pub async fn add_agenda_item(
        &self,
        event_id: Uuid,
        item: AgendaItem,
    ) -> Result<(), NexusError> {
        let mut events = self.events.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let event = events.get_mut(&event_id)
            .ok_or(NexusError::NotFound)?;
        
        event.agenda.push(item);
        event.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Add speaker to event
    pub async fn add_speaker(
        &self,
        event_id: Uuid,
        speaker: Speaker,
    ) -> Result<(), NexusError> {
        let mut events = self.events.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let event = events.get_mut(&event_id)
            .ok_or(NexusError::NotFound)?;
        
        event.speakers.push(speaker);
        event.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Create a marketing campaign
    pub async fn create_campaign(
        &self,
        name: String,
        description: String,
        campaign_type: CampaignType,
        start_date: u64,
        end_date: u64,
        budget_usd: u64,
    ) -> Result<Uuid, NexusError> {
        let campaign_id = Uuid::new_v4();
        
        let campaign = MarketingCampaign {
            campaign_id,
            name,
            description,
            campaign_type,
            status: CampaignStatus::Planned,
            start_date,
            end_date,
            budget_usd,
            actual_spend_usd: 0,
            target_audience: Vec::new(),
            channels: Vec::new(),
            content: Vec::new(),
            metrics: CampaignMetrics {
                total_impressions: 0,
                total_clicks: 0,
                total_conversions: 0,
                total_spend_usd: 0,
                cpi_usd: 0.0,
                cpc_usd: 0.0,
                cpa_usd: 0.0,
                roas: 0.0,
            },
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let mut campaigns = self.campaigns.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        campaigns.insert(campaign_id, campaign);
        
        log::info!("Created marketing campaign: {} ({})", campaign_id, name);
        
        Ok(campaign_id)
    }
    
    /// Get campaign by ID
    pub fn get_campaign(&self, campaign_id: Uuid) -> Option<MarketingCampaign> {
        let campaigns = self.campaigns.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        campaigns.get(&campaign_id).cloned()
    }
    
    /// Get all campaigns
    pub fn get_all_campaigns(&self) -> Vec<MarketingCampaign> {
        let campaigns = self.campaigns.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        campaigns.values().cloned().collect()
    }
    
    /// Create a press release
    pub async fn create_press_release(
        &self,
        title: String,
        summary: String,
        body: String,
        release_date: u64,
        contact_person: String,
        contact_email: String,
    ) -> Result<Uuid, NexusError> {
        let release_id = Uuid::new_v4();
        
        let press_release = PressRelease {
            release_id,
            title,
            summary,
            body,
            release_date,
            embargo_date: None,
            contact_person,
            contact_email,
            contact_phone: None,
            media_contacts: Vec::new(),
            status: PressReleaseStatus::Draft,
            distribution_channels: Vec::new(),
            published_at: None,
            metrics: PressReleaseMetrics {
                total_pickups: 0,
                total_reach: 0,
                total_impressions: 0,
                social_shares: 0,
                media_mentions: 0,
                backlinks: 0,
            },
        };
        
        let mut press_releases = self.press_releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        press_releases.insert(release_id, press_release);
        
        log::info!("Created press release: {} ({})", release_id, title);
        
        Ok(release_id)
    }
    
    /// Get press release by ID
    pub fn get_press_release(&self, release_id: Uuid) -> Option<PressRelease> {
        let press_releases = self.press_releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        press_releases.get(&release_id).cloned()
    }
    
    /// Get all press releases
    pub fn get_all_press_releases(&self) -> Vec<PressRelease> {
        let press_releases = self.press_releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        press_releases.values().cloned().collect()
    }
    
    /// Update press release status
    pub async fn update_press_release_status(
        &self,
        release_id: Uuid,
        status: PressReleaseStatus,
    ) -> Result<(), NexusError> {
        let mut press_releases = self.press_releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let press_release = press_releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        press_release.status = status;
        
        if status == PressReleaseStatus::Published {
            press_release.published_at = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs());
        }
        
        log::info!("Updated press release {} status to {:?}", release_id, status);
        
        Ok(())
    }
    
    /// Get overall premiere statistics
    pub fn get_premiere_statistics(&self) -> PremiereStatistics {
        let events = self.events.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let campaigns = self.campaigns.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let press_releases = self.press_releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let total_events = events.len();
        let total_campaigns = campaigns.len();
        let total_press_releases = press_releases.len();
        
        let total_expected_attendees: u32 = events.values()
            .map(|e| e.expected_attendees)
            .sum();
        
        let total_actual_attendees: u32 = events.values()
            .map(|e| e.actual_attendees)
            .sum();
        
        let total_budget_usd: u64 = campaigns.values()
            .map(|c| c.budget_usd)
            .sum();
        
        let total_social_impressions: u64 = events.values()
            .map(|e| e.metrics.social_media_impressions)
            .sum();
        
        let total_community_signups: u32 = events.values()
            .map(|e| e.metrics.community_signups)
            .sum();
        
        let total_enterprise_inquiries: u32 = events.values()
            .map(|e| e.metrics.enterprise_inquiries)
            .sum();
        
        PremiereStatistics {
            total_events,
            total_campaigns,
            total_press_releases,
            total_expected_attendees,
            total_actual_attendees,
            total_budget_usd,
            total_social_impressions,
            total_community_signups,
            total_enterprise_inquiries,
        }
    }
}

/// Premiere statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiereStatistics {
    /// Total events
    pub total_events: usize,
    
    /// Total campaigns
    pub total_campaigns: usize,
    
    /// Total press releases
    pub total_press_releases: usize,
    
    /// Total expected attendees
    pub total_expected_attendees: u32,
    
    /// Total actual attendees
    pub total_actual_attendees: u32,
    
    /// Total budget in USD
    pub total_budget_usd: u64,
    
    /// Total social media impressions
    pub total_social_impressions: u64,
    
    /// Total community sign-ups
    pub total_community_signups: u32,
    
    /// Total enterprise inquiries
    pub total_enterprise_inquiries: u32,
}