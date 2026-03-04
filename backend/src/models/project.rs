use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub target: String,
    pub domain: Option<String>,
    pub source: ProjectSource,
    pub status: ProjectStatus,
    pub created_at: String,
    pub last_deployed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectSource {
    Upload,
    GitRepo { url: String, branch: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Created,
    Deploying,
    Live,
    Failed,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub target: String,
    pub domain: Option<String>,
    pub git_url: Option<String>,
    pub git_branch: Option<String>,
}
