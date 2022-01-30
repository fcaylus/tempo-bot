use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoardConfiguration {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub location: BoardLocation,
    pub estimation: Option<BoardEstimation>,
    pub column_config: ColumnConfig,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoardLocation {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: String,
    pub name: String,
    pub key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoardEstimation {
    #[serde(rename = "type")]
    pub type_: String,
    pub field: Option<BoardEstimationField>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoardEstimationField {
    pub field_id: String,
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnConfig {
    pub constraint_type: String,
    pub columns: Vec<Column>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub name: String,
    pub statuses: Vec<ColumnStatus>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnStatus {
    #[serde(rename = "self")]
    pub self_: String,
    pub id: String,
}

impl BoardConfiguration {
    pub fn estimation_field_name(&self) -> Option<String> {
        if let Some(estimation) = &self.estimation {
            if estimation.type_ != "field" {
                return None;
            }
            if let Some(field) = &estimation.field {
                return Some(field.field_id.to_string());
            }
        }

        None
    }
}
