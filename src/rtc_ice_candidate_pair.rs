
use rtc_ice_candidate::RtcIceCandidate;

#[derive(Clone)]
pub struct RtcIceCandidatePair {
    pub local: RtcIceCandidate,
    pub remote: RtcIceCandidate,
}

impl RtcIceCandidatePair {
    pub fn new(local: RtcIceCandidate, remote: RtcIceCandidate) -> Self {
        RtcIceCandidatePair {
            local: local.clone(),
            remote: remote.clone(),
        }
    }
}
