use std::io::Write;
use std::error::Error;

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_reader, to_writer};

#[derive(Debug)]
pub struct RekordboxPlaylist {
    xml: DjPlaylists,
}

impl RekordboxPlaylist {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        let xml: DjPlaylists = from_reader(file)?;
        Ok(Self { xml })
    }

    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::create(path)?;
        let mut writer = Vec::new();
        to_writer(&mut writer, &self.xml)?;
        Ok(file.write_all(&writer)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct DjPlaylists {
    #[serde(rename = "PRODUCT")]
    pub product: Product,
    #[serde(rename = "COLLECTION")]
    pub collection: Collection,
    #[serde(rename = "PLAYLISTS")]
    pub playlists: Playlists,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Product {
    pub name: String,
    pub version: String,
    pub company: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Collection {
    pub entries: u32,
    #[serde(rename = "TRACK")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Track {
    #[serde(rename = "TrackID")]
    pub track_id: u32,
    pub name: String,
    pub artist: String,
    pub composer: String,
    pub album: String,
    pub grouping: String,
    pub genre: String,
    pub kind: String,
    pub size: u64,
    #[serde(rename = "TotalTime")]
    pub total_time: u32,
    #[serde(rename = "DiscNumber")]
    pub disc_number: u32,
    #[serde(rename = "TrackNumber")]
    pub track_number: u32,
    pub year: u32,
    #[serde(rename = "AverageBpm")]
    pub average_bpm: f32,
    #[serde(rename = "DateAdded")]
    pub date_added: Option<String>,
    #[serde(rename = "BitRate")]
    pub bit_rate: u32,
    #[serde(rename = "SampleRate")]
    pub sample_rate: u32,
    pub comments: String,
    #[serde(rename = "PlayCount")]
    pub play_count: u32,
    pub rating: u32,
    pub location: String,
    pub remixer: String,
    pub tonality: String,
    pub label: String,
    pub mix: String,
    #[serde(rename = "TEMPO")]
    pub tempos: Option<Vec<Tempo>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Tempo {
    pub inizio: f32,
    pub bpm: f32,
    pub metro: String,
    pub battito: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Playlists {
    #[serde(rename = "NODE")]
    pub nodes: Vec<Node>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Node {
    #[serde(rename = "Type")]
    pub node_type: u32,
    pub name: String,
    #[serde(rename = "KeyType")]
    pub key_type: Option<u32>,
    pub entries: Option<u32>,
    #[serde(rename = "Count")]
    pub count: u32,
    #[serde(rename = "TRACK")]
    pub tracks: Option<Vec<NodeTrack>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "TRACK")]
struct NodeTrack {
    pub key: u32,
}
