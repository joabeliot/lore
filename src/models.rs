use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Session ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub uid: String,
    pub project: String,
    pub shorthand: String,
    pub description: String,
    pub working_dir: String,
    pub lore_dir: String,
    pub config_file: String,
    pub created_at: NaiveDate,
}

// ─── Config ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: String,
    pub session: String,
    pub description: String,
    pub working_dir: String,
    pub lore_dir: String,
    pub ticket: String,
}

// ─── Ticket ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub context: Vec<String>,
    #[serde(default)]
    pub plan: Option<String>,
    pub status: TicketStatus,
    pub priority: Priority,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub assigned_to: Option<String>,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub logs: Vec<TicketLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketLog {
    pub at: String,
    pub event: String,
    #[serde(default)]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TicketStatus {
    Backlog,
    Todo,
    Inprogress,
    Done,
}

impl std::fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketStatus::Backlog => write!(f, "backlog"),
            TicketStatus::Todo => write!(f, "todo"),
            TicketStatus::Inprogress => write!(f, "inprogress"),
            TicketStatus::Done => write!(f, "done"),
        }
    }
}

impl std::str::FromStr for TicketStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "backlog" => Ok(TicketStatus::Backlog),
            "todo" => Ok(TicketStatus::Todo),
            "inprogress" | "in_progress" | "in-progress" => Ok(TicketStatus::Inprogress),
            "done" => Ok(TicketStatus::Done),
            _ => Err(format!("Invalid status: {}. Expected: backlog, todo, inprogress, done", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    #[serde(rename = "P0")]
    P0,
    #[serde(rename = "P1")]
    P1,
    #[serde(rename = "P2")]
    P2,
    #[serde(rename = "P3")]
    P3,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::P0 => write!(f, "P0"),
            Priority::P1 => write!(f, "P1"),
            Priority::P2 => write!(f, "P2"),
            Priority::P3 => write!(f, "P3"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "P0" => Ok(Priority::P0),
            "P1" => Ok(Priority::P1),
            "P2" => Ok(Priority::P2),
            "P3" => Ok(Priority::P3),
            _ => Err(format!("Invalid priority: {}. Expected: P0, P1, P2, P3", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketFile {
    pub shorthand: String,
    pub tickets: Vec<Ticket>,
    pub counter: u32,
}

// ─── LoreDir (the structure inside a project's lore/) ───────────────────────

#[derive(Debug, Clone)]
pub struct LoreDir {
    pub path: String,
}

impl LoreDir {
    pub fn new(base: &str) -> Self {
        Self {
            path: base.to_string(),
        }
    }

    pub fn config_path(&self) -> String {
        format!("{}/config.yml", self.path)
    }

    pub fn workspace_path(&self) -> String {
        format!("{}/workspace", self.path)
    }

    pub fn ticket_path(&self) -> String {
        format!("{}/workspace/ticket.json", self.path)
    }

    /// Create the lore directory structure
    pub fn create_structure(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.path)?;
        std::fs::create_dir_all(self.workspace_path())?;
        Ok(())
    }
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Session cache: maps prefix -> Session
pub type SessionMap = HashMap<String, Session>;

/// Load all sessions from ~/.lore/sessions/
pub fn load_all_sessions() -> anyhow::Result<SessionMap> {
    let sessions_dir = format!("{}/.lore/sessions", std::env::var("HOME").unwrap_or_default());
    let mut map = HashMap::new();

    let dir = match std::fs::read_dir(&sessions_dir) {
        Ok(d) => d,
        Err(_) => return Ok(map),
    };

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "yml") {
            let content = std::fs::read_to_string(&path)?;
            if let Ok(session) = serde_yaml::from_str::<Session>(&content) {
                map.insert(session.uid.clone(), session);
            }
        }
    }
    Ok(map)
}

/// Find a session by UUID or prefix. Returns the matching session.
pub fn find_session(sessions: &SessionMap, input: &str) -> anyhow::Result<String> {
    // Direct match
    if sessions.contains_key(input) {
        return Ok(input.to_string());
    }

    // Prefix match
    let matches: Vec<&String> = sessions.keys().filter(|k| k.starts_with(input)).collect();

    if matches.len() == 1 {
        return Ok(matches[0].clone());
    } else if matches.len() > 1 {
        let mut msg = format!("Multiple sessions match prefix '{}':", input);
        for m in &matches {
            let s = &sessions[*m];
            msg.push_str(&format!("\n  {}  {} ({})", m, s.project, s.description));
        }
        anyhow::bail!(msg);
    }

    anyhow::bail!("No session found matching '{}'", input)
}

/// Find lore/config.yml by walking up from cwd
pub fn find_config_from_cwd() -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    let mut dir = cwd.as_path();

    loop {
        let config_path = dir.join("lore").join("config.yml");
        if config_path.exists() {
            return Some(config_path.to_string_lossy().to_string());
        }
        match dir.parent() {
            Some(p) => dir = p,
            None => return None,
        }
    }
}

/// Read config file and get session UUID
pub fn read_config_session(config_path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config.session)
}

/// Load ticket file, creating default if missing
pub fn load_ticket_file(ticket_path: &str, shorthand: &str) -> anyhow::Result<TicketFile> {
    match std::fs::read_to_string(ticket_path) {
        Ok(content) => {
            let tf: TicketFile = serde_json::from_str(&content)?;
            Ok(tf)
        }
        Err(_) => {
            let tf = TicketFile {
                shorthand: shorthand.to_string(),
                tickets: Vec::new(),
                counter: 1,
            };
            Ok(tf)
        }
    }
}
