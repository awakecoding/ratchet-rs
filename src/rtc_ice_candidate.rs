
use rtc_ice_transport::RtcIceComponent;

#[derive(Clone)]
pub enum RtcIceProtocol {
    Udp,
    Tcp,
}

#[derive(Clone)]
pub enum RtcIceTcpCandidateType {
    Active,
    Passive,
    So,
}

#[derive(Clone)]
pub enum RtcIceCandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
}

#[derive(Clone)]
pub struct RtcIceCandidate {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_line_index: u16,
    pub foundation: String,
    pub component: RtcIceComponent,
    pub priority: u32,
    pub ip: String,
    pub protocol: RtcIceProtocol,
    pub port: u16,
    pub candidate_type: RtcIceCandidateType,
    pub tcp_candidate_type: RtcIceTcpCandidateType,
    pub related_address: String,
    pub related_port: u16,
    pub username_fragment: String,
}

impl RtcIceCandidate {
    pub fn new(candidate: &str, sdp_mid: &str, sdp_line_index: u16, username_fragment: &str) -> Self {
        RtcIceCandidate {
            candidate: candidate.to_string(),
            sdp_mid: sdp_mid.to_string(),
            sdp_line_index: sdp_line_index,
            foundation: "".to_string(),
            component: RtcIceComponent::Rtp,
            priority: 0,
            ip: "".to_string(),
            protocol: RtcIceProtocol::Udp,
            port: 0,
            candidate_type: RtcIceCandidateType::Host,
            tcp_candidate_type: RtcIceTcpCandidateType::Active,
            related_address: "".to_string(),
            related_port: 0,
            username_fragment: username_fragment.to_string(),
        }
    }
}
