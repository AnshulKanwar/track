use clap::{Parser, Subcommand};
use db::initialize_db;
use food::get_food_id;
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
    Init,
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}

fn run(cli: Cli) {
    match &cli.command {
        Some(Commands::Log { food, serving }) => {
            let food_id = get_food_id(food);
            match food_id {
                Ok(id) => log(id, *serving),
                Err(e) => println!("Error: {e:?}"),
            }
        }
        Some(Commands::Init) => initialize_db(),
        None => {}
    }
}
