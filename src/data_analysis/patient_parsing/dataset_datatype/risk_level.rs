use serde::Serialize;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize)]
pub enum Rischio {
    Alto,
    Medio,
    Basso,
}
impl Rischio {
    pub fn from_string(s: &str) -> Option<Rischio> {
        match s.to_lowercase().as_str() {
            "alto" => Some(Rischio::Alto),
            "medio" => Some(Rischio::Medio),
            "basso" => Some(Rischio::Basso),
            _ => None,
        }
    }
}
