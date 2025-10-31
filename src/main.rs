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
        // TBC
    },
    #[command(about = "Complete a task")]
    #[command(arg_required_else_help = true)]
    Complete {
        // TBC
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
        todo!() // TBC
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
        _ => {
            todo!() // TBC
        }
    }
}
