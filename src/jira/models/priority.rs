use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: String,
    pub name: String,
}

pub enum PriorityLevel {
    Highest,
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn level(&self) -> PriorityLevel {
        match self.name.as_str() {
            "Highest" => PriorityLevel::Highest,
            "High" => PriorityLevel::High,
            "Medium" => PriorityLevel::Medium,
            "Low" => PriorityLevel::Low,
            _ => PriorityLevel::Medium,
        }
    }
}
