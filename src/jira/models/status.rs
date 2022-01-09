use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: String,
    pub name: String,
    pub description: String,
    pub status_category: StatusCategory,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: i32,
    pub name: String,
    pub key: String,
    pub color_name: String,
}
