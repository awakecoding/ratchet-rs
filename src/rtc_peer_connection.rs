
use rtc_configuration::RtcConfiguration;
use rtc_session_description::RtcSessionDescription;

#[derive(Clone)]
pub enum RtcSignalingState {
    Stable,
    HaveLocalOffer,
    HaveRemoteOffer,
    HaveLocalPrAnswer,
    HaveRemotePrAnswer,
    Closed,
}

#[derive(Clone)]
pub enum RtcIceGatheringState {
    New,
    Gathering,
    Complete,
}

#[derive(Clone)]
pub enum RtcPeerConnectionState {
    New,
    Connecting,
    Connected,
    Disconnected,
    Failed,
    Closed,
}

#[derive(Clone)]
pub enum RtcIceConnectionState {
    New,
    Checking,
    Connected,
    Completed,
    Disconnected,
    Failed,
    Closed,
}

#[derive(Clone)]
pub struct RtcPeerConnection {
    pub configuration: RtcConfiguration,
    pub local_description: Option<RtcSessionDescription>,
    pub remote_description: Option<RtcSessionDescription>,
    pub signaling_state: RtcSignalingState,
    pub ice_gathering_state: RtcIceGatheringState,
    pub ice_connection_state: RtcIceConnectionState,
    pub connection_state: RtcPeerConnectionState,
    pub can_trickle_ice_candidates: bool,
}

impl RtcPeerConnection {
    pub fn new(configuration: RtcConfiguration) -> Self {
        RtcPeerConnection {
            configuration: configuration.clone(),
            local_description: None,
            remote_description: None,
            signaling_state: RtcSignalingState::Stable,
            ice_gathering_state: RtcIceGatheringState::New,
            ice_connection_state: RtcIceConnectionState::New,
            connection_state: RtcPeerConnectionState::New,
            can_trickle_ice_candidates: false,
        }
    }
}
