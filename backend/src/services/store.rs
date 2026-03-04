use crate::models::project::{CreateProjectRequest, Project, ProjectSource, ProjectStatus};
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory project store. Will be replaced with persistent storage later.
#[derive(Clone)]
pub struct ProjectStore {
    projects: Arc<RwLock<Vec<Project>>>,
}

impl ProjectStore {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn list(&self) -> Vec<Project> {
        self.projects.read().await.clone()
    }

    pub async fn get(&self, id: &str) -> Option<Project> {
        self.projects
            .read()
            .await
            .iter()
            .find(|p| p.id == id)
            .cloned()
    }

    pub async fn create(&self, req: CreateProjectRequest) -> Project {
        let id = format!("proj_{}", hex_id());
        let source = match (req.git_url, req.git_branch) {
            (Some(url), branch) => ProjectSource::GitRepo {
                url,
                branch: branch.unwrap_or_else(|| "main".to_string()),
            },
            _ => ProjectSource::Upload,
        };

        let project = Project {
            id: id.clone(),
            name: req.name,
            target: req.target,
            domain: req.domain,
            source,
            status: ProjectStatus::Created,
            created_at: now_iso(),
            last_deployed: None,
        };

        self.projects.write().await.push(project.clone());
        project
    }

    pub async fn delete(&self, id: &str) -> bool {
        let mut projects = self.projects.write().await;
        let len_before = projects.len();
        projects.retain(|p| p.id != id);
        projects.len() < len_before
    }
}

fn hex_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", ts)
}

fn now_iso() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Simple ISO-ish timestamp without pulling in chrono
    format!("{}Z", d)
}
