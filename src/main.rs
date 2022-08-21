use clap::Parser;
use food::Food;
use log::log;

mod food;
mod log;

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_parser)]
    food: String,

    serving: f32,
}

fn main() {
    let args = Args::parse();
    run(args);
}

fn run(args: Args) {
    let food = Food::new(&args.food, args.serving);
    match food {
        Some(food) => log(food),
        None => println!("Food {} not found", args.food),
    }
}
