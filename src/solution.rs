use clap::{Parser, Subcommand, ArgAction};
use sqlite;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new task")]
    #[command(arg_required_else_help = true)]
    Add {
        task: String,
    },
    #[command(about = "Complete a task")]
    #[command(arg_required_else_help = true)]
    Complete {
        task_id: i64,
    },
    #[command(about = "List tasks")]
    #[command(arg_required_else_help = true)]
    List {
        #[arg(short, long, help = "Show all tasks including completed ones", action = ArgAction::SetTrue)]
        all: Option<bool>,
    },
}

trait DBOperation {
    fn run(&self, connection: &sqlite::Connection) -> Result<(), sqlite::Error>;
}

impl DBOperation for Commands {
    fn run(&self, connection: &sqlite::Connection) -> Result<(), sqlite::Error> {
        match self {
            Commands::Add { task } => {
                let mut statement = connection
                    .prepare("INSERT INTO tasks (name, start_time) VALUES (?, ?)")
                    .unwrap();
                statement.bind((1, task.as_str())).unwrap();
                statement.bind((2, chrono::Utc::now().timestamp())).unwrap();
                statement.next().unwrap();
                println!("Added task: {}", task);
                Ok(())
            }
            Commands::Complete { task_id } => {
                let mut statement = connection
                    .prepare("UPDATE tasks SET end_time = ? WHERE id = ?")
                    .unwrap();
                statement.bind((1, chrono::Utc::now().timestamp())).unwrap();
                statement.bind((2, task_id.clone())).unwrap();
                statement.next().unwrap();
                println!("Completed task with ID: {}", task_id);
                Ok(())
            }
            Commands::List { all } => {
                let mut statement = if all.unwrap_or(false) {
                    connection.prepare("SELECT id, name, start_time, end_time FROM tasks").unwrap()
                } else {
                    connection.prepare("SELECT id, name, start_time, end_time FROM tasks WHERE end_time IS NULL").unwrap()
                };
                while let sqlite::State::Row = statement.next().unwrap() {
                    let id: i64 = statement.read(0).unwrap();
                    let name: String = statement.read(1).unwrap();
                    let start_time: i64 = statement.read(2).unwrap();
                    let end_time: Option<i64> = statement.read(3).unwrap();
                    let start_time_str = format!("{}", chrono::DateTime::from_timestamp_secs(start_time).unwrap());
                    let end_time_str = match end_time {
                        Some(t) => format!("{}", chrono::DateTime::from_timestamp_secs(t).unwrap()),
                        None => "In Progress".to_string(),
                    };
                    println!("ID: {}, Name: {}, Start Time: {}, End Time: {}", id, name, start_time_str, end_time_str);
                }
                Ok(())
            }
        }
    }
}

fn init_db(connection: &sqlite::Connection) {
    let statement = connection.prepare(
        "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, name TEXT UNIQUE, start_time TIMESTAMP, end_time TIMESTAMP)"
    );
    statement.unwrap().next().unwrap();
}

fn main() {
    let cli = Cli::parse();


    let connection = sqlite::Connection::open("dev.sqlite")
    .unwrap();

    init_db(&connection);

    match &cli.command {
        Some(command) => {
            match command.run(&connection) {
                Ok(_) => {},
                Err(e) => eprintln!("Error executing command: {}", e),
            }
        }
        None => {}
    }
}
