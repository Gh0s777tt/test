//! # Video Player Application
//!
//! A comprehensive video player application for VantisOS with support for
//! various video formats, playback controls, subtitles, and playlist management.

use serde::{Deserialize, Serialize};

/// Supported video formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoFormat {
    MP4,
    AVI,
    MKV,
    WEBM,
    MOV,
    FLV,
    WMV,
}

impl VideoFormat {
    /// Get file extension for format
    pub fn extension(&self) -> &str {
        match self {
            VideoFormat::MP4 => "mp4",
            VideoFormat::AVI => "avi",
            VideoFormat::MKV => "mkv",
            VideoFormat::WEBM => "webm",
            VideoFormat::MOV => "mov",
            VideoFormat::FLV => "flv",
            VideoFormat::WMV => "wmv",
        }
    }

    /// Get format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp4" => Some(VideoFormat::MP4),
            "avi" => Some(VideoFormat::AVI),
            "mkv" => Some(VideoFormat::MKV),
            "webm" => Some(VideoFormat::WEBM),
            "mov" => Some(VideoFormat::MOV),
            "flv" => Some(VideoFormat::FLV),
            "wmv" => Some(VideoFormat::WMV),
            _ => None,
        }
    }
}

/// Video metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration_seconds: u64,
    pub width: u32,
    pub height: u32,
    pub format: VideoFormat,
    pub size_bytes: u64,
    pub fps: f32,
    pub has_audio: bool,
    pub has_subtitles: bool,
}

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Buffering,
}

/// Repeat mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepeatMode {
    None,
    RepeatOne,
    RepeatAll,
}

/// Video track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTrack {
    pub id: String,
    pub language: String,
    pub title: String,
    pub is_default: bool,
}

/// Subtitle track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub id: String,
    pub language: String,
    pub title: String,
    pub is_default: bool,
}

/// Video player state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPlayer {
    pub current_video: Option<VideoInfo>,
    pub playlist: Vec<VideoInfo>,
    pub current_index: usize,
    pub playback_state: PlaybackState,
    pub current_time_seconds: u64,
    pub volume: f32,
    pub playback_speed: f32,
    pub repeat_mode: RepeatMode,
    pub shuffle: bool,
    pub fullscreen: bool,
    pub muted: bool,
    pub video_tracks: Vec<VideoTrack>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub current_video_track: Option<String>,
    pub current_subtitle_track: Option<String>,
}

/// Video information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub path: String,
    pub name: String,
    pub metadata: Option<VideoMetadata>,
}

impl Default for VideoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoPlayer {
    /// Create a new video player instance
    pub fn new() -> Self {
        VideoPlayer {
            current_video: None,
            playlist: Vec::new(),
            current_index: 0,
            playback_state: PlaybackState::Stopped,
            current_time_seconds: 0,
            volume: 1.0,
            playback_speed: 1.0,
            repeat_mode: RepeatMode::None,
            shuffle: false,
            fullscreen: false,
            muted: false,
            video_tracks: Vec::new(),
            subtitle_tracks: Vec::new(),
            current_video_track: None,
            current_subtitle_track: None,
        }
    }

    /// Load video from path
    pub fn load_video(&mut self, path: String) -> Result<(), String> {
        let name = path.split('/').last().unwrap_or("Unknown").to_string();
        
        let metadata = Self::load_metadata(&path)?;
        self.video_tracks = Self::detect_video_tracks(&metadata);
        self.subtitle_tracks = Self::detect_subtitle_tracks(&metadata);
        
        let video_info = VideoInfo {
            path: path.clone(),
            name,
            metadata: Some(metadata),
        };
        
        self.current_video = Some(video_info.clone());
        
        // Update playlist
        if let Some(index) = self.playlist.iter().position(|v| v.path == path) {
            self.current_index = index;
        } else {
            self.playlist.push(video_info);
            self.current_index = self.playlist.len() - 1;
        }
        
        self.current_time_seconds = 0;
        self.playback_state = PlaybackState::Stopped;
        
        // Set default tracks
        if let Some(default_track) = self.video_tracks.first() {
            self.current_video_track = Some(default_track.id.clone());
        }
        if let Some(default_track) = self.subtitle_tracks.first() {
            self.current_subtitle_track = Some(default_track.id.clone());
        }
        
        Ok(())
    }

    /// Load multiple videos into playlist
    pub fn load_playlist(&mut self, paths: Vec<String>) {
        for path in paths {
            if let Ok(video_info) = Self::create_video_info(&path) {
                self.playlist.push(video_info);
            }
        }
        
        if !self.playlist.is_empty() && self.current_video.is_none() {
            self.current_index = 0;
            self.load_video(self.playlist[0].path.clone()).ok();
        }
    }

    /// Play current video
    pub fn play(&mut self) {
        if self.current_video.is_some() {
            self.playback_state = PlaybackState::Playing;
        }
    }

    /// Pause playback
    pub fn pause(&mut self) {
        self.playback_state = PlaybackState::Paused;
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.current_time_seconds = 0;
    }

    /// Toggle play/pause
    pub fn toggle_play(&mut self) {
        match self.playback_state {
            PlaybackState::Playing => self.pause(),
            PlaybackState::Paused => self.play(),
            PlaybackState::Stopped => {
                if self.current_video.is_some() {
                    self.play();
                }
            }
            PlaybackState::Buffering => {}
        }
    }

    /// Seek to specific time
    pub fn seek(&mut self, time_seconds: u64) {
        if let Some(video) = &self.current_video {
            if let Some(metadata) = &video.metadata {
                self.current_time_seconds = time_seconds.min(metadata.duration_seconds);
            }
        }
    }

    /// Seek forward
    pub fn seek_forward(&mut self, seconds: u64) {
        self.seek(self.current_time_seconds + seconds);
    }

    /// Seek backward
    pub fn seek_backward(&mut self, seconds: u64) {
        let new_time = self.current_time_seconds.saturating_sub(seconds);
        self.seek(new_time);
    }

    /// Set volume
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Increase volume
    pub fn volume_up(&mut self) {
        self.set_volume(self.volume + 0.1);
    }

    /// Decrease volume
    pub fn volume_down(&mut self) {
        self.set_volume(self.volume - 0.1);
    }

    /// Toggle mute
    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    /// Set playback speed
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed.clamp(0.25, 4.0);
    }

    /// Set repeat mode
    pub fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
    }

    /// Toggle repeat mode
    pub fn toggle_repeat(&mut self) {
        self.repeat_mode = match self.repeat_mode {
            RepeatMode::None => RepeatMode::RepeatAll,
            RepeatMode::RepeatAll => RepeatMode::RepeatOne,
            RepeatMode::RepeatOne => RepeatMode::None,
        };
    }

    /// Toggle shuffle
    pub fn toggle_shuffle(&mut self) {
        self.shuffle = !self.shuffle;
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }

    /// Play next video in playlist
    pub fn next_video(&mut self) -> bool {
        if self.playlist.is_empty() {
            return false;
        }
        
        if self.shuffle {
            // Random next video
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let random_index = (timestamp % self.playlist.len() as u128) as usize;
            self.current_index = random_index;
        } else {
            self.current_index = (self.current_index + 1) % self.playlist.len();
        }
        
        if let Some(video) = self.playlist.get(self.current_index) {
            self.load_video(video.path.clone()).ok();
            self.play();
            true
        } else {
            false
        }
    }

    /// Play previous video in playlist
    pub fn previous_video(&mut self) -> bool {
        if self.playlist.is_empty() {
            return false;
        }
        
        self.current_index = if self.current_index == 0 {
            self.playlist.len() - 1
        } else {
            self.current_index - 1
        };
        
        if let Some(video) = self.playlist.get(self.current_index) {
            self.load_video(video.path.clone()).ok();
            self.play();
            true
        } else {
            false
        }
    }

    /// Set video track
    pub fn set_video_track(&mut self, track_id: String) -> bool {
        if self.video_tracks.iter().any(|t| t.id == track_id) {
            self.current_video_track = Some(track_id);
            true
        } else {
            false
        }
    }

    /// Set subtitle track
    pub fn set_subtitle_track(&mut self, track_id: Option<String>) -> bool {
        if let Some(ref track_id) = track_id {
            if self.subtitle_tracks.iter().any(|t| t.id == *track_id) {
                self.current_subtitle_track = Some(track_id.clone());
                true
            } else {
                false
            }
        } else {
            self.current_subtitle_track = None;
            true
        }
    }

    /// Get current playback position as percentage
    pub fn playback_percentage(&self) -> f32 {
        if let Some(video) = &self.current_video {
            if let Some(metadata) = &video.metadata {
                if metadata.duration_seconds > 0 {
                    return (self.current_time_seconds as f32 / metadata.duration_seconds as f32) * 100.0;
                }
            }
        }
        0.0
    }

    /// Get formatted time string
    pub fn format_time(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, secs)
        } else {
            format!("{:02}:{:02}", minutes, secs)
        }
    }

    /// Get current time formatted
    pub fn current_time_formatted(&self) -> String {
        Self::format_time(self.current_time_seconds)
    }

    /// Get total duration formatted
    pub fn total_duration_formatted(&self) -> String {
        if let Some(video) = &self.current_video {
            if let Some(metadata) = &video.metadata {
                return Self::format_time(metadata.duration_seconds);
            }
        }
        "00:00".to_string()
    }

    /// Check if next video exists
    pub fn has_next(&self) -> bool {
        !self.playlist.is_empty() && self.current_index < self.playlist.len() - 1
    }

    /// Check if previous video exists
    pub fn has_previous(&self) -> bool {
        !self.playlist.is_empty() && self.current_index > 0
    }

    /// Get playlist count
    pub fn playlist_count(&self) -> usize {
        self.playlist.len()
    }

    /// Get current position (1-based)
    pub fn current_position(&self) -> usize {
        self.current_index + 1
    }

    /// Create video info from path
    fn create_video_info(path: &str) -> Result<VideoInfo, String> {
        let name = path.split('/').last().unwrap_or("Unknown").to_string();
        let metadata = Self::load_metadata(path)?;
        
        Ok(VideoInfo {
            path: path.to_string(),
            name,
            metadata: Some(metadata),
        })
    }

    /// Load video metadata (simplified)
    fn load_metadata(path: &str) -> Result<VideoMetadata, String> {
        let ext = path.split('.').last().unwrap_or("");
        let format = VideoFormat::from_extension(ext)
            .ok_or_else(|| format!("Unsupported format: {}", ext))?;
        
        // In a real implementation, this would read actual video metadata
        Ok(VideoMetadata {
            duration_seconds: 3600, // 1 hour default
            width: 1920,
            height: 1080,
            format,
            size_bytes: 1024 * 1024 * 100, // 100MB default
            fps: 30.0,
            has_audio: true,
            has_subtitles: false,
        })
    }

    /// Detect available video tracks (simplified)
    fn detect_video_tracks(_metadata: &VideoMetadata) -> Vec<VideoTrack> {
        vec![
            VideoTrack {
                id: "default".to_string(),
                language: "en".to_string(),
                title: "Default".to_string(),
                is_default: true,
            }
        ]
    }

    /// Detect available subtitle tracks (simplified)
    fn detect_subtitle_tracks(_metadata: &VideoMetadata) -> Vec<SubtitleTrack> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_player_creation() {
        let player = VideoPlayer::new();
        assert!(player.current_video.is_none());
        assert_eq!(player.playback_state, PlaybackState::Stopped);
    }

    #[test]
    fn test_volume_control() {
        let mut player = VideoPlayer::new();
        player.set_volume(0.5);
        assert_eq!(player.volume, 0.5);
    }

    #[test]
    fn test_volume_clamp() {
        let mut player = VideoPlayer::new();
        player.set_volume(2.0);
        assert_eq!(player.volume, 1.0);
        player.set_volume(-1.0);
        assert_eq!(player.volume, 0.0);
    }

    #[test]
    fn test_playback_speed() {
        let mut player = VideoPlayer::new();
        player.set_playback_speed(2.0);
        assert_eq!(player.playback_speed, 2.0);
    }

    #[test]
    fn test_time_formatting() {
        assert_eq!(VideoPlayer::format_time(0), "00:00");
        assert_eq!(VideoPlayer::format_time(90), "01:30");
        assert_eq!(VideoPlayer::format_time(3661), "01:01:01");
    }

    #[test]
    fn test_video_format_from_extension() {
        assert_eq!(VideoFormat::from_extension("mp4"), Some(VideoFormat::MP4));
        assert_eq!(VideoFormat::from_extension("mkv"), Some(VideoFormat::MKV));
        assert_eq!(VideoFormat::from_extension("unknown"), None);
    }
}