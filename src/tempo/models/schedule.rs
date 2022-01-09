use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub date: String,
    pub required_seconds: i32,
    #[serde(rename = "type")]
    pub type_: String,
}
