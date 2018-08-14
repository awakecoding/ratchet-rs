
#[derive(Clone)]
pub struct RtcIceParameters {
    pub username_fragment: String,
    pub password: String,
}

impl RtcIceParameters {
    pub fn new(username_fragment: &str, password: &str) -> Self {
        RtcIceParameters {
            username_fragment: username_fragment.to_string(),
            password: password.to_string(),
        }
    }
}
