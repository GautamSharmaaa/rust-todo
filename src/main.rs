use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};
use chrono::{Local, NaiveDate};
use colored::*;
use std::fs::{self, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;

const FILE_PATH: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    done: bool,
    priority: Priority,
    due_date: String,
}

#[derive(Parser)]
#[command(name = "Rust Advanced To-Do")]
#[command(about = "An Advanced CLI To-Do List App in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task with a priority and optional due date
    Add { 
        description: String, 
        priority: String, 
        due_date: Option<String> 
    },

    /// List all tasks (optionally filter by 'done' or 'pending')
    List { 
        #[arg(short, long)]
        status: Option<String> 
    },

    /// Mark a task as done by ID
    Done { id: usize },

    /// Remove a task by ID
    Remove { id: usize },
}

fn load_tasks() -> Vec<Task> {
    if Path::new(FILE_PATH).exists() {
        let mut file = fs::File::open(FILE_PATH).expect("Failed to open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file");
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILE_PATH).expect("Failed to open file");
    file.write_all(json.as_bytes()).expect("Failed to write file");
}

fn parse_priority(priority: &str) -> Priority {
    match priority.to_lowercase().as_str() {
        "low" => Priority::Low,
        "medium" => Priority::Medium,
        "high" => Priority::High,
        _ => {
            println!("{}", "Invalid priority! Defaulting to Medium.".yellow());
            Priority::Medium
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description, priority, due_date } => {
            let id = tasks.len() + 1;
            let priority = parse_priority(&priority);
            let due_date = due_date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

            tasks.push(Task { id, description, done: false, priority, due_date });
            save_tasks(&tasks);
            println!("{}", "Task added successfully!".green());
        }

        Commands::List { status } => {
            if tasks.is_empty() {
                println!("{}", "No tasks found.".yellow());
            } else {
                for task in &tasks {
                    let status_icon = if task.done { "✓".green() } else { "✗".red() };
                    let priority_color = match task.priority {
                        Priority::Low => "Low".blue(),
                        Priority::Medium => "Medium".yellow(),
                        Priority::High => "High".red(),
                    };

                    if let Some(filter) = &status {
                        if (filter == "done" && !task.done) || (filter == "pending" && task.done) {
                            continue;
                        }
                    }

                    println!(
                        "[{}] {} - {} (Due: {}) - Priority: {}",
                        status_icon, task.id, task.description, task.due_date, priority_color
                    );
                }
            }
        }

        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_tasks(&tasks);
                println!("{}", "Task marked as done!".green());
            } else {
                println!("{}", "Task not found.".red());
            }
        }

        Commands::Remove { id } => {
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                tasks.remove(pos);
                save_tasks(&tasks);
                println!("{}", "Task removed successfully!".green());
            } else {
                println!("{}", "Task not found.".red());
            }
        }
    }
}
