use super::XYZ;
pub use crate::dep::v0_4_0::scpcoa::SideOfTrack;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
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
    pub azim_ang: f64,
    pub layover_ang: f64,
}

#[cfg(test)]
mod tests {
    use super::SCPCOA;
    use quick_xml::de::from_str;

    #[test]
    fn test_scpcoa() {
        let xml_str = r#"<SCPCOA><SCPTime>0</SCPTime><ARPPos><X>0</X><Y>0</Y><Z>
            0</Z></ARPPos><ARPVel><X>0</X><Y>0</Y><Z>0</Z></ARPVel><ARPAcc><X>0
            </X><Y>0</Y><Z>0</Z></ARPAcc><SideOfTrack>L</SideOfTrack>
            <SlantRange>0</SlantRange><GroundRange>0</GroundRange>
            <DopplerConeAng>0</DopplerConeAng><GrazeAng>0</GrazeAng>
            <IncidenceAng>0</IncidenceAng><TwistAng>0</TwistAng><SlopeAng>0
            </SlopeAng><AzimAng>0</AzimAng><LayoverAng>0</LayoverAng></SCPCOA>"#;
        assert!(match from_str::<SCPCOA>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
