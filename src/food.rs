use serde::Serialize;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Serialize)]
pub struct Food {
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
    name: String,
    calories: u32,
    carbohydrates: f32,
    fat: f32,
    protein: f32,
    serving: f32,
}

impl Food {
    pub fn new(name: &str, serving: f32) -> Option<Self> {
        match name {
            "boiled_eggs" => Some(Food::boiled_eggs(serving)),
            "dal_makhani" => Some(Food::dal_makhani(serving)),
            "pasta" => Some(Food::pasta(serving)),
            "fried_rice" => Some(Food::fried_rice(serving)),
            "rice" => Some(Food::rice(serving)),
            "vanilla_ice_cream" => Some(Food::vanilla_ice_cream(serving)),
            _ => None,
        }
    }
}

impl Food {
    fn boiled_eggs(serving: f32) -> Self {
        Food {
            date: Utc::now(),
            name: String::from("Boiled Eggs"),
            calories: 100,
            carbohydrates: 0.6,
            fat: 5.3,
            protein: 6.3,
            serving: serving,
        }
    }
    fn dal_makhani(serving: f32) -> Self {
        Food {
            date: Utc::now(),
            name: String::from("Dal Makhani"),
            calories: 330,
            carbohydrates: 31.0,
            fat: 19.0,
            protein: 13.0,
            serving: serving,
        }
    }
    fn fried_rice(serving: f32) -> Self {
        Food {
            date: Utc::now(),
            name: String::from("Fried Rice"),
            calories: 238,
            carbohydrates: 45.0,
            fat: 4.1,
            protein: 5.5,
            serving: serving,
        }
    }
    fn pasta(serving: f32) -> Self {
        Food {
            date: Utc::now(),
            name: String::from("Pasta"),
            calories: 196,
            carbohydrates: 38.0,
            fat: 1.2,
            protein: 7.2,
            serving: serving,
        }
    }
    fn rice(serving: f32) -> Self {
        Food {
            date: Utc::now(),
            name: String::from("Cooked Rice"),
            calories: 150,
            carbohydrates: 35.0,
            fat: 0.2,
            protein: 3.0,
            serving: serving,
        }
    }
    fn vanilla_ice_cream(serving: f32) -> Self {
        Food {
            date: Utc::now(),
            name: String::from("Vanilla Ice Cream"),
            calories: 168,
            carbohydrates: 21.5,
            fat: 7.5,
            protein: 3.5,
            serving: serving,
        }
    }
}
