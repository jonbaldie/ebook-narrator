use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;

pub const SCHEMA_VERSION: u32 = 1;
const SILENT_PEAK_DBFS: f32 = -120.0;

fn deserialize_peak_dbfs<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<f32>::deserialize(deserializer)?.unwrap_or(SILENT_PEAK_DBFS))
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trim {
    pub start_frame: u64,
    pub end_frame: u64,
}

impl Trim {
    pub fn checked(start_frame: u64, end_frame: u64, frame_count: u64) -> Result<Self, String> {
        if start_frame >= end_frame {
            return Err("The Trim start must be before its end.".into());
        }
        if end_frame > frame_count {
            return Err("The Trim end is after the last Take frame.".into());
        }
        Ok(Self {
            start_frame,
            end_frame,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub id: String,
    pub source_path: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Take {
    pub id: String,
    pub segment_id: String,
    pub relative_path: String,
    pub created_at: String,
    pub source_device: String,
    pub source_channels: u16,
    pub source_sample_rate: u32,
    pub source_sample_format: String,
    pub wav_channels: u16,
    pub wav_sample_rate: u32,
    pub wav_bits_per_sample: u16,
    pub frame_count: u64,
    #[serde(
        default = "silent_peak_dbfs",
        deserialize_with = "deserialize_peak_dbfs"
    )]
    pub peak_dbfs: f32,
    pub trim: Trim,
}

fn silent_peak_dbfs() -> f32 {
    SILENT_PEAK_DBFS
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub schema_version: u32,
    pub id: String,
    pub source_epub_name: Option<String>,
    pub source_epub_sha256: Option<String>,
    pub segments: Vec<Segment>,
    pub current_segment_index: usize,
    pub takes: Vec<Take>,
    pub selected_take_id: Option<String>,
}

impl Project {
    pub fn new(id: String) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            id,
            source_epub_name: None,
            source_epub_sha256: None,
            segments: Vec::new(),
            current_segment_index: 0,
            takes: Vec::new(),
            selected_take_id: None,
        }
    }

    pub fn current_segment(&self) -> Option<&Segment> {
        self.segments.get(self.current_segment_index)
    }

    pub fn move_segment(&mut self, offset: i32) {
        if self.segments.is_empty() {
            self.current_segment_index = 0;
            return;
        }
        let last = self.segments.len() as i32 - 1;
        self.current_segment_index =
            (self.current_segment_index as i32 + offset).clamp(0, last) as usize;
    }

    pub fn selected_take(&self) -> Option<&Take> {
        let selected = self.selected_take_id.as_ref()?;
        self.takes.iter().find(|take| &take.id == selected)
    }

    pub fn selected_take_mut(&mut self) -> Option<&mut Take> {
        let selected = self.selected_take_id.as_ref()?;
        self.takes.iter_mut().find(|take| &take.id == selected)
    }
}

pub fn safe_project_relative_path(path: &str) -> Result<&Path, String> {
    let candidate = Path::new(path);
    if candidate.is_absolute()
        || candidate
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return Err("The Project path must stay inside the Project directory.".into());
    }
    Ok(candidate)
}
