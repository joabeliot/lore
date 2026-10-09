use crate::models::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_http::{Header, Response, Server};

// ─── Response types ──────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct ProjectSummary {
    uid: String,
    project: String,
    shorthand: String,
    description: String,
    working_dir: String,
    created_at: String,
    total: usize,
    backlog: usize,
    todo: usize,
    inprogress: usize,
    done: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DashboardStats {
    total_projects: usize,
    total_tickets: usize,
    total_done: usize,
    total_todo: usize,
    total_inprogress: usize,
    total_backlog: usize,
    done_this_week: usize,
    active_sessions: usize,
    healthy_projects: usize,
    stale_projects: usize,
    critical_projects: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectHealthSummary {
    project: String,
    shorthand: String,
    description: String,
    health: String, // "green" | "yellow" | "red"
    health_reason: String,
    total: usize,
    backlog: usize,
    todo: usize,
    inprogress: usize,
    done: usize,
    latest_session: Option<String>,
    stale_days: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OverviewResponse {
    projects: Vec<ProjectHealthSummary>,
    stats: DashboardStats,
}

#[derive(Serialize)]
struct TicketSummary {
    id: String,
    name: String,
    status: String,
    priority: String,
    tags: Vec<String>,
    created: String,
    completed_at: Option<String>,
    assigned_to: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiProjectsResponse {
    projects: Vec<ProjectSummary>,
    stats: DashboardStats,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiTicketsResponse {
    session: ProjectSummary,
    tickets: Vec<TicketSummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDetailResponse {
    project: String,
    shorthand: String,
    description: String,
    health: String,
    health_reason: String,
    tickets: Vec<TicketSummary>,
    sessions: Vec<SessionSummary>,
    stats: ProjectTicketStats,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectTicketStats {
    total: usize,
    backlog: usize,
    todo: usize,
    inprogress: usize,
    done: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionSummary {
    id: String,
    project: String,
    description: String,
    created_at: String,
    logs_count: usize,
}

#[derive(Serialize)]
struct ActivityEvent {
    timestamp: String,
    kind: String,
    project: String,
    shorthand: String,
    ticket_id: Option<String>,
    message: String,
    agent: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ActivityResponse {
    events: Vec<ActivityEvent>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WeeklyEntry {
    week_start: String,
    created: usize,
    completed: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StatsResponse {
    completed_this_week: usize,
    created_this_week: usize,
    active_sessions: usize,
    projects_by_health: serde_json::Value,
    total_sessions: usize,
    total_tickets: usize,
    weekly_history: Vec<WeeklyEntry>,
}

#[derive(Serialize)]
struct ApiError {
    error: String,
}

#[derive(Serialize)]
struct ApiSessionsResponse(Vec<SessionSummary>);

#[derive(Serialize)]
struct ApiNamesResponse(Vec<String>);

const HTML: &str = include_str!("dashboard.html");

// ─── Main entry point ───────────────────────────────────────────────────────

pub fn cmd_dashboard(port: u16) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("Failed to start HTTP server on {}: {}", addr, e))?;

    println!("🔥 lore dashboard running at http://localhost:{}", port);

    // Try to open browser
    let url = format!("http://localhost:{}", port);
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
    }

    println!("Press Ctrl+C to stop");

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().as_str().to_string();

        let response = handle_request(&url, &method);

        let content_type = match response {
            ApiResponse::Html(_) => "text/html; charset=utf-8",
            ApiResponse::Json(_) => "application/json",
            ApiResponse::NotFound | ApiResponse::ServerError(_) => "application/json",
            ApiResponse::Redirect(_) => "text/plain",
        };

        let (status_code, body_bytes) = match &response {
            ApiResponse::Html(h) => (200, h.as_bytes().to_vec()),
            ApiResponse::Json(json) => (200, json.as_bytes().to_vec()),
            ApiResponse::NotFound => (404, b"{\"error\":\"Not found\"}".to_vec()),
            ApiResponse::ServerError(e) => (
                500,
                format!("{{\"error\":\"{}\"}}", e.replace('"', "\\\""))
                    .as_bytes()
                    .to_vec(),
            ),
            ApiResponse::Redirect(loc) => {
                let body = format!("{{\"redirect\":\"{}\"}}", loc);
                (301, body.as_bytes().to_vec())
            }
        };

        let ct_header = Header::from_bytes("Content-Type", content_type).unwrap();
        let cors_header = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();

        let mut resp = Response::from_string(String::from_utf8_lossy(&body_bytes).to_string())
            .with_status_code(status_code)
            .with_header(ct_header)
            .with_header(cors_header);

        if let ApiResponse::Redirect(loc) = &response {
            if let Ok(h) = Header::from_bytes("Location", loc.as_str()) {
                resp = resp.with_header(h);
            }
        }

        let _ = request.respond(resp);
    }

    Ok(())
}

// ─── Routing ─────────────────────────────────────────────────────────────────

enum ApiResponse {
    Html(String),
    Json(String),
    NotFound,
    ServerError(String),
    Redirect(String),
}

fn handle_request(url: &str, _method: &str) -> ApiResponse {
    match url {
        "/" | "/index.html" => ApiResponse::Html(HTML.to_string()),
        "/api/overview" => json_or_error(build_overview_response()),
        "/api/projects" => json_or_error(build_projects_response()),
        "/api/stats" => json_or_error(build_stats_response()),
        "/api/activity" => {
            let events = build_activity_response(50);
            json_or_error(events)
        }
        _ if url.starts_with("/api/tickets") => {
            let uid = match url.split('?').nth(1).and_then(|q| {
                q.split('&').find_map(|pair| {
                    let mut kv = pair.splitn(2, '=');
                    match (kv.next(), kv.next()) {
                        (Some("session"), Some(val)) => Some(val.to_string()),
                        _ => None,
                    }
                })
            }) {
                Some(u) => u,
                None => {
                    return ApiResponse::Json(
                        r#"{"error":"Missing session parameter"}"#.to_string(),
                    )
                }
            };
            json_or_error(build_tickets_response(&uid))
        }
        _ if url.starts_with("/api/projects/") && url.ends_with("/sessions") => {
            let shorthand = url
                .trim_start_matches("/api/projects/")
                .trim_end_matches("/sessions");
            json_or_error(build_project_sessions_response(shorthand))
        }
        _ if url.starts_with("/api/projects/") && url.ends_with("/tickets") => {
            let shorthand = url
                .trim_start_matches("/api/projects/")
                .trim_end_matches("/tickets");
            json_or_error(build_project_tickets_response(shorthand))
        }
        _ if url.starts_with("/api/projects/") => {
            let shorthand = url.trim_start_matches("/api/projects/");
            json_or_error(build_project_detail_response(shorthand))
        }
        _ if url == "/api/project_names" => json_or_error(build_project_names_response()),
        _ => ApiResponse::NotFound,
    }
}

fn json_or_error(result: anyhow::Result<String>) -> ApiResponse {
    match result {
        Ok(json) => ApiResponse::Json(json),
        Err(e) => ApiResponse::ServerError(e.to_string()),
    }
}

// ─── Build project data (load from session configs) ──────────────────────────

fn load_all_projects() -> anyhow::Result<Vec<(String, Config)>> {
    let sessions = load_all_sessions()?;
    // Deduplicate by project config path
    let mut seen = std::collections::HashSet::new();
    let mut projects = Vec::new();

    for (_uid, session) in &sessions {
        if !seen.insert(&session.config_file) {
            continue;
        }
        let config_content = match std::fs::read_to_string(&session.config_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let config: Config = match serde_yaml::from_str(&config_content) {
            Ok(c) => c,
            Err(_) => continue,
        };
        projects.push((session.project.clone(), config));
    }

    Ok(projects)
}

fn get_ticket_counts(ticket_file: &TicketFile) -> (usize, usize, usize, usize, usize) {
    let mut backlog = 0;
    let mut todo = 0;
    let mut inprogress = 0;
    let mut done = 0;
    for t in &ticket_file.tickets {
        match t.status {
            TicketStatus::Backlog => backlog += 1,
            TicketStatus::Todo => todo += 1,
            TicketStatus::Inprogress => inprogress += 1,
            TicketStatus::Done => done += 1,
        }
    }
    let total = backlog + todo + inprogress + done;
    (total, backlog, todo, inprogress, done)
}

fn now_iso() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Rough ISO-ish date
    let days = now / 86400;
    let year = 1970 + (days as f64 / 365.25) as u64;
    // Just return rfc3339 from chrono
    chrono::Utc::now().to_rfc3339()
}

fn days_ago(iso: &Option<String>) -> Option<u64> {
    let date_str = iso.as_ref()?;
    // Parse ISO date: try rfc3339 first, then simple date
    let instant = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        dt
    } else if let Ok(naive) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        naive.and_hms_opt(0, 0, 0).unwrap().and_utc().fixed_offset()
    } else {
        return None;
    };

    let now = chrono::Utc::now();
    let dur = now.signed_duration_since(instant);
    Some(dur.num_days().max(0) as u64)
}

// ─── Build /api/overview ─────────────────────────────────────────────────────

fn build_overview_response() -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;
    let mut projects_data: HashMap<String, ProjectHealthData> = HashMap::new();

    for (_uid, session) in &sessions {
        let entry = projects_data
            .entry(session.project.clone())
            .or_insert_with(|| ProjectHealthData {
                config_file: session.config_file.clone(),
                project: session.project.clone(),
                shorthand: session.shorthand.clone(),
                description: String::new(),
                latest_session: None,
            });
        // Update latest session date if this one is newer
        let session_date = session.created_at.to_string();
        match &entry.latest_session {
            Some(existing) => {
                if session_date > existing.clone() {
                    entry.latest_session = Some(session_date.clone());
                }
            }
            None => {
                entry.latest_session = Some(session_date.clone());
            }
        }
    }

    let mut projects = Vec::new();
    let mut total_tickets = 0usize;
    let mut total_done = 0usize;
    let mut total_todo = 0usize;
    let mut total_inprogress = 0usize;
    let mut total_backlog = 0usize;
    let mut done_this_week = 0usize;
    let mut active_sessions_count = 0usize;
    let mut healthy = 0usize;
    let mut stale = 0usize;
    let mut critical = 0usize;

    let week_ago = chrono::Utc::now() - chrono::Duration::days(7);

    for (_name, data) in &projects_data {
        let config_content = match std::fs::read_to_string(&data.config_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let config: Config = match serde_yaml::from_str(&config_content) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ticket_file = match load_ticket_file(&config.ticket, &data.shorthand) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let (total, backlog, todo, inprogress, done) = get_ticket_counts(&ticket_file);
        total_tickets += total;
        total_done += done;
        total_todo += todo;
        total_inprogress += inprogress;
        total_backlog += backlog;

        // Health
        let stale_days = days_ago(&data.latest_session);
        let (health, health_reason) = match stale_days {
            None => ("red".to_string(), "No sessions yet".to_string()),
            Some(d) if d >= 14 => ("red".to_string(), format!("No activity for {} days", d)),
            Some(d) if d >= 7 => ("yellow".to_string(), format!("Last session {} days ago", d)),
            Some(d) => ("green".to_string(), format!("Active {} days ago", d)),
        };

        match health.as_str() {
            "green" => healthy += 1,
            "yellow" => stale += 1,
            "red" => critical += 1,
            _ => {}
        }

        // Done this week
        for t in &ticket_file.tickets {
            if let Some(ref completed_at) = t.completed_at {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(completed_at) {
                    if dt > week_ago {
                        done_this_week += 1;
                    }
                }
            }
        }

        let latest_session = data.latest_session.clone();
        let stale_days_val = stale_days;

        projects.push(ProjectHealthSummary {
            project: data.project.clone(),
            shorthand: data.shorthand.clone(),
            description: data.description.clone(),
            health,
            health_reason,
            total,
            backlog,
            todo,
            inprogress,
            done,
            latest_session,
            stale_days: stale_days_val,
        });
    }

    // Count active sessions from session files
    for (_uid, session) in &sessions {
        // Session files with working_dir that exist are "active" if recently created
        active_sessions_count += 1;
    }

    let resp = OverviewResponse {
        projects,
        stats: DashboardStats {
            total_projects: projects_data.len(),
            total_tickets,
            total_done,
            total_todo,
            total_inprogress,
            total_backlog,
            done_this_week,
            active_sessions: active_sessions_count,
            healthy_projects: healthy,
            stale_projects: stale,
            critical_projects: critical,
        },
    };

    Ok(serde_json::to_string_pretty(&resp)?)
}

struct ProjectHealthData {
    config_file: String,
    project: String,
    shorthand: String,
    description: String,
    latest_session: Option<String>,
}

// ─── Build /api/projects (v1 compat) ─────────────────────────────────────────

fn build_projects_response() -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;
    let mut projects: Vec<ProjectSummary> = Vec::new();
    let mut total_tickets = 0usize;
    let mut total_done = 0usize;
    let mut total_todo = 0usize;
    let mut total_inprogress = 0usize;
    let mut total_backlog = 0usize;

    // Deduplicate by config path
    let mut seen = std::collections::HashSet::new();

    for (_uid, session) in &sessions {
        if !seen.insert(&session.config_file) {
            continue;
        }
        let config_content = match std::fs::read_to_string(&session.config_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let config: Config = match serde_yaml::from_str(&config_content) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ticket_file = match load_ticket_file(&config.ticket, &session.shorthand) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let (total, backlog, todo, inprogress, done) = get_ticket_counts(&ticket_file);
        total_tickets += total;
        total_backlog += backlog;
        total_todo += todo;
        total_inprogress += inprogress;
        total_done += done;

        projects.push(ProjectSummary {
            uid: session.uid.clone(),
            project: session.project.clone(),
            shorthand: session.shorthand.clone(),
            description: session.description.clone(),
            working_dir: session.working_dir.clone(),
            created_at: session.created_at.to_string(),
            total,
            backlog,
            todo,
            inprogress,
            done,
        });
    }

    projects.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    let resp = ApiProjectsResponse {
        stats: DashboardStats {
            total_projects: projects.len(),
            total_tickets,
            total_done,
            total_todo,
            total_inprogress,
            total_backlog,
            done_this_week: 0,
            active_sessions: 0,
            healthy_projects: 0,
            stale_projects: 0,
            critical_projects: 0,
        },
        projects,
    };

    Ok(serde_json::to_string_pretty(&resp)?)
}

// ─── Build /api/tickets (v1 compat) ──────────────────────────────────────────

fn build_tickets_response(uid: &str) -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;
    let resolved_uid = find_session(&sessions, uid)?;
    let session = &sessions[&resolved_uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let project = ProjectSummary {
        uid: session.uid.clone(),
        project: session.project.clone(),
        shorthand: session.shorthand.clone(),
        description: session.description.clone(),
        working_dir: session.working_dir.clone(),
        created_at: session.created_at.to_string(),
        total: ticket_file.tickets.len(),
        backlog: ticket_file
            .tickets
            .iter()
            .filter(|t| t.status == TicketStatus::Backlog)
            .count(),
        todo: ticket_file
            .tickets
            .iter()
            .filter(|t| t.status == TicketStatus::Todo)
            .count(),
        inprogress: ticket_file
            .tickets
            .iter()
            .filter(|t| t.status == TicketStatus::Inprogress)
            .count(),
        done: ticket_file
            .tickets
            .iter()
            .filter(|t| t.status == TicketStatus::Done)
            .count(),
    };

    let tickets: Vec<TicketSummary> = ticket_file
        .tickets
        .iter()
        .map(|t| TicketSummary {
            id: t.id.clone(),
            name: t.name.clone(),
            status: t.status.to_string(),
            priority: format!("{}", t.priority),
            tags: t.tags.clone(),
            created: t.created.clone(),
            completed_at: t.completed_at.clone(),
            assigned_to: t.assigned_to.clone(),
        })
        .collect();

    let resp = ApiTicketsResponse {
        session: project,
        tickets,
    };
    Ok(serde_json::to_string_pretty(&resp)?)
}

// ─── Build /api/projects/<shorthand> ────────────────────────────────────────

fn find_shorthand_sessions<'a>(sessions: &'a SessionMap, shorthand: &str) -> Vec<(String, &'a Session)> {
    let mut matches = Vec::new();
    for (uid, session) in sessions {
        if session.shorthand.eq_ignore_ascii_case(shorthand) {
            matches.push((uid.clone(), session));
        }
    }
    matches
}

fn build_project_detail_response(shorthand: &str) -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;
    let project_sessions = find_shorthand_sessions(&sessions, shorthand);

    if project_sessions.is_empty() {
        anyhow::bail!("No project found with shorthand '{}'", shorthand);
    }

    // Use first session for config
    let (_uid, first_session) = &project_sessions[0];
    let config_content = std::fs::read_to_string(&first_session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file = load_ticket_file(&config.ticket, shorthand)?;

    let (total, backlog, todo, inprogress, done) = get_ticket_counts(&ticket_file);

    let latest_date = project_sessions
        .iter()
        .map(|(_, s)| s.created_at.to_string())
        .max()
        .unwrap_or_default();

    let stale_days = days_ago(&Some(latest_date.clone()));
    let (health, health_reason) = match stale_days {
        None => ("red".to_string(), "No sessions yet".to_string()),
        Some(d) if d >= 14 => ("red".to_string(), format!("No activity for {} days", d)),
        Some(d) if d >= 7 => ("yellow".to_string(), format!("Last session {} days ago", d)),
        Some(d) => ("green".to_string(), format!("Active {} days ago", d)),
    };

    let tickets: Vec<TicketSummary> = ticket_file
        .tickets
        .iter()
        .map(|t| TicketSummary {
            id: t.id.clone(),
            name: t.name.clone(),
            status: t.status.to_string(),
            priority: format!("{}", t.priority),
            tags: t.tags.clone(),
            created: t.created.clone(),
            completed_at: t.completed_at.clone(),
            assigned_to: t.assigned_to.clone(),
        })
        .collect();

    let sessions_summary: Vec<SessionSummary> = project_sessions
        .iter()
        .map(|(_, s)| SessionSummary {
            id: s.uid.clone(),
            project: s.project.clone(),
            description: s.description.clone(),
            created_at: s.created_at.to_string(),
            logs_count: 0, // Sessions don't have logs in current model
        })
        .collect();

    let resp = ProjectDetailResponse {
        project: first_session.project.clone(),
        shorthand: shorthand.to_string(),
        description: first_session.description.clone(),
        health,
        health_reason,
        tickets,
        sessions: sessions_summary,
        stats: ProjectTicketStats {
            total,
            backlog,
            todo,
            inprogress,
            done,
        },
    };

    Ok(serde_json::to_string_pretty(&resp)?)
}

// ─── Build /api/projects/<shorthand>/tickets ─────────────────────────────────

fn build_project_tickets_response(shorthand: &str) -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;
    let project_sessions = find_shorthand_sessions(&sessions, shorthand);

    if project_sessions.is_empty() {
        anyhow::bail!("No project found with shorthand '{}'", shorthand);
    }

    let (_uid, first_session) = &project_sessions[0];
    let config_content = std::fs::read_to_string(&first_session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file = load_ticket_file(&config.ticket, shorthand)?;

    let tickets: Vec<TicketSummary> = ticket_file
        .tickets
        .iter()
        .map(|t| TicketSummary {
            id: t.id.clone(),
            name: t.name.clone(),
            status: t.status.to_string(),
            priority: format!("{}", t.priority),
            tags: t.tags.clone(),
            created: t.created.clone(),
            completed_at: t.completed_at.clone(),
            assigned_to: t.assigned_to.clone(),
        })
        .collect();

    Ok(serde_json::to_string_pretty(&tickets)?)
}

// ─── Build /api/projects/<shorthand>/sessions ────────────────────────────────

fn build_project_sessions_response(shorthand: &str) -> anyhow::Result<String> {
    let all_sessions = load_all_sessions()?;
    let project_sessions = find_shorthand_sessions(&all_sessions, shorthand);

    let summary: Vec<SessionSummary> = project_sessions
        .iter()
        .map(|(_, s)| SessionSummary {
            id: s.uid.clone(),
            project: s.project.clone(),
            description: s.description.clone(),
            created_at: s.created_at.to_string(),
            logs_count: 0,
        })
        .collect();

    let resp = ApiSessionsResponse(summary);
    Ok(serde_json::to_string_pretty(&resp)?)
}

// ─── Build /api/activity ─────────────────────────────────────────────────────

fn build_activity_response(limit: usize) -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;
    let mut events: Vec<ActivityEvent> = Vec::new();

    // Collect events from sessions
    for (_uid, session) in &sessions {
        // Session creation event
        let created_at_str = session.created_at.format("%Y-%m-%d").to_string();
        events.push(ActivityEvent {
            timestamp: created_at_str.clone(),
            kind: "session_created".to_string(),
            project: session.project.clone(),
            shorthand: session.shorthand.clone(),
            ticket_id: None,
            message: format!("Session started: {}", session.description),
            agent: None,
        });

        // Try to read ticket.json for ticket events
        let config_content = match std::fs::read_to_string(&session.config_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let config: Config = match serde_yaml::from_str(&config_content) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ticket_file = match load_ticket_file(&config.ticket, &session.shorthand) {
            Ok(f) => f,
            Err(_) => continue,
        };

        for ticket in &ticket_file.tickets {
            // Ticket creation event
            if !ticket.created.is_empty() {
                events.push(ActivityEvent {
                    timestamp: ticket.created.clone(),
                    kind: "ticket_created".to_string(),
                    project: session.project.clone(),
                    shorthand: session.shorthand.clone(),
                    ticket_id: Some(ticket.id.clone()),
                    message: format!("Ticket {} created: {}", ticket.id, ticket.name),
                    agent: None,
                });
            }

            // Ticket completion event
            if let Some(ref completed_at) = ticket.completed_at {
                events.push(ActivityEvent {
                    timestamp: completed_at.clone(),
                    kind: "ticket_completed".to_string(),
                    project: session.project.clone(),
                    shorthand: session.shorthand.clone(),
                    ticket_id: Some(ticket.id.clone()),
                    message: format!("Ticket {} completed: {}", ticket.id, ticket.name),
                    agent: ticket.assigned_to.clone(),
                });
            }

            // Ticket logs as events
            for log in &ticket.logs {
                events.push(ActivityEvent {
                    timestamp: log.at.clone(),
                    kind: "ticket_event".to_string(),
                    project: session.project.clone(),
                    shorthand: session.shorthand.clone(),
                    ticket_id: Some(ticket.id.clone()),
                    message: log
                        .detail
                        .clone()
                        .unwrap_or_else(|| format!("Event: {}", log.event)),
                    agent: None,
                });
            }
        }
    }

    // Sort by timestamp descending (newest first)
    events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    events.truncate(limit);

    let resp = ActivityResponse { events };
    Ok(serde_json::to_string_pretty(&resp)?)
}

// ─── Build /api/stats ────────────────────────────────────────────────────────

fn build_stats_response() -> anyhow::Result<String> {
    let overview = build_overview_response()?;
    let overview_data: OverviewResponse = serde_json::from_str(&overview)?;

    let week_ago = chrono::Utc::now() - chrono::Duration::days(7);

    // Count created this week too
    let mut created_this_week = 0usize;
    let sessions = load_all_sessions()?;
    for (_uid, session) in &sessions {
        let config_content = match std::fs::read_to_string(&session.config_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let config: Config = match serde_yaml::from_str(&config_content) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ticket_file = match load_ticket_file(&config.ticket, &session.shorthand) {
            Ok(f) => f,
            Err(_) => continue,
        };
        for ticket in &ticket_file.tickets {
            if !ticket.created.is_empty() {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&ticket.created) {
                    if dt > week_ago {
                        created_this_week += 1;
                    }
                }
            }
        }
    }

    let projects_by_health = serde_json::json!({
        "green": overview_data.stats.healthy_projects,
        "yellow": overview_data.stats.stale_projects,
        "red": overview_data.stats.critical_projects,
    });

    // Weekly history — simple version
    let weekly_history = vec![WeeklyEntry {
        week_start: (chrono::Utc::now() - chrono::Duration::days(7))
            .format("%Y-%m-%d")
            .to_string(),
        created: created_this_week,
        completed: overview_data.stats.done_this_week,
    }];

    let resp = StatsResponse {
        completed_this_week: overview_data.stats.done_this_week,
        created_this_week,
        active_sessions: overview_data.stats.active_sessions,
        projects_by_health,
        total_sessions: overview_data.stats.active_sessions,
        total_tickets: overview_data.stats.total_tickets,
        weekly_history,
    };

    Ok(serde_json::to_string_pretty(&resp)?)
}

// ─── Build /api/project_names ────────────────────────────────────────────────

fn build_project_names_response() -> anyhow::Result<String> {
    let mut names: Vec<String> = load_all_projects()?
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    names.sort();
    let resp = ApiNamesResponse(names);
    Ok(serde_json::to_string_pretty(&resp)?)
}
