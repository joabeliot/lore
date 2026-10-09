mod commands;
mod dashboard;
mod models;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lore", about = "Lore - Project session and ticket management", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show version information
    Version,
    /// Create a new project
    Create {
        #[command(subcommand)]
        create_cmd: CreateCommands,
    },
    /// Recall project session info
    Recall {
        /// UUID or prefix
        input: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// List projects
    List {
        #[command(subcommand)]
        list_cmd: ListCommands,
    },
    /// Delete a project
    Delete {
        #[command(subcommand)]
        delete_cmd: DeleteCommands,
    },
    /// Manage tickets
    Ticket {
        #[command(subcommand)]
        ticket_cmd: TicketCommands,
    },
    /// Manage sessions
    Session {
        #[command(subcommand)]
        session_cmd: SessionCommands,
    },
    /// Inspect a ticket in a session (context, build, git checks)
    Inspect {
        /// Session UUID or prefix
        input: String,
        /// Ticket ID (e.g., DRS-1)
        ticket_id: String,
    },
    /// Edit project settings
    Edit {
        #[command(subcommand)]
        edit_cmd: EditCommands,
    },
    /// Update lore to the latest version
    Update,
    /// Show a dashboard in the browser
    Dashboard {
        /// Port to serve on (default: 8080)
        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

#[derive(Subcommand)]
enum CreateCommands {
    /// Create a new project
    Project {
        /// Path to zip file to unzip into lore/workspace/
        #[arg(long)]
        unzip: Option<String>,
        /// Project name
        #[arg(long)]
        name: String,
        /// Project description
        #[arg(long)]
        description: String,
        /// Working directory
        #[arg(long = "wrk-dir")]
        wrk_dir: String,
        /// Shorthand code (e.g., DRS)
        #[arg(long)]
        shorthand: String,
    },
}

#[derive(Subcommand)]
enum ListCommands {
    /// List all projects
    Projects,
}

#[derive(Subcommand)]
enum DeleteCommands {
    /// Delete a project
    Project {
        /// UUID or prefix
        input: String,
    },
}

#[derive(Subcommand)]
enum TicketCommands {
    /// Add a new ticket
    Add {
        /// Session UUID or prefix
        #[arg(long)]
        session: Option<String>,
        /// Ticket name
        #[arg(long)]
        name: String,
        /// Ticket description
        #[arg(long)]
        desc: Option<String>,
        /// Ticket status (backlog, todo, inprogress, done)
        #[arg(long)]
        status: Option<String>,
        /// Ticket source
        #[arg(long)]
        source: Option<String>,
        /// Ticket priority (P0, P1, P2, P3)
        #[arg(long)]
        priority: Option<String>,
        /// Comma-separated tags
        #[arg(long)]
        tags: Option<String>,
        /// Comma-separated context file paths (relative to lore_dir)
        #[arg(long)]
        context: Option<String>,
    },
    /// List tickets
    List {
        /// Session UUID or prefix
        #[arg(long)]
        session: Option<String>,
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
        /// Filter by priority
        #[arg(long)]
        priority: Option<String>,
    },
    /// Show ticket details
    Show {
        /// Session UUID or prefix
        session: Option<String>,
        /// Ticket ID (e.g., DRS-1)
        ticket_id: String,
    },
    /// Schedule a ticket (set status to todo)
    Schedule {
        /// Session UUID or prefix
        session: Option<String>,
        /// Ticket ID
        ticket_id: String,
    },
    /// Start working on a ticket
    Start {
        /// Session UUID or prefix
        session: Option<String>,
        /// Ticket ID
        ticket_id: String,
        /// Agent name
        #[arg(long)]
        agent: String,
    },
    /// Mark a ticket as done
    Done {
        /// Session UUID or prefix
        session: Option<String>,
        /// Ticket ID
        ticket_id: String,
    },
    /// Edit a ticket
    Edit {
        /// Session UUID or prefix
        session: Option<String>,
        /// Ticket ID
        ticket_id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New description
        #[arg(long)]
        desc: Option<String>,
        /// New priority
        #[arg(long)]
        priority: Option<String>,
        /// New comma-separated tags
        #[arg(long)]
        tags: Option<String>,
        /// New status
        #[arg(long)]
        status: Option<String>,
        /// Comma-separated context file paths (relative to lore_dir)
        #[arg(long)]
        context: Option<String>,
    },
}

#[derive(Subcommand)]
enum SessionCommands {
    /// Close a session (delete session file)
    Close {
        /// UUID or prefix
        input: String,
    },
    /// Show session status with ticket counts
    Status {
        /// UUID or prefix
        input: String,
    },
    /// Attach to an existing project by creating a session
    Attach {
        /// Working directory containing lore/config.yml
        #[arg(long = "wrk-dir")]
        wrk_dir: String,
    },
    /// Log a message to the active ticket in a session
    Log {
        /// Session UUID or prefix
        input: String,
        /// Message to log
        message: String,
    },
}

#[derive(Subcommand)]
enum EditCommands {
    /// Edit project settings
    Project {
        /// Session UUID or prefix
        input: String,
        /// New project name
        #[arg(long)]
        name: Option<String>,
        /// New project description
        #[arg(long)]
        description: Option<String>,
        /// New shorthand code
        #[arg(long)]
        shorthand: Option<String>,
        /// New working directory
        #[arg(long = "wrk-dir")]
        wrk_dir: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Version => commands::cmd_version(),
        Commands::Create { create_cmd } => match create_cmd {
            CreateCommands::Project {
                unzip,
                name,
                description,
                wrk_dir,
                shorthand,
            } => commands::cmd_create_project(
                unzip.clone(),
                name,
                description,
                wrk_dir,
                shorthand,
            ),
        },
        Commands::Recall { input, json } => commands::cmd_recall(input, *json),
        Commands::List { list_cmd } => match list_cmd {
            ListCommands::Projects => commands::cmd_list_projects(),
        },
        Commands::Delete { delete_cmd } => match delete_cmd {
            DeleteCommands::Project { input } => commands::cmd_delete_project(input),
        },
        Commands::Ticket { ticket_cmd } => match ticket_cmd {
            TicketCommands::Add {
                session,
                name,
                desc,
                status,
                source,
                priority,
                tags,
                context,
            } => commands::cmd_ticket_add(
                session.as_deref(),
                name,
                desc.as_deref(),
                status.as_deref(),
                source.as_deref(),
                priority.as_deref(),
                tags.as_deref(),
                context.as_deref(),
            ),
            TicketCommands::List {
                session,
                status,
                priority,
            } => commands::cmd_ticket_list(
                session.as_deref(),
                status.as_deref(),
                priority.as_deref(),
            ),
            TicketCommands::Show { session, ticket_id } => {
                commands::cmd_ticket_show(session.as_deref(), ticket_id)
            }
            TicketCommands::Schedule { session, ticket_id } => {
                commands::cmd_ticket_schedule(session.as_deref(), ticket_id)
            }
            TicketCommands::Start {
                session,
                ticket_id,
                agent,
            } => commands::cmd_ticket_start(session.as_deref(), ticket_id, agent),
            TicketCommands::Done { session, ticket_id } => {
                commands::cmd_ticket_done(session.as_deref(), ticket_id)
            }
            TicketCommands::Edit {
                session,
                ticket_id,
                name,
                desc,
                priority,
                tags,
                status,
                context,
            } => commands::cmd_ticket_edit(
                session.as_deref(),
                ticket_id,
                name.as_deref(),
                desc.as_deref(),
                priority.as_deref(),
                tags.as_deref(),
                status.as_deref(),
                context.as_deref(),
            ),
        },
        Commands::Session { session_cmd } => match session_cmd {
            SessionCommands::Close { input } => commands::cmd_session_close(input),
            SessionCommands::Status { input } => commands::cmd_session_status(input),
            SessionCommands::Attach { wrk_dir } => commands::cmd_session_attach(wrk_dir),
            SessionCommands::Log { input, message } => {
                commands::cmd_session_log(input, message)
            }
        },
        Commands::Inspect { input, ticket_id } => commands::cmd_inspect(input, ticket_id),
        Commands::Edit { edit_cmd } => match edit_cmd {
            EditCommands::Project {
                input,
                name,
                description,
                shorthand,
                wrk_dir,
            } => commands::cmd_edit_project(input, name.as_deref(), description.as_deref(), shorthand.as_deref(), wrk_dir.as_deref()),
        },
        Commands::Update => commands::cmd_update(),
        Commands::Dashboard { port } => dashboard::cmd_dashboard(*port),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
