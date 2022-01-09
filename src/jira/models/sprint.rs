use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sprint {
    pub id: i32,
    #[serde(rename = "self")]
    pub self_: String,
    pub state: String,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub complete_date: Option<String>,
    pub origin_board_id: i32,
    pub goal: String,
}
