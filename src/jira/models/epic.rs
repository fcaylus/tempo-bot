use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Epic {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: i32,
    pub key: String,
    pub name: String,
    pub summary: String,
    pub color: EpicColor,
    pub done: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpicColor {
    pub key: String,
}
