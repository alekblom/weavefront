use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Deployment {
    pub id: String,
    pub project_id: String,
    pub target: String,
    pub status: DeploymentStatus,
    pub content_hash: Option<String>,
    pub gateway_url: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStatus {
    Queued,
    Uploading,
    Pinning,
    Live,
    Failed,
}
