use crate::food::Food;
use csv;
use std::fs::{File, OpenOptions};

pub fn get_writer() -> csv::Writer<File> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.csv")
        .unwrap();

    let wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(log_file);

    wtr
}

pub fn log(food: Food) {
    let mut wtr = get_writer();
    wtr.serialize(&food).unwrap();
    wtr.flush().unwrap();
}
