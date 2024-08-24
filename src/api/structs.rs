use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MBZSearchResponse {
    pub created: String,
    pub count: u32,
    offset: u32,
    pub releases: Vec<MBZRelease>,
}

#[derive(Debug, Deserialize)]
pub struct MBZRelease {
    pub id: String,
    pub score: String,
    pub count: u32,
    pub title: String,
    #[serde(rename = "status-id")]
    status_id: String,
    pub status: String,
    pub packaging: String,
    #[serde(rename = "text-representation")]
    pub text_representation: MBZTextRepresentation,
    pub date: String,
    pub country: String,
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
