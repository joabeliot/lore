use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use chrono::Utc;
use clap::{Parser, Subcommand};
use regex::Regex;

// ── Constants ──────────────────────────────────────────────────────────────

const KANBAN_DIR: &str = "lore/kanban";
const STATES: &[&str] = &["backlog", "todo", "inprogress", "done"];

// ── CLI ────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "lore", about = "The lore kanban CLI — manage lore project kanban boards")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Manage kanban board
    Kanban {
        #[command(subcommand)]
        cmd: KanbanCmd,
    },
}

#[derive(Subcommand)]
enum KanbanCmd {
    /// Add a ticket to backlog
    Add {
        /// Ticket description
        description: String,
        /// Source of the ticket (JB, Agent, Web)
        #[arg(long)]
        source: Option<String>,
    },
    /// Move backlog -> todo
    Schedule {
        /// Ticket ID (e.g. #003)
        id: String,
    },
    /// Move todo -> inprogress
    Start {
        /// Ticket ID (e.g. #003)
        id: String,
        /// Agent assigned (e.g. agy)
        #[arg(long)]
        agent: Option<String>,
    },
    /// Move inprogress -> done
    Done {
        /// Ticket ID (e.g. #003)
        id: String,
    },
    /// Show tickets by state
    List {
        /// Filter by state
        #[arg(long, value_parser = ["backlog", "todo", "inprogress", "done"])]
        state: Option<String>,
    },
}

// ── File paths ─────────────────────────────────────────────────────────────

fn kanban_path(state: &str) -> PathBuf {
    PathBuf::from(KANBAN_DIR).join(format!("{}.md", state))
}

fn validate_lore_dir() {
    let p = Path::new(KANBAN_DIR);
    if !p.exists() {
        eprintln!(
            "❌ No lore/kanban/ directory found here ({}).\n   Run this from the root of a lore-initialized project.",
            p.canonicalize().unwrap_or_else(|_| p.to_path_buf()).display()
        );
        process::exit(1);
    }
}

// ── Reading / writing ──────────────────────────────────────────────────────

fn read_all() -> std::collections::HashMap<&'static str, String> {
    let mut map = std::collections::HashMap::new();
    for state in STATES {
        let p = kanban_path(state);
        let content = if p.exists() {
            fs::read_to_string(&p).unwrap_or_default()
        } else {
            format!("# {}\n\n", capitalize(state))
        };
        map.insert(*state, content);
    }
    map
}

fn read_state(state: &str) -> String {
    let p = kanban_path(state);
    if p.exists() {
        fs::read_to_string(&p).unwrap_or_default()
    } else {
        format!("# {}\n\n", capitalize(state))
    }
}

fn write_state(state: &str, text: &str) {
    let p = kanban_path(state);
    if let Some(parent) = p.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&p, text).unwrap_or_else(|e| {
        eprintln!("❌ Failed to write {}: {}", p.display(), e);
        process::exit(1);
    });
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

// ── Parsing ────────────────────────────────────────────────────────────────

/// Extract all (id_str, checkbox, rest) from kanban text
fn parse_tasks(text: &str) -> Vec<(String, String, String)> {
    let task_re = Regex::new(r"(?m)^- (?P<box>\[.\]) (?P<rest>.+)$").unwrap();
    let id_re = Regex::new(r"#(\d+)").unwrap();

    let mut results = Vec::new();
    for cap in task_re.captures_iter(text) {
        let rest = cap["rest"].to_string();
        if let Some(id_cap) = id_re.captures(&rest) {
            let id_str = id_cap.get(0).unwrap().as_str().to_string();
            results.push((id_str, cap["box"].to_string(), rest));
        }
    }
    results
}

fn highest_id(lines: &std::collections::HashMap<&str, String>) -> u32 {
    let id_re = Regex::new(r"#(\d+)").unwrap();
    let mut max_id = 0u32;
    for text in lines.values() {
        for cap in id_re.captures_iter(text) {
            let num: u32 = cap[1].parse().unwrap_or(0);
            if num > max_id {
                max_id = num;
            }
        }
    }
    max_id
}

/// Strip tags (`` `[key: ...]` ``) from a line, return clean description
fn strip_tags(rest: &str) -> String {
    let tag_re = Regex::new(r"`\[[^\]]*\]`").unwrap();
    let id_re = Regex::new(r"#\d+").unwrap();
    let cleaned = tag_re.replace_all(rest, "");
    let cleaned = id_re.replace(&cleaned, "");
    cleaned.trim().to_string()
}

/// Find a specific tag value from a line (e.g. `source: JB, 2026-07-20` -> "JB, 2026-07-20")
fn extract_tag<'a>(text: &'a str, tag_name: &str) -> Option<&'a str> {
    let pattern = format!(r"`\[{}:\s*([^\]]*)\]`", regex::escape(tag_name));
    let re = Regex::new(&pattern).unwrap();
    re.captures(text).map(|c| c.get(1).unwrap().as_str().trim())
}

/// Remove a line containing the given ID from a state file. Returns (box, rest) if found.
fn remove_line_from(state: &str, id_str: &str) -> Option<(String, String)> {
    let text = read_state(state);
    let task_re = Regex::new(r"(?m)^- (?P<box>\[.\]) (?P<rest>.+)$").unwrap();
    let id_re = Regex::new(r"#\d+").unwrap();

    let mut new_lines = Vec::new();
    let mut removed = None;

    for line in text.lines() {
        if let Some(cap) = task_re.captures(line) {
            let rest = &cap["rest"];
            if let Some(id_cap) = id_re.find(rest) {
                if id_cap.as_str() == id_str.trim() {
                    removed = Some((cap["box"].to_string(), rest.to_string()));
                    continue;
                }
            }
        }
        new_lines.push(line.to_string());
    }

    write_state(state, &(new_lines.join("\n") + "\n"));
    removed
}

fn append_line_to(state: &str, line: &str) {
    let text = read_state(state);
    let text = text.trim_end().to_string();
    write_state(state, &(text + "\n" + line + "\n"));
}

// ── Commands ───────────────────────────────────────────────────────────────

fn cmd_add(description: &str, source: Option<&str>) {
    let lines = read_all();
    let next_id = highest_id(&lines) + 1;
    let source = source.unwrap_or("Agent");
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let line = format!(
        "- [ ] #{:03} {} `[source: {}, {}]`",
        next_id, description, source, today
    );
    append_line_to("backlog", &line);
    println!("✅ Added #{:03} to backlog", next_id);
    println!("   {}", description);
    println!("   Source: {}", source);
}

fn cmd_schedule(id: &str) {
    let id = id.trim();
    let result = remove_line_from("backlog", id);
    match result {
        None => {
            eprintln!("❌ Task {} not found in backlog.", id);
            process::exit(1);
        }
        Some((_box, rest)) => {
            let today = Utc::now().format("%Y-%m-%d").to_string();
            let desc = strip_tags(&rest);
            // Preserve source tag if present
            let default_source = format!("Agent, {}", today);
            let source_part = extract_tag(&rest, "source").unwrap_or(&default_source);
            let line = format!(
                "- [ ] {} {} `[source: {}]` `[scheduled: {}]`",
                id, desc, source_part, today
            );
            append_line_to("todo", &line);
            println!("✅ {} scheduled → todo", id);
        }
    }
}

fn cmd_start(id: &str, agent: Option<&str>) {
    let id = id.trim();
    let result = remove_line_from("todo", id);
    match result {
        None => {
            eprintln!("❌ Task {} not found in todo.", id);
            process::exit(1);
        }
        Some((_box, rest)) => {
            let today = Utc::now().format("%Y-%m-%d").to_string();
            let desc = strip_tags(&rest);
            let agent = agent.unwrap_or("unknown");
            let line = format!(
                "- [~] {} {} `[started: {}, assigned: {}]`",
                id, desc, today, agent
            );
            append_line_to("inprogress", &line);
            println!("✅ {} started → in progress (agent: {})", id, agent);
        }
    }
}

fn cmd_done(id: &str) {
    let id = id.trim();
    let result = remove_line_from("inprogress", id);
    match result {
        None => {
            eprintln!("❌ Task {} not found in inprogress.", id);
            process::exit(1);
        }
        Some((_box, rest)) => {
            let today = Utc::now().format("%Y-%m-%d").to_string();
            let desc = strip_tags(&rest);
            // Extract agent from `[started: DATE, assigned: NAME]` or `[started: DATE, agent: NAME]`
            let agent = extract_inprogress_agent(&rest).unwrap_or_else(|| "unknown".to_string());
            let line = format!(
                "- [x] {} {} `[completed: {}, by: {}]`",
                id, desc, today, agent
            );
            append_line_to("done", &line);
            println!("✅ {} completed → done", id);
        }
    }
}

/// Extract agent name from inprogress tag format: `[started: DATE, assigned: NAME]` or `[started: DATE, agent: NAME]`
fn extract_inprogress_agent(text: &str) -> Option<String> {
    let re1 = Regex::new(r"`\[started: [^\]]+,\s*assigned:\s*([^\]]+)\]`").unwrap();
    if let Some(c) = re1.captures(text) {
        return Some(c.get(1).unwrap().as_str().trim().to_string());
    }
    let re2 = Regex::new(r"`\[started: [^\]]+,\s*agent:\s*([^\]]+)\]`").unwrap();
    if let Some(c) = re2.captures(text) {
        return Some(c.get(1).unwrap().as_str().trim().to_string());
    }
    None
}

fn cmd_list(state_filter: Option<&str>) {
    let data = read_all();
    let states: Vec<&str> = match state_filter {
        Some(s) => vec![s],
        None => STATES.to_vec(),
    };

    for state in states {
        let text = &data[state];
        let tasks = parse_tasks(text);
        if tasks.is_empty() {
            continue;
        }
        let label = match state {
            "backlog" => "BACKLOG",
            "todo" => "TODO",
            "inprogress" => "IN PROGRESS",
            "done" => "DONE",
            _ => "",
        };
        println!("\n─── {} ───", label);
        for (_id_str, box_str, rest) in &tasks {
            let id_str = rest.split_whitespace().next().unwrap_or("");
            let desc = strip_tags(rest);
            // Collect tag string for display
            let tag_re = Regex::new(r"`\[([^\]]*)\]`").unwrap();
            let tags: Vec<String> = tag_re
                .captures_iter(rest)
                .map(|c| c.get(1).unwrap().as_str().to_string())
                .collect();
            let tag_display = if tags.is_empty() {
                String::new()
            } else {
                format!("  [{}]", tags.join(", "))
            };
            println!("  {} {} {}{}", box_str, id_str, desc, tag_display);
        }
    }
}

// ── Main ───────────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Kanban { cmd } => {
            validate_lore_dir();
            match cmd {
                KanbanCmd::Add {
                    description,
                    source,
                } => cmd_add(description, source.as_deref()),
                KanbanCmd::Schedule { id } => cmd_schedule(id),
                KanbanCmd::Start { id, agent } => cmd_start(id, agent.as_deref()),
                KanbanCmd::Done { id } => cmd_done(id),
                KanbanCmd::List { state } => cmd_list(state.as_deref()),
            }
        }
    }
}
