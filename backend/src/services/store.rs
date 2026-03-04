use anyhow::{Context, Result};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::project::{CreateProjectRequest, Project, ProjectSource, ProjectStatus};

#[derive(Clone)]
pub struct ProjectStore {
    conn: Arc<Mutex<Connection>>,
}

impl ProjectStore {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path).context("Failed to open SQLite database")?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Self::migrate(&conn)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    fn migrate(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS projects (
                id          TEXT PRIMARY KEY,
                name        TEXT NOT NULL,
                target      TEXT NOT NULL,
                domain      TEXT,
                source_type TEXT NOT NULL DEFAULT 'upload',
                git_url     TEXT,
                git_branch  TEXT,
                status      TEXT NOT NULL DEFAULT 'created',
                created_at  TEXT NOT NULL,
                last_deployed TEXT
            );

            CREATE TABLE IF NOT EXISTS deployments (
                id          TEXT PRIMARY KEY,
                project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                target      TEXT NOT NULL,
                status      TEXT NOT NULL DEFAULT 'queued',
                content_hash TEXT,
                gateway_url TEXT,
                size_bytes  INTEGER,
                created_at  TEXT NOT NULL,
                completed_at TEXT
            );",
        )
        .context("Failed to run migrations")?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Project>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, target, domain, source_type, git_url, git_branch, status, created_at, last_deployed FROM projects ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(row_to_project(row))
        })?;
        let mut projects = Vec::new();
        for row in rows {
            projects.push(row?);
        }
        Ok(projects)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Project>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, target, domain, source_type, git_url, git_branch, status, created_at, last_deployed FROM projects WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map([id], |row| {
            Ok(row_to_project(row))
        })?;
        Ok(rows.next().transpose()?)
    }

    pub async fn create(&self, req: CreateProjectRequest) -> Result<Project> {
        let id = format!("proj_{}", hex_id());
        let created_at = now_iso();

        let (source_type, git_url, git_branch) = match (&req.git_url, &req.git_branch) {
            (Some(url), branch) => (
                "git_repo",
                Some(url.clone()),
                Some(branch.clone().unwrap_or_else(|| "main".to_string())),
            ),
            _ => ("upload", None, None),
        };

        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO projects (id, name, target, domain, source_type, git_url, git_branch, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'created', ?8)",
            rusqlite::params![id, req.name, req.target, req.domain, source_type, git_url, git_branch, created_at],
        )?;

        let source = match (git_url, git_branch) {
            (Some(url), Some(branch)) => ProjectSource::GitRepo { url, branch },
            _ => ProjectSource::Upload,
        };

        Ok(Project {
            id,
            name: req.name,
            target: req.target,
            domain: req.domain,
            source,
            status: ProjectStatus::Created,
            created_at,
            last_deployed: None,
        })
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let conn = self.conn.lock().await;
        let affected = conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
        Ok(affected > 0)
    }
}

fn row_to_project(row: &rusqlite::Row) -> Project {
    let source_type: String = row.get(4).unwrap_or_default();
    let git_url: Option<String> = row.get(5).unwrap_or(None);
    let git_branch: Option<String> = row.get(6).unwrap_or(None);
    let status_str: String = row.get(7).unwrap_or_default();

    let source = match source_type.as_str() {
        "git_repo" => ProjectSource::GitRepo {
            url: git_url.unwrap_or_default(),
            branch: git_branch.unwrap_or_else(|| "main".to_string()),
        },
        _ => ProjectSource::Upload,
    };

    let status = match status_str.as_str() {
        "deploying" => ProjectStatus::Deploying,
        "live" => ProjectStatus::Live,
        "failed" => ProjectStatus::Failed,
        _ => ProjectStatus::Created,
    };

    Project {
        id: row.get(0).unwrap_or_default(),
        name: row.get(1).unwrap_or_default(),
        target: row.get(2).unwrap_or_default(),
        domain: row.get(3).unwrap_or(None),
        source,
        status,
        created_at: row.get(8).unwrap_or_default(),
        last_deployed: row.get(9).unwrap_or(None),
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
    format!("{}Z", d)
}
