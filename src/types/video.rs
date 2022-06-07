use serde::Deserialize;
use serde_json::Value;

use super::{endpoints::{EndpointWatch, EndpointBrowse}, channel::Author, misc::*};

/// Represents a video with all the information aviable except for captions und storyboards
pub struct Video{
   pub title: String,
   pub id: String,
   pub author: Author,
   pub ucid: String,
   pub published: String,
   pub views: String,
   pub description_html: String,
   pub length_seconds: i64,
   pub live_now: bool,
   pub premiere_timestamp: String,
   pub video_player: VideoPlayer,
   pub channel_thumbnail: String,
   pub thumbnail: String,
   pub whitelisted_regions: Vec<Value>, // TODO: remove Value and move to correct type
   pub likes: String,
   pub gerne: String,
   pub is_upcoming: bool,
   pub is_family_safe: bool,
}

/// Represents all formats aviable for the current video
pub struct VideoPlayer{
   pub formats: Vec<Format>,
   pub apdaptiveformts: Vec<Format>,
}

// Represents a format
pub struct Format{
   pub itag: i64,
   pub url: String,
   pub mime_type: String,
   pub bitrate: i64,
   pub quality: String,
   pub fps: i64,
   pub quality_label: String,
   pub audio_quality: String,
}

/// Represents a video found by search query
pub struct SearchVideo{
    pub title: String,
    pub id: String,
    pub channel_name: String,
    pub published_text: String,
    pub author: Author,
    pub channel_thumbnail: String,
    pub view_count_text: String,
    pub length_text: String,
    pub thumbnail: String,
    pub endpoint: EndpointWatch,
    pub browse_channel: EndpointBrowse,
}
/// Represents a video shown in a channel
pub struct ChannelVideo{
    pub title: String, 
    pub id: String, 
    pub published_text: String, 
    pub author_name: String,
    pub thumbnail:String,
    pub view_count_text:  String,
    pub length_text: String,
    pub channel_thumbnail: String, 
    pub endpoint: EndpointWatch
}
// Represents a video in a playlist
pub struct PlaylistVideo{
   pub title: String, 
   pub id: String, 
   pub author: Author, 
   pub thumbnail:String,
   pub length_text: String,
   pub index: i64,
   pub endpoint: EndpointWatch
}
// Represents a comment under a video
pub struct Comment{
   pub comment_id: String,
   pub text: String,
   pub author: Author,
   pub is_author_channel_owner: bool,
   pub author_thumbnail: String,
   pub replies: i64,
   pub reply_continuation: String,
   pub published_time_text: String,
   pub vote_count: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct  VideoRenderer{
   pub video_id: String,
   pub thumbnail: Thumbnails,
   pub title: Runs,
   pub long_byline_text: Runs,
   pub published_time_text: SimpleText,
   pub length_text: AccessibilitySimpleText,
   pub view_count_text: SimpleText,
   pub navigation_endpoint: NavigationEndpoint,
   pub badges: Option<Vec<BadgeRendererVec>>,
   pub owner_badges:Option<Vec<BadgeRendererVec>>,
   pub owner_text: Runs,
   pub short_byline_text: Runs,
   pub short_view_count_text: AccessibilitySimpleText,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactVideoRenderer{
    pub video_id: String,
    pub thumbnail: Thumbnails,
    pub title: Title,
    pub long_byline_text: Runs,
    pub published_time_text: SimpleText,
    pub length_text: AccessibilitySimpleText,
    pub view_count_text: SimpleText,
    pub channel_thumbnail: Thumbnails,
    pub navigation_endpoint: NavigationEndpoint,
    pub badges: Option<Vec<BadgeRendererVec>>,
    pub owner_badges:Option<Vec<BadgeRendererVec>>,
    pub short_byline_text: Runs,
    pub short_view_count_text: AccessibilitySimpleText,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridVideoRenderer{
    pub video_id: String,
    pub thumbnail: Thumbnails,
    pub title: Title,
    pub published_time_text: SimpleText,
    pub view_count_text: SimpleText,
    pub navigation_endpoint: NavigationEndpoint,
    pub owner_badges:Option<Vec<BadgeRendererVec>>,
    pub short_view_count_text: AccessibilitySimpleText,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPrimaryInfoRenderer{
    pub title: Runs,
    pub view_count:VideoViewCountRendererWrapper,
    pub date_text: SimpleText
}
#[derive(Debug, Clone, Deserialize)]
pub struct VideoSecondaryInfoRenderer{
    pub owner: Owner,
    pub description: Runs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommentThreadRenderer{
    pub comment: CommentRenderer,
    pub replies: CommentRepliesRenderer
}