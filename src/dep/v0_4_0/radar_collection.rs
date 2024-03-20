use super::{IdxLLH, Parameter, XYZ};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarCollection {
    #[serde(rename = "TxFrequency")]
    pub tx_frequency: TxFrequency,
    #[serde(rename = "RefFreqIndex")]
    pub ref_freq_index: Option<i32>,
    #[serde(rename = "Waveform")]
    pub waveform: Option<Waveform>,
    #[serde(rename = "TxPolarization")]
    pub tx_polarization: TxPolarization,
    #[serde(rename = "TxSequence")]
    pub tx_sequence: Option<TxSequence>,
    #[serde(rename = "RcvChannels")]
    pub rcv_channels: RcvChannels,
    #[serde(rename = "Area")]
    pub area: Option<Area>,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct TxFrequency {
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde(rename = "Max")]
    pub max: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Waveform {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "WFParameters")]
    pub wf_parameters: Vec<WFParameters>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WFParameters {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "TxPulseLength")]
    #[serde(default)]
    pub tx_pulse_length: f64,
    #[serde(rename = "TxRFBandwidth")]
    #[serde(default)]
    pub tx_rf_bandwidth: f64,
    #[serde(rename = "TxFreqStart")]
    #[serde(default)]
    pub tx_freq_start: f64,
    #[serde(rename = "TxFMRate")]
    #[serde(default)]
    pub tx_fm_rate: f64,
    #[serde(rename = "RcvDemodType")]
    #[serde(default)]
    pub rcv_demod_type: RcvDemodType,
    #[serde(rename = "RcvWindowLength")]
    #[serde(default)]
    pub rcv_window_length: f64,
    #[serde(rename = "ADCSampleRate")]
    #[serde(default)]
    pub adc_sample_rate: f64,
    #[serde(rename = "RcvIFBandwidth")]
    #[serde(default)]
    pub rcv_if_bandwidth: f64,
    #[serde(rename = "RcvFreqStart")]
    #[serde(default)]
    pub rcv_freq_start: f64,
    #[serde(rename = "RcvFMRate")]
    #[serde(default)]
    pub rcv_fm_rate: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RcvDemodType {
    #[serde(rename = "$text")]
    pub value: RcvDemodTypeEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum RcvDemodTypeEnum {
    STRETCH,
    CHIRP,
    #[default]
    UNKNOWN
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxPolarization {
    #[serde(rename = "$text")]
    pub value: TxPolarizationEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum TxPolarizationEnum {
    V,
    H,
    X,
    Y,
    S,
    E,
    RHC,
    LHC,
    OTHER,
    UNKNOWN,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct TxSequence {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "TxStep")]
    pub tx_step: Vec<TxStep>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxStep {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "WFIndex")]
    #[serde(default)]
    pub wf_index: usize,
    #[serde(rename = "TxPolarization")]
    #[serde(default)]
    pub tx_polarization: TxStepPolarization,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct TxStepPolarization {
    #[serde(rename = "$text")]
    pub value: TxStepPolarizationEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum TxStepPolarizationEnum {
    V,
    H,
    RHC,
    LHC,
    #[default]
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvChannels {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "ChanParameters")]
    pub chan_parameters: Option<Vec<ChanParameters>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ChanParameters {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "TxRcvPolarization")]
    pub tx_rcv_polarization: Option<String>, // TODO: Implement this enum
    #[serde(rename = "RcvAPCIndex")]
    pub rcv_apc_index: Option<usize>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Area {
    #[serde(rename = "Corner")]
    #[serde(default)]
    pub corner: Corner,
    #[serde(rename = "Plane")]
    #[serde(default)]
    pub plane: Plane,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Corner {
    #[serde(rename = "$value")]
    pub acp: Vec<IdxLLH>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Plane {
    #[serde(rename = "RefPt")]
    pub ref_pt: RefPt,
    #[serde(rename = "XDir")]
    pub x_dir: XDir,
    #[serde(rename = "YDir")]
    pub y_dir: YDir,
    #[serde(rename = "SegmentList")]
    #[serde(default)]
    pub segment_list: SegmentList,
    #[serde(rename = "Orientation")]
    #[serde(default)]
    pub orientation: Orientation,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RefPt {
    #[serde(rename = "@name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ECF")]
    pub ecf: XYZ,
    #[serde(rename = "Line")]
    pub line: f64,
    #[serde(rename = "Sample")]
    pub sample: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct XDir {
    #[serde(rename = "UVectECF")]
    pub u_vect_ecf: XYZ,
    #[serde(rename = "LineSpacing")]
    pub line_spacing: f64,
    #[serde(rename = "NumLines")]
    pub num_lines: u64,
    #[serde(rename = "FirstLine")]
    pub first_line: i32,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct YDir {
    #[serde(rename = "UVectECF")]
    pub u_vect_ecf: XYZ,
    #[serde(rename = "SampleSpacing")]
    pub sample_spacing: f64,
    #[serde(rename = "NumSamples")]
    pub num_samples: u64,
    #[serde(rename = "FirstSample")]
    pub first_sample: i32,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct SegmentList {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "Segment")]
    pub segment: Vec<Segment>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Segment {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "StartLine")]
    pub start_line: i32,
    #[serde(rename = "StartSample")]
    pub start_sample: i32,
    #[serde(rename = "EndLine")]
    pub end_line: i32,
    #[serde(rename = "EndSample")]
    pub end_sample: i32,
    #[serde(rename = "Identifier")]
    pub identifier: String,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Orientation {
    #[serde(rename = "$text")]
    pub value: OrientationEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum OrientationEnum {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    #[default]
    ARBITRARY,
}
