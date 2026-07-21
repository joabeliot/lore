use crate::models::*;
use serde::Serialize;
use tiny_http::{Header, Response, Server};

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

#[derive(Serialize)]
struct DashboardStats {
    total_projects: usize,
    total_tickets: usize,
    total_done: usize,
    total_todo: usize,
    total_inprogress: usize,
    total_backlog: usize,
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
struct ApiError {
    error: String,
}

const HTML: &str = include_str!("dashboard.html");

pub fn cmd_dashboard(port: u16) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("Failed to start HTTP server on {}: {}", addr, e))?;

    println!("🔥 lore dashboard running at http://localhost:{}", port);
    
    // Try to open browser
    let url = format!("http://localhost:{}", port);
    #[cfg(target_os = "macos")]
    { let _ = std::process::Command::new("open").arg(&url).spawn(); }
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(&url).spawn(); }
    
    println!("Press Ctrl+C to stop");

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().as_str().to_string();
        
        let response = handle_request(&url, &method);
        
        let content_type = match response {
            ApiResponse::Html(_) => "text/html; charset=utf-8",
            ApiResponse::Json(_) => "application/json",
            ApiResponse::NotFound | ApiResponse::ServerError(_) => "application/json",
        };
        
        let (status_code, body_bytes) = match &response {
            ApiResponse::Html(h) => (200, h.as_bytes().to_vec()),
            ApiResponse::Json(json) => (200, json.as_bytes().to_vec()),
            ApiResponse::NotFound => (404, b"{\"error\":\"Not found\"}".to_vec()),
            ApiResponse::ServerError(e) => (500, format!("{{\"error\":\"{}\"}}", e.replace('"', "\\\"")).as_bytes().to_vec()),
        };
        
        let ct_header = Header::from_bytes("Content-Type", content_type).unwrap();
        let cors_header = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();
        
        let mut response = Response::from_string(
            String::from_utf8_lossy(&body_bytes).to_string()
        )
        .with_status_code(status_code)
        .with_header(ct_header)
        .with_header(cors_header);
        
        // Set Connection: close for simplicity
        if let Ok(h) = Header::from_bytes("Connection", "close") {
            if let Some(ref mut r) = Option::Some(&mut response) {
                // Can't add more headers easily with tiny_http
                // but the content length header works fine
            }
            let _ = h;
        }
        
        let _ = request.respond(response);
    }

    Ok(())
}

enum ApiResponse {
    Html(String),
    Json(String),
    NotFound,
    ServerError(String),
}

fn handle_request(url: &str, _method: &str) -> ApiResponse {
    match url {
        "/" | "/index.html" => {
            ApiResponse::Html(HTML.to_string())
        }
        "/api/projects" => {
            match build_projects_response() {
                Ok(json) => ApiResponse::Json(json),
                Err(e) => ApiResponse::ServerError(e.to_string()),
            }
        }
        _ if url.starts_with("/api/tickets") => {
            // Parse ?session=<uid>
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
                None => return ApiResponse::Json(
                    r#"{"error":"Missing session parameter"}"#.to_string()
                ),
            };
            
            match build_tickets_response(&uid) {
                Ok(json) => ApiResponse::Json(json),
                Err(e) => ApiResponse::ServerError(e.to_string()),
            }
        }
        _ => ApiResponse::NotFound,
    }
}

fn build_projects_response() -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;
    let mut projects: Vec<ProjectSummary> = Vec::new();
    let mut total_tickets = 0usize;
    let mut total_done = 0usize;
    let mut total_todo = 0usize;
    let mut total_inprogress = 0usize;
    let mut total_backlog = 0usize;

    for (_uid, session) in &sessions {
        let config_content = match std::fs::read_to_string(&session.config_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let config: Config = match serde_yaml::from_str(&config_content) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;
        
        let mut backlog = 0usize;
        let mut todo = 0usize;
        let mut inprogress = 0usize;
        let mut done = 0usize;
        
        for t in &ticket_file.tickets {
            match t.status {
                TicketStatus::Backlog => backlog += 1,
                TicketStatus::Todo => todo += 1,
                TicketStatus::Inprogress => inprogress += 1,
                TicketStatus::Done => done += 1,
            }
        }
        
        let total = backlog + todo + inprogress + done;
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

    // Sort by created_at (oldest first)
    projects.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    let resp = ApiProjectsResponse {
        stats: DashboardStats {
            total_projects: projects.len(),
            total_tickets,
            total_done,
            total_todo,
            total_inprogress,
            total_backlog,
        },
        projects,
    };

    Ok(serde_json::to_string_pretty(&resp)?)
}

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
        backlog: ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Backlog).count(),
        todo: ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Todo).count(),
        inprogress: ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Inprogress).count(),
        done: ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Done).count(),
    };
    
    let tickets: Vec<TicketSummary> = ticket_file.tickets.iter().map(|t| {
        TicketSummary {
            id: t.id.clone(),
            name: t.name.clone(),
            status: t.status.to_string(),
            priority: format!("{}", t.priority),
            tags: t.tags.clone(),
            created: t.created.clone(),
            completed_at: t.completed_at.clone(),
            assigned_to: t.assigned_to.clone(),
        }
    }).collect();
    
    let resp = ApiTicketsResponse { session: project, tickets };
    Ok(serde_json::to_string_pretty(&resp)?)
}
