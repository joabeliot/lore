use crate::models::*;
use chrono::Local;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

// ─── Global data dir helpers ────────────────────────────────────────────────

fn lore_home() -> String {
    format!("{}/.lore", std::env::var("HOME").unwrap_or_default())
}

fn sessions_dir() -> String {
    format!("{}/.lore/sessions", std::env::var("HOME").unwrap_or_default())
}

fn ensure_global_dirs() -> anyhow::Result<()> {
    std::fs::create_dir_all(lore_home())?;
    std::fs::create_dir_all(sessions_dir())?;
    Ok(())
}

// ─── Helper: make a log entry ──────────────────────────────────────────────

fn make_log(event: &str, detail: Option<&str>) -> TicketLog {
    TicketLog {
        at: chrono::Utc::now().to_rfc3339(),
        event: event.to_string(),
        detail: detail.map(|s| s.to_string()),
    }
}

// ─── Resolve session: try --session, then cwd config ────────────────────────

pub fn resolve_session(session_arg: Option<&str>) -> anyhow::Result<String> {
    let sessions = load_all_sessions()?;

    match session_arg {
        Some(input) => find_session(&sessions, input),
        None => {
            // Try cwd/parent config
            match find_config_from_cwd() {
                Some(config_path) => {
                    let uid = read_config_session(&config_path)?;
                    find_session(&sessions, &uid)
                }
                None => {
                    anyhow::bail!(
                        "No session specified. Use --session <UUID|prefix> or run from a project directory with lore/config.yml"
                    )
                }
            }
        }
    }
}

// ─── Save ticket file helper ───────────────────────────────────────────────

fn save_ticket_file(config: &Config, ticket_file: &TicketFile) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(ticket_file)?;
    std::fs::write(&config.ticket, &json)?;
    Ok(())
}

// ─── Parse comma-separated list ────────────────────────────────────────────

fn parse_csv(s: Option<&str>) -> Vec<String> {
    match s {
        Some(v) if !v.is_empty() => v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        _ => Vec::new(),
    }
}

// ─── create project ─────────────────────────────────────────────────────────

pub fn cmd_create_project(
    unzip: Option<String>,
    name: &str,
    description: &str,
    wrk_dir: &str,
    shorthand: &str,
) -> anyhow::Result<()> {
    ensure_global_dirs()?;

    let lore_dir_path = format!("{}/lore", wrk_dir.trim_end_matches('/'));
    let lore_dir = LoreDir::new(&lore_dir_path);

    if let Some(zip_path) = unzip {
        // Unzip an existing project structure
        lore_dir.create_structure()?;
        let status = std::process::Command::new("unzip")
            .arg("-o")
            .arg(&zip_path)
            .arg("-d")
            .arg(&lore_dir.workspace_path())
            .status();
        match status {
            Ok(s) if s.success() => {}
            Ok(s) => anyhow::bail!("unzip failed with exit code: {}", s),
            Err(e) => anyhow::bail!("Failed to run unzip: {}", e),
        }
    } else {
        lore_dir.create_structure()?;
    }

    let uid = uuid::Uuid::new_v4().to_string();
    let today = Local::now().date_naive();

    let config = Config {
        project: name.to_string(),
        session: uid.clone(),
        description: description.to_string(),
        working_dir: wrk_dir.to_string(),
        lore_dir: lore_dir_path.clone(),
        ticket: lore_dir.ticket_path(),
    };

    let config_yaml = serde_yaml::to_string(&config)?;
    std::fs::write(lore_dir.config_path(), &config_yaml)?;

    let session = Session {
        uid: uid.clone(),
        project: name.to_string(),
        shorthand: shorthand.to_string(),
        description: description.to_string(),
        working_dir: wrk_dir.to_string(),
        lore_dir: lore_dir_path.clone(),
        config_file: lore_dir.config_path(),
        created_at: today,
    };

    let session_yaml = serde_yaml::to_string(&session)?;
    let session_file = format!("{}/{}.yml", sessions_dir(), uid);
    std::fs::write(&session_file, &session_yaml)?;

    // Create empty ticket.json
    let ticket_file = TicketFile {
        shorthand: shorthand.to_string(),
        tickets: Vec::new(),
        counter: 1,
    };
    let ticket_json = serde_json::to_string_pretty(&ticket_file)?;
    std::fs::write(lore_dir.ticket_path(), &ticket_json)?;

    println!("Created project '{}' in {}", name, wrk_dir);
    println!("Session UUID: {}", uid);
    println!("Shorthand: {}", shorthand);

    Ok(())
}

// ─── recall ─────────────────────────────────────────────────────────────────

pub fn cmd_recall(input: &str, json_output: bool) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;
    let uid = find_session(&sessions, input)?;
    let session = &sessions[&uid];

    if json_output {
        println!("{}", serde_json::to_string_pretty(session)?);
    } else {
        println!("Session: {}", uid);
        println!("  Project:     {}", session.project);
        println!("  Shorthand:   {}", session.shorthand);
        println!("  Description: {}", session.description);
        println!("  Working Dir: {}", session.working_dir);
        println!("  Lore Dir:    {}", session.lore_dir);
        println!("  Config:      {}", session.config_file);
        println!("  Created:     {}", session.created_at);
    }
    Ok(())
}

// ─── list projects ──────────────────────────────────────────────────────────

pub fn cmd_list_projects() -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;

    if sessions.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    println!("Projects:");
    let mut sorted: Vec<_> = sessions.iter().collect();
    sorted.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));

    for (uid, session) in &sorted {
        println!(
            "  {}  {} ({})  [{}]",
            uid, session.project, session.description, session.shorthand
        );
    }
    Ok(())
}

// ─── delete project ─────────────────────────────────────────────────────────

pub fn cmd_delete_project(input: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;
    let uid = find_session(&sessions, input)?;
    let session = &sessions[&uid];

    // Remove session file
    let session_file = format!("{}/{}.yml", sessions_dir(), uid);
    std::fs::remove_file(&session_file)?;

    // Remove project lore dir
    if Path::new(&session.lore_dir).exists() {
        std::fs::remove_dir_all(&session.lore_dir)?;
    }

    println!("Deleted project '{}' ({})", session.project, uid);
    Ok(())
}

// ─── ticket add ─────────────────────────────────────────────────────────────

pub fn cmd_ticket_add(
    session_arg: Option<&str>,
    name: &str,
    description: Option<&str>,
    status: Option<&str>,
    source: Option<&str>,
    priority: Option<&str>,
    tags: Option<&str>,
    context: Option<&str>,
) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let uid = resolve_session(session_arg)?;
    let sessions = load_all_sessions()?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file_path = &config.ticket;

    let shorthand = &session.shorthand;
    let mut ticket_file = load_ticket_file(ticket_file_path, shorthand)?;

    let ticket_id = format!("{}-{}", shorthand, ticket_file.counter);
    let today = Local::now().date_naive().to_string();

    let status_val: TicketStatus = match status {
        Some(s) => s.parse().map_err(|e: String| anyhow::anyhow!(e))?,
        None => TicketStatus::Backlog,
    };

    let priority_val: Priority = match priority {
        Some(p) => p.parse().map_err(|e: String| anyhow::anyhow!(e))?,
        None => Priority::P2,
    };

    let tags_vec = parse_csv(tags);
    let context_vec = parse_csv(context);

    let log_entry = make_log("created", Some(&ticket_id));

    let ticket = Ticket {
        id: ticket_id.clone(),
        name: name.to_string(),
        description: description.unwrap_or("").to_string(),
        context: context_vec,
        plan: None,
        status: status_val,
        priority: priority_val,
        tags: tags_vec,
        source: source.unwrap_or("Agent").to_string(),
        created: today,
        assigned_to: None,
        started_at: None,
        completed_at: None,
        logs: vec![log_entry],
    };

    ticket_file.tickets.push(ticket);
    ticket_file.counter += 1;

    save_ticket_file(&config, &ticket_file)?;

    println!("Created ticket {} in session {}", ticket_id, uid);
    Ok(())
}

// ─── ticket list ────────────────────────────────────────────────────────────

pub fn cmd_ticket_list(
    session_arg: Option<&str>,
    status_filter: Option<&str>,
    priority_filter: Option<&str>,
) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let uid = resolve_session(session_arg)?;
    let sessions = load_all_sessions()?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let mut ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    // Filter
    if let Some(status_str) = status_filter {
        let status_filter_val: TicketStatus =
            status_str.parse().map_err(|e: String| anyhow::anyhow!(e))?;
        ticket_file
            .tickets
            .retain(|t| t.status == status_filter_val);
    }
    if let Some(priority_str) = priority_filter {
        let priority_filter_val: Priority =
            priority_str.parse().map_err(|e: String| anyhow::anyhow!(e))?;
        ticket_file
            .tickets
            .retain(|t| t.priority == priority_filter_val);
    }

    if ticket_file.tickets.is_empty() {
        println!("No tickets found in session {}", uid);
        return Ok(());
    }

    println!("Tickets in session {} ({}):", uid, session.project);
    for t in &ticket_file.tickets {
        let assigned = t
            .assigned_to
            .as_deref()
            .unwrap_or("-");
        println!(
            "  {}  [{}] [{}] {}  (assigned: {})",
            t.id, t.status, t.priority, t.name, assigned
        );
    }
    Ok(())
}

// ─── ticket show ────────────────────────────────────────────────────────────

pub fn cmd_ticket_show(session_arg: Option<&str>, ticket_id: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let uid = resolve_session(session_arg)?;
    let sessions = load_all_sessions()?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let ticket = ticket_file
        .tickets
        .iter()
        .find(|t| t.id == ticket_id)
        .ok_or_else(|| anyhow::anyhow!("Ticket '{}' not found in session {}", ticket_id, uid))?;

    println!("Ticket: {}", ticket.id);
    println!("  Name:        {}", ticket.name);
    println!("  Description: {}", ticket.description);
    if !ticket.context.is_empty() {
        println!("  Context:     {}", ticket.context.join(", "));
    }
    println!("  Status:      {}", ticket.status);
    println!("  Priority:    {}", ticket.priority);
    println!("  Tags:        {}", ticket.tags.join(", "));
    println!("  Source:      {}", ticket.source);
    println!("  Created:     {}", ticket.created);
    if let Some(ref a) = ticket.assigned_to {
        println!("  Assigned To: {}", a);
    }
    if let Some(ref s) = ticket.started_at {
        println!("  Started At:  {}", s);
    }
    if let Some(ref c) = ticket.completed_at {
        println!("  Completed At: {}", c);
    }
    if !ticket.logs.is_empty() {
        println!("  Logs:");
        for log in &ticket.logs {
            let detail = log.detail.as_deref().unwrap_or("");
            println!("    [{}] {} {}", log.at, log.event, detail);
        }
    }

    Ok(())
}

// ─── ticket schedule ────────────────────────────────────────────────────────

pub fn cmd_ticket_schedule(session_arg: Option<&str>, ticket_id: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let uid = resolve_session(session_arg)?;
    let sessions = load_all_sessions()?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let mut ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let ticket = ticket_file
        .tickets
        .iter_mut()
        .find(|t| t.id == ticket_id)
        .ok_or_else(|| anyhow::anyhow!("Ticket '{}' not found in session {}", ticket_id, uid))?;

    ticket.status = TicketStatus::Todo;
    ticket.logs.push(make_log("scheduled", None));

    save_ticket_file(&config, &ticket_file)?;

    println!("Scheduled ticket {} (status -> todo)", ticket_id);
    Ok(())
}

// ─── ticket start ───────────────────────────────────────────────────────────

pub fn cmd_ticket_start(
    session_arg: Option<&str>,
    ticket_id: &str,
    agent: &str,
) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let uid = resolve_session(session_arg)?;
    let sessions = load_all_sessions()?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let mut ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let today = Local::now().date_naive().to_string();

    let ticket = ticket_file
        .tickets
        .iter_mut()
        .find(|t| t.id == ticket_id)
        .ok_or_else(|| anyhow::anyhow!("Ticket '{}' not found in session {}", ticket_id, uid))?;

    ticket.status = TicketStatus::Inprogress;
    ticket.assigned_to = Some(agent.to_string());
    ticket.started_at = Some(today.clone());
    ticket.logs.push(make_log("started", Some(agent)));

    save_ticket_file(&config, &ticket_file)?;

    println!(
        "Started ticket {} (assigned to {}, status -> inprogress)",
        ticket_id, agent
    );
    Ok(())
}

// ─── ticket done ────────────────────────────────────────────────────────────

pub fn cmd_ticket_done(session_arg: Option<&str>, ticket_id: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let uid = resolve_session(session_arg)?;
    let sessions = load_all_sessions()?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let mut ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let today = Local::now().date_naive().to_string();

    let ticket = ticket_file
        .tickets
        .iter_mut()
        .find(|t| t.id == ticket_id)
        .ok_or_else(|| anyhow::anyhow!("Ticket '{}' not found in session {}", ticket_id, uid))?;

    ticket.status = TicketStatus::Done;
    ticket.completed_at = Some(today);
    ticket.logs.push(make_log("completed", None));

    save_ticket_file(&config, &ticket_file)?;

    println!("Completed ticket {}", ticket_id);
    Ok(())
}

// ─── ticket edit ────────────────────────────────────────────────────────────

pub fn cmd_ticket_edit(
    session_arg: Option<&str>,
    ticket_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    priority: Option<&str>,
    tags: Option<&str>,
    status: Option<&str>,
    context: Option<&str>,
) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let uid = resolve_session(session_arg)?;
    let sessions = load_all_sessions()?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let mut ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let ticket = ticket_file
        .tickets
        .iter_mut()
        .find(|t| t.id == ticket_id)
        .ok_or_else(|| anyhow::anyhow!("Ticket '{}' not found in session {}", ticket_id, uid))?;

    if let Some(n) = name {
        ticket.name = n.to_string();
    }
    if let Some(d) = description {
        ticket.description = d.to_string();
    }
    if let Some(p) = priority {
        ticket.priority = p.parse().map_err(|e: String| anyhow::anyhow!(e))?;
    }
    if let Some(t) = tags {
        ticket.tags = t.split(',').map(|s| s.trim().to_string()).collect();
    }
    if let Some(s) = status {
        ticket.status = s.parse().map_err(|e: String| anyhow::anyhow!(e))?;
    }
    if let Some(c) = context {
        let new_context: Vec<String> = c.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        // Log each context file that is new
        for ctx in &new_context {
            if !ticket.context.contains(ctx) {
                ticket.logs.push(make_log("context_added", Some(ctx)));
            }
        }
        ticket.context = new_context;
    }

    save_ticket_file(&config, &ticket_file)?;

    println!("Updated ticket {}", ticket_id);
    Ok(())
}

// ─── session close ──────────────────────────────────────────────────────────

pub fn cmd_session_close(input: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;
    let uid = find_session(&sessions, input)?;
    let session = &sessions[&uid];

    let session_file = format!("{}/{}.yml", sessions_dir(), uid);
    std::fs::remove_file(&session_file)?;

    println!("Closed session {} ({})", uid, session.project);
    Ok(())
}

// ─── session status ─────────────────────────────────────────────────────────

pub fn cmd_session_status(input: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;
    let uid = find_session(&sessions, input)?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let total = ticket_file.tickets.len();
    let backlog = ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Backlog).count();
    let todo = ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Todo).count();
    let inprogress = ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Inprogress).count();
    let done = ticket_file.tickets.iter().filter(|t| t.status == TicketStatus::Done).count();

    println!("Session: {} ({})", uid, session.project);
    println!("  Description: {}", session.description);
    println!("  Shorthand:   {}", session.shorthand);
    println!("  Created:     {}", session.created_at);
    println!("  Tickets:");
    println!("    Total:      {}", total);
    println!("    Backlog:    {}", backlog);
    println!("    Todo:       {}", todo);
    println!("    In Progress: {}", inprogress);
    println!("    Done:       {}", done);

    Ok(())
}

// ─── inspect ────────────────────────────────────────────────────────────────

pub fn cmd_inspect(input: &str, ticket_id: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;
    let uid = find_session(&sessions, input)?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    let ticket = ticket_file
        .tickets
        .iter()
        .find(|t| t.id == ticket_id)
        .ok_or_else(|| anyhow::anyhow!("Ticket '{}' not found in session {}", ticket_id, uid))?;

    let lore_dir = &session.lore_dir;
    let working_dir = &session.working_dir;
    let mut all_ok = true;

    // Header
    println!("🔍 Inspecting {} in project '{}'", ticket.id, session.project);
    println!("   Working dir: {}", working_dir);
    println!();

    // ── Context files ──────────────────────────────────────────────────
    println!("📄 Context files:");
    if ticket.context.is_empty() {
        println!("   (no context files set)");
    } else {
        for ctx_path in &ticket.context {
            let full_path = format!("{}/{}", lore_dir, ctx_path);
            if Path::new(&full_path).exists() {
                println!("   ✅ {}", ctx_path);
            } else {
                println!("   ❌ {} (NOT FOUND)", ctx_path);
                all_ok = false;
            }
        }
    }
    println!();

    // ── Working directory exists? ──────────────────────────────────────
    let wrk_exists = Path::new(working_dir).exists();
    if !wrk_exists {
        println!("🏗️  Working directory:");
        println!("   ❌ Working directory does not exist: {}", working_dir);
        all_ok = false;
        println!();
    }

    // ── Git status ─────────────────────────────────────────────────────
    if wrk_exists {
        println!("📦 Git status:");
        let git_dir = format!("{}/.git", working_dir);
        if Path::new(&git_dir).exists() {
            println!("   ✅ Git repo detected");

            // Check for uncommitted changes
            let output = run_cmd(working_dir, "git", &["status", "--porcelain"], None);
            match output {
                Ok(out) => {
                    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
                    if !lines.is_empty() {
                        println!("   ⚠️  Uncommitted changes: {} file(s)", lines.len());
                    } else {
                        println!("   ✅ Working tree clean");
                    }
                }
                Err(e) => {
                    println!("   ❌ Failed to check git status: {}", e.trim());
                    all_ok = false;
                }
            }
        } else {
            println!("   ❌ Not a git repository (no .git directory found)");
            all_ok = false;
        }
        println!();

        // ── Build check ────────────────────────────────────────────────
        println!("🏗️  Build check:");
        let cargo_toml = format!("{}/Cargo.toml", working_dir);
        let package_json = format!("{}/package.json", working_dir);

        if Path::new(&cargo_toml).exists() {
            // Rust project
            // cargo build
            let build_result = run_cmd(working_dir, "cargo", &["build"], Some(120));
            match build_result {
                Ok(_out) => {
                    println!("   ✅ cargo build passed");
                    // cargo test
                    let test_result = run_cmd(working_dir, "cargo", &["test"], Some(120));
                    match test_result {
                        Ok(test_out) => {
                            // Extract test summary line
                            let summary = extract_test_summary(&test_out);
                            println!("   ✅ cargo test passed{}", summary);
                        }
                        Err(test_err) => {
                            println!("   ❌ cargo test failed:");
                            for line in test_err.lines().take(20) {
                                println!("      {}", line);
                            }
                            all_ok = false;
                        }
                    }
                }
                Err(build_err) => {
                    println!("   ❌ cargo build failed:");
                    for line in build_err.lines().take(20) {
                        println!("      {}", line);
                    }
                    all_ok = false;
                }
            }
        } else if Path::new(&package_json).exists() {
            // Node project
            let build_result = run_cmd(working_dir, "npm", &["run", "build"], Some(120));
            match build_result {
                Ok(_out) => {
                    println!("   ✅ npm run build passed");
                }
                Err(build_err) => {
                    println!("   ❌ npm run build failed:");
                    for line in build_err.lines().take(20) {
                        println!("      {}", line);
                    }
                    all_ok = false;
                }
            }

            // npm test
            let test_result = run_cmd(working_dir, "npm", &["test"], Some(120));
            match test_result {
                Ok(_) => {
                    println!("   ✅ npm test passed");
                }
                Err(test_err) => {
                    println!("   ❌ npm test failed:");
                    for line in test_err.lines().take(20) {
                        println!("      {}", line);
                    }
                    all_ok = false;
                }
            }
        } else {
            println!("   ℹ️  No build system detected, skipping compilation check.");
        }
        println!();
    }

    if all_ok {
        println!("✅ All checks passed.");
        Ok(())
    } else {
        println!("❌ Some checks failed.");
        std::process::exit(1);
    }
}

// ─── version ─────────────────────────────────────────────────────────────────

pub fn cmd_version() -> anyhow::Result<()> {
    println!("lore {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

// ─── session attach ──────────────────────────────────────────────────────────

pub fn cmd_session_attach(wrk_dir: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;

    let config_path = format!("{}/lore/config.yml", wrk_dir.trim_end_matches('/'));
    let config_file_path = Path::new(&config_path);

    if !config_file_path.exists() {
        eprintln!("Error: No lore/config.yml found in {}", wrk_dir);
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(config_file_path)?;
    let config: Config = serde_yaml::from_str(&content)?;

    let uid = uuid::Uuid::new_v4().to_string();
    let today = chrono::Local::now().date_naive();

    let wrk = wrk_dir.trim_end_matches('/');
    let lore_dir_path = format!("{}/lore", wrk);

    let session = Session {
        uid: uid.clone(),
        project: config.project.clone(),
        shorthand: "".to_string(), // Will be read from ticket.json shorthand
        description: config.description.clone(),
        working_dir: wrk.to_string(),
        lore_dir: lore_dir_path.clone(),
        config_file: config_path.clone(),
        created_at: today,
    };

    // Save session file
    let session_yaml = serde_yaml::to_string(&session)?;
    let session_file = format!("{}/{}.yml", sessions_dir(), uid);
    std::fs::create_dir_all(sessions_dir())?;
    std::fs::write(&session_file, &session_yaml)?;

    // Update config.yml's session field
    let mut updated_config = config;
    updated_config.session = uid.clone();
    let config_yaml = serde_yaml::to_string(&updated_config)?;
    std::fs::write(&config_path, &config_yaml)?;

    println!("Attached session {} to project '{}' in {}", uid, updated_config.project, wrk);
    Ok(())
}

// ─── session log ─────────────────────────────────────────────────────────────

pub fn cmd_session_log(input: &str, message: &str) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;
    let uid = find_session(&sessions, input)?;
    let session = &sessions[&uid];

    let config_content = std::fs::read_to_string(&session.config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    let ticket_file_path = &config.ticket;

    let mut ticket_file = load_ticket_file(ticket_file_path, &session.shorthand)?;

    if ticket_file.tickets.is_empty() {
        eprintln!("Warning: No tickets found in session {}. Log not recorded.", uid);
        return Ok(());
    }

    // Find active ticket: first "inprogress", else last one
    let target_idx = ticket_file
        .tickets
        .iter()
        .position(|t| t.status == TicketStatus::Inprogress)
        .unwrap_or(ticket_file.tickets.len() - 1);

    let log_entry = TicketLog {
        at: chrono::Utc::now().to_rfc3339(),
        event: "log".to_string(),
        detail: Some(message.to_string()),
    };

    ticket_file.tickets[target_idx].logs.push(log_entry);

    save_ticket_file(&config, &ticket_file)?;

    let target_ticket = &ticket_file.tickets[target_idx];
    println!("Logged to ticket {} in session {}", target_ticket.id, uid);
    Ok(())
}

// ─── edit project ────────────────────────────────────────────────────────────

pub fn cmd_edit_project(
    input: &str,
    name: Option<&str>,
    description: Option<&str>,
    shorthand: Option<&str>,
    wrk_dir: Option<&str>,
) -> anyhow::Result<()> {
    ensure_global_dirs()?;
    let sessions = load_all_sessions()?;
    let uid = find_session(&sessions, input)?;
    let session = &sessions[&uid];

    let mut changes: Vec<String> = Vec::new();

    // Read config.yml
    let config_content = std::fs::read_to_string(&session.config_file)?;
    let mut config: Config = serde_yaml::from_str(&config_content)?;

    // Load ticket file for shorthand rename
    let mut ticket_file = load_ticket_file(&config.ticket, &session.shorthand)?;

    // Track old shorthand for renaming
    let old_shorthand = session.shorthand.clone();

    // Update session YAML fields
    if let Some(n) = name {
        changes.push(format!("name: '{}' -> '{}'", session.project, n));
    }
    if let Some(d) = description {
        changes.push(format!("description: '{}' -> '{}'", session.description, d));
    }
    if let Some(s) = shorthand {
        changes.push(format!("shorthand: '{}' -> '{}'", session.shorthand, s));
    }
    if let Some(w) = wrk_dir {
        changes.push(format!("working_dir: '{}' -> '{}'", session.working_dir, w));
    }

    // 1. Read current session, apply changes
    let session_path = format!("{}/{}.yml", sessions_dir(), uid);
    let session_content = std::fs::read_to_string(&session_path)?;
    let mut sess: Session = serde_yaml::from_str(&session_content)?;

    if let Some(n) = name {
        sess.project = n.to_string();
        config.project = n.to_string();
    }
    if let Some(d) = description {
        sess.description = d.to_string();
        config.description = d.to_string();
    }
    if let Some(s) = shorthand {
        sess.shorthand = s.to_string();
        ticket_file.shorthand = s.to_string();
    }
    if let Some(w) = wrk_dir {
        let wrk = w.trim_end_matches('/');
        sess.working_dir = wrk.to_string();
        config.working_dir = wrk.to_string();
        let new_lore_dir = format!("{}/lore", wrk);
        sess.lore_dir = new_lore_dir.clone();
        config.lore_dir = new_lore_dir;
        let new_ticket_path = format!("{}/lore/workspace/ticket.json", wrk);
        config.ticket = new_ticket_path;
        let new_config_path = format!("{}/lore/config.yml", wrk);
        sess.config_file = new_config_path;
    }

    // If shorthand changed, rename all ticket IDs and the counter
    if let Some(new_shorthand) = shorthand {
        let old_prefix = &old_shorthand;
        let new_prefix = new_shorthand;

        for ticket in &mut ticket_file.tickets {
            // Replace old prefix in ticket ID, keep the number
            if ticket.id.starts_with(old_prefix) {
                let num_part = &ticket.id[old_prefix.len()..]; // e.g., "-1"
                ticket.id = format!("{}{}", new_prefix, num_part);
            }
        }
    }

    // Write updated session YAML
    let updated_session_yaml = serde_yaml::to_string(&sess)?;
    std::fs::write(&session_path, &updated_session_yaml)?;

    // Write updated config.yml
    let updated_config_yaml = serde_yaml::to_string(&config)?;
    std::fs::write(&sess.config_file, &updated_config_yaml)?;

    // Write updated ticket file (needed for shorthand and ID changes)
    save_ticket_file(&config, &ticket_file)?;

    println!("Updated project '{}':", sess.project);
    for c in &changes {
        println!("  - {}", c);
    }

    Ok(())
}

// ─── update ──────────────────────────────────────────────────────────────────

pub fn cmd_update() -> anyhow::Result<()> {
    let update_url = std::env::var("LORE_UPDATE_URL");

    match update_url {
        Ok(url) => {
            if url.is_empty() {
                eprintln!("Error: LORE_UPDATE_URL is set but empty.");
                std::process::exit(1);
            }

            let current_exe = std::env::current_exe()
                .map_err(|e| anyhow::anyhow!("Failed to determine current binary path: {}", e))?;

            println!("Downloading update from {} ...", url);

            // Use ureq to download
            let response = ureq::get(&url)
                .call()
                .map_err(|e| anyhow::anyhow!("Failed to download update: {}", e))?;

            let mut body: Vec<u8> = Vec::new();
            response
                .into_reader()
                .read_to_end(&mut body)
                .map_err(|e| anyhow::anyhow!("Failed to read downloaded data: {}", e))?;

            // Replace current binary
            std::fs::write(&current_exe, &body)
                .map_err(|e| anyhow::anyhow!("Failed to write new binary: {}", e))?;

            // Make executable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = std::fs::Permissions::from_mode(0o755);
                std::fs::set_permissions(&current_exe, perms)
                    .map_err(|e| anyhow::anyhow!("Failed to set executable permissions: {}", e))?;
            }

            println!("Successfully updated lore binary at {:?}", current_exe);
            Ok(())
        }
        Err(_) => {
            println!("Update not configured yet. Set LORE_UPDATE_URL env var or update source.");
            Ok(())
        }
    }
}

fn run_cmd(cwd: &str, program: &str, args: &[&str], timeout_secs: Option<u64>) -> Result<String, String> {
    let mut cmd = std::process::Command::new(program);
    cmd.args(args);
    cmd.current_dir(cwd);

    // Spawn with timeout
    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn {}: {}", program, e))?;

    let start = std::time::Instant::now();
    let max_wait = Duration::from_secs(timeout_secs.unwrap_or(120));

    // Poll for completion (non-blocking wait)
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let output = child.wait_with_output().map_err(|e| format!("Failed to read output: {}", e))?;
                let combined = String::from_utf8_lossy(&output.stdout).to_string()
                    + &String::from_utf8_lossy(&output.stderr).to_string();
                if status.success() {
                    return Ok(combined);
                } else {
                    return Err(combined);
                }
            }
            Ok(None) => {
                // Still running
                if start.elapsed() > max_wait {
                    // Kill and report timeout
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!("Command timed out after {}s: {} {:?}", max_wait.as_secs(), program, args));
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(format!("Error waiting for {}: {}", program, e));
            }
        }
    }
}

/// Extract test summary line from cargo test output (e.g., "test result: ok. 3 passed; 0 failed; ...")
fn extract_test_summary(output: &str) -> String {
    for line in output.lines() {
        if line.starts_with("test result:") {
            // Trim "test result: " prefix and any ";" artifacts
            let summary = line.trim().trim_start_matches("test result:").trim();
            return format!(" ({})", summary);
        }
    }
    String::new()
}
