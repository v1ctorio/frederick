use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MBZSearchResponse {
    created: String,
    count: u32,
    offset: u32,
    releases: Vec<MBZRelease>,
}

#[derive(Debug, Deserialize)]
pub struct MBZRelease {
    id: String,
    score: String,
    count: u32,
    title: String,
    #[serde(rename = "status-id")]
    status_id: String,
    status: String,
    packaging: String,
    #[serde(rename = "text-representation")]
    text_representation: MBZTextRepresentation,
    date: String,
    country: String,
}

#[derive(Debug, Deserialize)]
pub struct MBZTextRepresentation {
    language: String,
    script: String,
}

#[derive(Debug, Deserialize)]
pub struct MBZReleaseGroupOnlyId {
    id: String,
    #[serde(rename = "primary-type")]
    primary_type: String,
}

#[derive(Debug, Deserialize)]
pub struct MBZReleaseGroup {
    id: String,
    score: String,
    count: u32,
    title: String,
    #[serde(rename = "first-release-date")]
    first_release_date: String,
    #[serde(rename = "primary-type")]
    primary_type: String,
}

#[derive(Debug, Deserialize)]
pub struct MBZReleaseGroupResponse {
    created: String,
    count: u32,
    offset: u32,
    release_groups: Vec<MBZReleaseGroup>,
}
