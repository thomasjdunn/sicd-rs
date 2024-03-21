use super::Parameter;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorStatistics {
    #[serde(rename = "CompositeSCP")]
    #[serde(default)]
    pub composite_scp: CompositeSCP,
    #[serde(rename = "Components")]
    #[serde(default)]
    pub components: Components,
    #[serde(rename = "AdditionalParams")]
    #[serde(default)]
    pub additional_params: Vec<Parameter>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct CompositeSCP {
    #[serde(rename = "RgAzErr")]
    #[serde(default)]
    pub rg_az_err: RgAzErr,
    #[serde(rename = "RowColErr")]
    #[serde(default)]
    pub row_col_err: RowColErr,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RgAzErr {
    #[serde(rename = "Rg")]
    pub rg: f64,
    #[serde(rename = "Az")]
    pub az: f64,
    #[serde(rename = "RgAz")]
    pub rg_az: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RowColErr {
    #[serde(rename = "Row")]
    pub row: f64,
    #[serde(rename = "Col")]
    pub col: f64,
    #[serde(rename = "RowCol")]
    pub row_col: f64,
}

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Components {
    #[serde(rename = "PosVelErr")]
    #[serde(default)]
    pub pos_vel_err: PosVelErr,
    #[serde(rename = "RadarSensor")]
    #[serde(default)]
    pub radar_sensor: RadarSensor,
    #[serde(rename = "TropoErro")]
    #[serde(default)]
    pub tropo_erro: TropoError,
    #[serde(rename = "IonoError")]
    #[serde(default)]
    pub iono_error: IonoError,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct PosVelErr {
    #[serde(rename = "Frame")]
    pub frame: Frame,
    #[serde(rename = "P1")]
    pub p1: f64,
    #[serde(rename = "P2")]
    pub p2: f64,
    #[serde(rename = "P3")]
    pub p3: f64,
    #[serde(rename = "V1")]
    pub v1: f64,
    #[serde(rename = "V2")]
    pub v2: f64,
    #[serde(rename = "V3")]
    pub v3: f64,
    #[serde(rename = "CorrCoefs")]
    #[serde(default)]
    pub corr_coefs: CorrCoefs,
    #[serde(rename = "PositionDecorr")]
    #[serde(default)]
    pub position_decorr: Decorr,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Frame {
    #[serde(rename = "$text")]
    pub value: FrameEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum FrameEnum {
    #[default]
    ECF,
    #[serde(rename = "RIC_ECF")]
    RICECF,
    #[serde(rename = "RIC_ECI")]
    RICECI,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct CorrCoefs {
    #[serde(rename = "P1P2")]
    pub p1p2: f64,
    #[serde(rename = "P1P3")]
    pub p1p3: f64,
    #[serde(rename = "P1V1")]
    pub p1v1: f64,
    #[serde(rename = "P1V2")]
    pub p1v2: f64,
    #[serde(rename = "P1V3")]
    pub p1v3: f64,
    #[serde(rename = "P2P3")]
    pub p2p3: f64,
    #[serde(rename = "P2V1")]
    pub p2v1: f64,
    #[serde(rename = "P2V2")]
    pub p2v2: f64,
    #[serde(rename = "P2V3")]
    pub p2v3: f64,
    #[serde(rename = "P3V1")]
    pub p3v1: f64,
    #[serde(rename = "P3V2")]
    pub p3v2: f64,
    #[serde(rename = "P3V3")]
    pub p3v3: f64,
    #[serde(rename = "V1V2")]
    pub v1v2: f64,
    #[serde(rename = "V1V3")]
    pub v1v3: f64,
    #[serde(rename = "V2V3")]
    pub v2v3: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RadarSensor {
    #[serde(rename = "RangeBias")]
    pub range_bias: f64,
    #[serde(rename = "ClockFreqSF")]
    #[serde(default)]
    pub clock_freq_sf: f64,
    #[serde(rename = "TransmitFreqSF")]
    #[serde(default)]
    pub transmit_freq_sf: f64,
    #[serde(rename = "RangeBiasDecorr")]
    #[serde(default)]
    pub range_bias_decorr: Decorr,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct TropoError {
    #[serde(rename = "TropoRangeVertical")]
    #[serde(default)]
    pub tropo_range_vertical: f64,
    #[serde(rename = "TropoRangeSlant")]
    #[serde(default)]
    pub tropo_range_slant: f64,
    #[serde(rename = "TropoRangeDecorr")]
    #[serde(default)]
    pub tropo_range_decorr: Decorr,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct IonoError {
    #[serde(rename = "IonoRangeVertical")]
    #[serde(default)]
    pub iono_range_vertical: f64,
    #[serde(rename = "IonoRangeRateVertical")]
    #[serde(default)]
    pub iono_range_rate_vertical: f64,
    #[serde(rename = "IonoRgRgRateCC")]
    pub iono_rg_rg_rate_cc: f64,
    #[serde(rename = "IonoRangeVertDecorr")]
    #[serde(default)]
    pub iono_range_vert_decorr: Decorr,
}

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Decorr {
    #[serde(rename = "CorrCoefZero")]
    pub corr_coef_zero: f64,
    #[serde(rename = "DecorrRate")]
    pub decorr_rate: f64,
}
