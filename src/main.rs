use chrono::{Date, Utc};
use clap::{Parser, Subcommand};

use db::initialize_db;
use log::{log, show_log};

mod db;
mod food;
mod log;
mod utils;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// log food
    Log {
        #[clap(value_parser)]
        food: String,

        serving: f32,
    },

    /// show history
    Show {
        #[clap(parse(try_from_str = utils::parse_str_to_date))]
        date: Date<Utc>,
    },

    /// initialize database
    Init,
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}

fn run(cli: Cli) {
    match &cli.command {
        Commands::Log { food, serving } => {
            log(food, *serving);
        }
        Commands::Show { date } => {
            show_log(date);
        }
        Commands::Init => initialize_db(),
    }
}
