use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: String,
    pub name: String,
}
