
#[derive(Clone)]
pub enum RtcIceRole {
    Controlling,
    Controlled,
}

#[derive(Clone)]
pub enum RtcIceComponent {
    Rtp,
    Rtcp,
}

#[derive(Clone)]
pub enum RtcIceTransportState {
    New,
    Checking,
    Connected,
    Completed,
    Disconnected,
    Failed,
    Closed,
}

#[derive(Clone)]
pub enum RtcIceGathererState {
    New,
    Gathering,
    Complete,
}

#[derive(Clone)]
pub struct RtcIceTransport {
    pub role: RtcIceRole,
    pub component: RtcIceComponent,
    pub state: RtcIceTransportState,
    pub gathering_state: RtcIceGathererState,
}

impl RtcIceTransport {
    pub fn new() -> Self {
        RtcIceTransport {
            role: RtcIceRole::Controlling,
            component: RtcIceComponent::Rtp,
            state: RtcIceTransportState::New,
            gathering_state: RtcIceGathererState::New,
        }
    }
}

