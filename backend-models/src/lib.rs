// add models here

use aide::OperationInput;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct LibraryOpeningHour {
    pub name: String,
    pub desc: String,
    pub start: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LibraryOpeningHourRequestBody {
    #[serde(default = "default_days")]
    pub days: u64,
    #[serde(default = "default_filter")]
    pub filter: String,
}

fn default_days() -> u64 {
    7
}

fn default_filter() -> String {
    String::from("all")
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LibraryOpeningHourResponseEntry {
    pub name: String,
    pub hours: String,
}

impl From<LibraryOpeningHour> for LibraryOpeningHourResponseEntry {
    fn from(hour: LibraryOpeningHour) -> Self {
        Self {
            name: hour.name,
            hours: hour.desc,
        }
    }
}

impl OperationInput for LibraryOpeningHourRequestBody {}
