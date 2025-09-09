use serde::Serialize;

#[derive(Serialize)]
pub struct AuthOutput {
    access_token: String,
    refresh_token: String,
    device_id: String,
}

impl AuthOutput {

    pub fn new(access_token: &str, refresh_token: &str, device_id: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            device_id: device_id.to_string(),
        }
    }
}