use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::fs;
use time::OffsetDateTime;

/// Ultra-fast CLI snippet manager
#[derive(Parser)]
#[command(name = "snip", about = "Manage your code/text snippets locally")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save a snippet with content
    Save {
        name: String,
        content: String,
    },
    /// Get a snippet by name (prints directly)
    Get {
        name: String,
    },
    /// List all snippets
    List,
    /// Remove a snippet by name
    Rm {
        name: String,
    },
    /// Search snippets by name or content (partial match)
    Search {
        query: String,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let proj_dirs = ProjectDirs::from("dev", "snip", "snip").unwrap();
    fs::create_dir_all(proj_dirs.data_dir())?;
    let db_path = proj_dirs.data_dir().join("snip.db");
    let conn = Connection::open(db_path)?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS snippets (
            id          INTEGER PRIMARY KEY,
            name        TEXT UNIQUE NOT NULL,
            content     TEXT NOT NULL,
            created_at  INTEGER NOT NULL
        );
        "
    )?;

    match cli.command {
        Commands::Save { name, content } => {
            conn.execute(
                "INSERT OR REPLACE INTO snippets (name, content, created_at)
                 VALUES (?1, ?2, ?3)",
                params![&name, &content, OffsetDateTime::now_utc().unix_timestamp()],
            )?;
            println!("Snippet '{}' saved.", name);
        }
        Commands::Get { name } => {
            let mut stmt = conn.prepare("SELECT content FROM snippets WHERE name = ?1")?;
            let snippet: String = stmt.query_row([&name], |row| row.get(0))?;
            println!("{}", snippet); // pure CLI output
        }
        Commands::List => {
            let mut stmt = conn.prepare("SELECT id, name FROM snippets")?;
            let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)))?;
            println!("{:<5} | {}", "ID", "Name");
            println!("{}", "-".repeat(30));
            for r in rows {
                let (id, name) = r?;
                println!("{:<5} | {}", id, name);
            }
        }
        Commands::Rm { name } => {
            let affected = conn.execute("DELETE FROM snippets WHERE name = ?1", [&name])?;
            if affected > 0 {
                println!("Snippet '{}' removed.", name);
            } else {
                println!("Snippet '{}' not found.", name);
            }
        }
        Commands::Search { query } => {
            let like_query = format!("%{}%", query);
            let mut stmt = conn.prepare(
                "SELECT id, name, content FROM snippets WHERE name LIKE ?1 OR content LIKE ?1"
            )?;
            let rows = stmt.query_map([&like_query], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
            })?;

            println!("{:<5} | {:<20} | {}", "ID", "Name", "Preview");
            println!("{}", "-".repeat(60));
            for r in rows {
                let (id, name, content) = r?;
                let preview = if content.len() > 50 { &content[..50] } else { &content };
                println!("{:<5} | {:<20} | {}", id, name, preview);
            }
        }
    }

    Ok(())
}
   
