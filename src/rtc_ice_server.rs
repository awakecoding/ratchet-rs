
#[derive(Clone)]
pub enum RtcIceCredentialType {
    Password,
    OAuth,
}

#[derive(Clone)]
pub struct RtcIceServer {
    pub url: String,
    pub username: Option<String>,
    pub credential: Option<String>,
    pub credential_type: RtcIceCredentialType,
}

impl RtcIceServer {
    pub fn new() -> Self {
        RtcIceServer {
            url: "".to_string(),
            username: None,
            credential: None,
            credential_type: RtcIceCredentialType::Password,
        }
    }
}
