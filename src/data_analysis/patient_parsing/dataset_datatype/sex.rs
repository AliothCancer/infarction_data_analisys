use serde::Serialize;

#[derive(Debug, PartialEq, PartialOrd, Serialize)]
pub enum Sex {
    Male,
    Femmina,
}

impl Sex {
    pub fn from_string(s: &str) -> Option<Sex> {
        match s.to_lowercase().as_str() {
            "m" => Some(Sex::Male),
            "f" => Some(Sex::Femmina),
            _ => None,
        }
    }
}
