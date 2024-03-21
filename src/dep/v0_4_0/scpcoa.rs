use super::XYZ;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SCPCOA {
    #[serde(rename = "SCPTime")]
    pub scp_time: f64,
    #[serde(rename = "ARPPos")]
    pub arp_pos: XYZ,
    #[serde(rename = "ARPVel")]
    pub arp_vel: XYZ,
    #[serde(rename = "ARPAcc")]
    pub arp_acc: XYZ,
    pub side_of_track: SideOfTrack,
    pub slant_range: f64,
    pub ground_range: f64,
    pub doppler_cone_ang: f64,
    pub graze_ang: f64,
    pub incidence_ang: f64,
    pub twist_ang: f64,
    pub slope_ang: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct SideOfTrack {
    #[serde(rename = "$text")]
    pub value: SideOfTrackEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum SideOfTrackEnum {
    L,
    #[default]
    R,
}
