use std::fs;
use chrono::format::ParseError;
use chrono::{Date, NaiveDate, Utc};

pub fn parse_str_to_date(arg: &str) -> Result<Date<Utc>, ParseError> {
    let naive_date = NaiveDate::parse_from_str(arg, "%e/%m/%Y")?;
    let date = Date::from_utc(naive_date, Utc);
    Ok(date)
}

pub fn parse_date_to_string(date: &Date<Utc>) -> String {
    format!("{}", date.and_hms(0, 0, 0).format("%Y/%m/%e"))
}

pub fn get_track_directory() -> String {
    let home_dir = match home::home_dir() {
        Some(path) => path.to_str().unwrap().to_string(), 
        None => panic!("Can't find home directory")
    };

    let directory = format!("{}/.track", home_dir);
    fs::create_dir_all(&directory).unwrap();

    directory 
}