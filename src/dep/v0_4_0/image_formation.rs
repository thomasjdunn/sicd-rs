use super::{Parameter, Poly1D, Poly2D, CMPLX, XYZ};
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormation {
    #[serde(rename = "RcvChanProc")]
    pub rcv_chan_proc: RcvChanProc,
    #[serde(rename = "TxRcvPolarizationProc")]
    pub tx_rcv_polarization_proc: String, // TODO: implement this enum
    #[serde(rename = "TStartProc")]
    pub t_start_proc: f64,
    #[serde(rename = "TEndProc")]
    pub t_end_proc: f64,
    #[serde(rename = "TxFrequencyProc")]
    pub tx_frequency_proc: TxFrequencyProc,
    #[serde(rename = "SegmentIdentifier")]
    pub segment_identifier: Option<String>,
    #[serde(rename = "ImageFormAlgo")]
    pub image_form_algo: ImageFormAlgo,
    #[serde(rename = "STBeamComp")]
    pub st_beam_comp: STBeamComp,
    #[serde(rename = "ImageBeamComp")]
    pub image_beam_comp: ImageBeamComp,
    #[serde(rename = "AzAutofocus")]
    pub az_autofocus: AzAutofocus,
    #[serde(rename = "RgAutofocus")]
    pub rg_autofocus: RgAutofocus,
    #[serde(rename = "Processing")]
    pub processing: Option<Vec<Processing>>,
    #[serde(rename = "PolarizationCalibration")]
    pub polarization_calibration: PolCal,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RcvChanProc {
    #[serde(rename = "NumChanProc")]
    pub num_chan_proc: u64,
    #[serde(rename = "PRFScaleFactor")]
    #[serde(default)]
    pub prf_scale_factor: f64,
    #[serde(rename = "ChanIndex")]
    pub chan_index: Vec<usize>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct TxFrequencyProc {
    #[serde(rename = "MinProc")]
    pub min_proc: f64,
    #[serde(rename = "MaxProc")]
    pub max_proc: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormAlgo {
    #[serde(rename = "$text")]
    pub value: ImageFormAlgoEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum ImageFormAlgoEnum {
    PFA,
    RMA,
    RGAZCOMP,
    #[default]
    OTHER,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct STBeamComp {
    #[serde(rename = "$text")]
    pub value: STBeamCompEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum STBeamCompEnum {
    #[default]
    NO,
    GLOBAL,
    SV,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct ImageBeamComp {
    #[serde(rename = "$text")]
    pub value: ImageBeamCompEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum ImageBeamCompEnum {
    #[default]
    NO,
    SV,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct AzAutofocus {
    #[serde(rename = "$text")]
    pub value: AzAutofocusEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum AzAutofocusEnum {
    #[default]
    NO,
    GLOBAL,
    SV,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RgAutofocus {
    #[serde(rename = "$text")]
    pub value: RgAutofocusEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum RgAutofocusEnum {
    #[default]
    NO,
    GLOBAL,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Processing {
    #[serde(rename = "Type")]
    pub type_proc: String,
    #[serde(rename = "Applied")]
    pub applied: bool,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PolCal {
    #[serde(rename = "HVAngleCompApplied")]
    pub hv_angle_comp_applied: bool,
    #[serde(rename = "DistortCorrectionApplied")]
    pub distort_correction_applied: Option<bool>,
    #[serde(rename = "Distortion")]
    pub distortion: Distortion,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct Distortion {
    #[serde(rename = "CalibrationDate")]
    #[serde(default)]
    pub calibration_date: String,
    #[serde(rename = "A")]
    pub a: f64,
    #[serde(rename = "F1")]
    pub f1: CMPLX,
    #[serde(rename = "Q1")]
    pub q1: CMPLX,
    #[serde(rename = "Q2")]
    pub q2: CMPLX,
    #[serde(rename = "F2")]
    pub f2: CMPLX,
    #[serde(rename = "Q3")]
    pub q3: CMPLX,
    #[serde(rename = "Q4")]
    pub q4: CMPLX,
    #[serde(rename = "GainErrorA")]
    #[serde(default)]
    pub gain_error_a: f64,
    #[serde(rename = "GainErrorF1")]
    #[serde(default)]
    pub gain_error_f1: f64,
    #[serde(rename = "GainErrorF2")]
    #[serde(default)]
    pub gain_error_f2: f64,
    #[serde(rename = "PhaseErrorF1")]
    #[serde(default)]
    pub phase_error_f1: f64,
    #[serde(rename = "PhaseErrorF2")]
    #[serde(default)]
    pub phase_error_f2: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RGAZCOMP {
    #[serde(rename = "RgAzRefTime")]
    pub rg_az_ref_time: f64,
    #[serde(rename = "Time1")]
    pub time1: f64,
    #[serde(rename = "Time2")]
    pub time2: f64,
    #[serde(rename = "AzToCosSF")]
    pub az_to_cos_sf: f64,
    #[serde(rename = "KazToTimePoly")]
    pub kaz_to_time_poly: Poly1D,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RMA {
    #[serde(rename = "RMAlgoType")]
    pub rm_algo_type: RMAlgoType,
    #[serde(rename = "ImageType")]
    pub image_type: ImageType,
    #[serde(rename = "RMAT")]
    pub rmat: Option<RMAlgo>,
    #[serde(rename = "INCA")]
    pub inca: Option<INCA>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RMAlgoType {
    #[serde(rename = "$text")]
    pub value: RMAlgoTypeEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum RMAlgoTypeEnum {
    #[serde(rename = "OMEGA_K")]
    OMEGAK,
    CSA,
    #[serde(rename = "RG_DOP")]
    RGDOP,
    #[default]
    UNKNOWN
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageType {
    #[serde(rename = "$text")]
    pub value: ImageTypeEnum,
}

// TODO-TD: set defaults for enums
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageTypeEnum {
    RMAT,
    INCA,
}

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct RMAlgo {
    #[serde(rename = "RMRefTime")]
    pub ref_time: f64,
    #[serde(rename = "RMPosRef")]
    pub pos_ref: XYZ,
    #[serde(rename = "CosDCACOAPoly")]
    pub cos_dcacoa_poly: Poly2D,
    #[serde(rename = "Kx1")]
    pub kx1: f64,
    #[serde(rename = "Kx2")]
    pub kx2: f64,
    #[serde(rename = "Ky1")]
    pub ky1: f64,
    #[serde(rename = "Ky2")]
    pub ky2: f64,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct INCA {
    #[serde(rename = "TimeCAPoly")]
    pub time_ca_poly: Poly1D,
    #[serde(rename = "R_CA_SCP")]
    pub r_ca_scp: f64,
    #[serde(rename = "FreqZero")]
    pub freq_zero: f64,
    #[serde(rename = "DRateSFPoly")]
    pub d_rate_sf_poly: Poly2D,
    #[serde(rename = "DopCentroidPoly")]
    #[serde(default)]
    pub dop_centroid_poly: Poly2D,
    #[serde(rename = "DopCentroidCOA")]
    #[serde(default)]
    pub dop_centroid_coa: bool,
}
