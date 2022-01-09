use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "self")]
    pub self_: String,
    pub account_id: String,
    pub email_address: String,
    pub display_name: String,
    pub active: bool,
    pub time_zone: String,
    pub account_type: String,
}

impl User {
    pub fn is(&self, email: &str) -> bool {
        self.email_address == email
    }
}
