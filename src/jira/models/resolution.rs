use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: String,
    pub name: String,
    pub description: String,
}
