
#[derive(Clone)]
pub enum RtcPriorityType {
    VeryLow,
    Low,
    Medium,
    High,
}

#[derive(Clone)]
pub enum RtcDataChannelState {
    Connecting,
    Open,
    Closing,
    Closed,
}

#[derive(Clone)]
pub struct RtcDataChannel {
    pub label: String,
    pub ordered: bool,
    pub max_packet_lifetime: u16,
    pub max_retransmits: u16,
    pub protocol: String,
    pub negotiated: bool,
    pub id: u16,
    pub priority: RtcPriorityType,
    pub ready_state: RtcDataChannelState,
    pub buffered_amount: u32,
    pub buffered_amount_low_threshold: u32,
}

impl RtcDataChannel {
    pub fn new() -> Self {
        RtcDataChannel {
            label: "".to_string(),
            ordered: true,
            max_packet_lifetime: 30000,
            max_retransmits: 10,
            protocol: "".to_string(),
            negotiated: false,
            id: 0,
            priority: RtcPriorityType::Medium,
            ready_state: RtcDataChannelState::Closed,
            buffered_amount: 0,
            buffered_amount_low_threshold: 0,
        }
    }
}
