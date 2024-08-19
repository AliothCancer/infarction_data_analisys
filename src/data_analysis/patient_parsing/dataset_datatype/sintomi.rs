use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Sintomi(Vec<Sintomo>);
impl Sintomi {
    pub fn from_string(sintomi: &str) -> Option<Self> {
        Some(Sintomi(
            sintomi
                .split(',')
                .filter_map(Sintomo::from_string)
                .collect::<Vec<_>>(),
        ))
    }
    pub fn inner(&self) -> &Vec<Sintomo> {
        &self.0
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum Sintomo {
    Angina,
    Astenia,
    BrucioreRetrosternale,
    Cardiopalmo,
    CostrizioneGiugulare,
    CostrizioneMandibolare,
    CostrizioneRetrosternale,
    Dispnea,
    DoloreBraccioSinistro,
    DoloreInterscapolare,
    Epigastralgie,
    Lipotimia,
    Malessere,
    No,
    OppressionEpigastrica,
    OppressionRetrosternale,
    Precordialgie,
    ScompensoCardiaco,
    Sincope,
    Toracoalgie,
    Vertigini,
}

impl Sintomo {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn from_string(s: &str) -> Option<Sintomo> {
        match s.trim() {
            "angina" => Some(Sintomo::Angina),
            "astenia" => Some(Sintomo::Astenia),
            "bruciore retrosternale" => Some(Sintomo::BrucioreRetrosternale),
            "cardiopalmo" => Some(Sintomo::Cardiopalmo),
            "costrizione giugulare" => Some(Sintomo::CostrizioneGiugulare),
            "costrizione mandibolare" => Some(Sintomo::CostrizioneMandibolare),
            "costrizione retrosternale" => Some(Sintomo::CostrizioneRetrosternale),
            "dispnea" => Some(Sintomo::Dispnea),
            "dolore braccio sinistro" => Some(Sintomo::DoloreBraccioSinistro),
            "dolore interscapolare" => Some(Sintomo::DoloreInterscapolare),
            "epigastralgie" => Some(Sintomo::Epigastralgie),
            "lipotimia" => Some(Sintomo::Lipotimia),
            "malessere" => Some(Sintomo::Malessere),
            "no" => Some(Sintomo::No),
            "oppressione epigastrica" => Some(Sintomo::OppressionEpigastrica),
            "oppressione retrosternale" => Some(Sintomo::OppressionRetrosternale),
            "precordialgie" => Some(Sintomo::Precordialgie),
            "scompenso cardiaco" => Some(Sintomo::ScompensoCardiaco),
            "sincope" => Some(Sintomo::Sincope),
            "toracoalgie" => Some(Sintomo::Toracoalgie),
            "vertigini" => Some(Sintomo::Vertigini),
            _ => None,
        }
    }
}
