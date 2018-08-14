
#[derive(Clone)]
pub enum RtcSdpType {
    Offer,
    PrAnswer,
    Answer,
    Rollback,
}

#[derive(Clone)]
pub struct RtcSessionDescription {
    pub sdp_type: RtcSdpType,
    pub sdp_data: String,
}

impl RtcSessionDescription {
    pub fn new(sdp_type: RtcSdpType, sdp_data: &str) -> Self {
        RtcSessionDescription {
            sdp_type: sdp_type,
            sdp_data: sdp_data.to_string(),
        }
    }
}
