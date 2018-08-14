
use rtc_ice_server::RtcIceServer;

#[derive(Clone)]
pub enum RtcIceTransportPolicy {
    Relay,
    All,
}

#[derive(Clone)]
pub enum RtcBundlePolicy {
    Balanced,
    MaxCompat,
    MaxBundle,
}

#[derive(Clone)]
pub struct RtcConfiguration {
    pub ice_servers: Option<RtcIceServer>,
    pub ice_transport_policy: RtcIceTransportPolicy,
    pub bundle_policy: RtcBundlePolicy,
    pub peer_identity: String,
    pub ice_candidate_pool_size: u8,
}

impl RtcConfiguration {
    pub fn new() -> Self {
        RtcConfiguration {
            ice_servers: None,
            ice_transport_policy: RtcIceTransportPolicy::All,
            bundle_policy: RtcBundlePolicy::Balanced,
            peer_identity: "".to_string(),
            ice_candidate_pool_size: 0,
        }
    }
}

