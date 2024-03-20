use super::Parameter;
use serde::Deserialize;

pub use crate::dep::v0_4_0::error_statistics::{
    Decorr, IonoError, PosVelErr, RadarSensor, TropoError,
};

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorStatistics {
    #[serde(rename = "CompositeSCP")]
    #[serde(default)]
    pub composite_scp: CompositeSCP,
    #[serde(default)]
    pub components: Components,
    #[serde(default)]
    pub unmodeled: Unmodeled,
    #[serde(default)]
    pub additional_params: Vec<Parameter>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CompositeSCP {
    pub rg: f64,
    pub az: f64,
    pub rg_az: f64,
}

// TODO-TD: defaults
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Components {
    pub pos_vel_err: PosVelErr,
    pub radar_sensor: RadarSensor,
    #[serde(default)]
    pub tropo_erro: TropoError,
    #[serde(default)]
    pub iono_error: IonoError,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Unmodeled {
    pub xrow: f64,
    pub ycol: f64,
    pub xrow_ycol: f64,
    #[serde(default)]
    pub unmodeled_decorr: UnmodeledDecorr,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UnmodeledDecorr {
    pub xrow: Decorr,
    pub ycol: Decorr,
}

#[cfg(test)]
mod tests {
    use super::ErrorStatistics;
    use quick_xml::de::from_str;

    #[test]
    fn test_errror_statistics() {
        let xml_str = r#"<ErrorStatistics><Components><PosVelErr><Frame>RIC_ECI
            </Frame><P1>0</P1><P2>0</P2><P3>0</P3><V1>0</V1><V2>0</V2><V3>0</V3>
            <CorrCoefs><P1P2>0</P1P2><P1P3>0</P1P3><P1V1>0</P1V1><P1V2>0</P1V2>
            <P1V3>0</P1V3><P2P3>0</P2P3><P2V1>0</P2V1><P2V2>0</P2V2><P2V3>0
            </P2V3><P3V1>0</P3V1><P3V2>0</P3V2><P3V3>0</P3V3><V1V2>0</V1V2>
            <V1V3>0</V1V3><V2V3>0</V2V3></CorrCoefs></PosVelErr><RadarSensor>
            <RangeBias>0</RangeBias><ClockFreqSF>0</ClockFreqSF><TransmitFreqSF>
            0</TransmitFreqSF></RadarSensor><TropoError><TropoRangeVertical>0
            </TropoRangeVertical><TropoRangeSlant>0</TropoRangeSlant>
            </TropoError></Components></ErrorStatistics>"#;
        assert!(match from_str::<ErrorStatistics>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
