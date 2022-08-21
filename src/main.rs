use clap::{Parser, Subcommand};
use food::Food;
use db::initialize_db;
use log::log;

mod db;
mod food;
mod log;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// log food
    Log {
        #[clap(value_parser)]
        food: String,

        serving: f32,
    },

    /// initialize database
    Init
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}

fn run(cli: Cli) {
    match &cli.command {
        Some(Commands::Log { food, serving }) => {
            let food_item = Food::new(food, *serving);
            match food_item {
                Some(f) => log(f),
                None => println!("Food {} not found", food),
            }
        }
        Some(Commands::Init) => {
            initialize_db()
        }
        None => {}
    }
}
