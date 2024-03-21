use super::{DualPolarization, Poly1D, XYZ};
pub use crate::dep::v0_5_0::image_formation::{
    AzAutofocus, Distortion, ImageBeamComp, ImageFormAlgo, Processing, RMAlgoType, RMAlgoTypeEnum,
    RcvChanProc, RgAutofocus, RgAutofocusEnum, STBeamComp, TxFrequencyProc, INCA,
};
use serde::Deserialize;
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ImageFormation {
    pub rcv_chan_proc: RcvChanProc,
    pub tx_rcv_polarization_proc: TxRcvPolarizationProc,
    pub t_start_proc: f64,
    pub t_end_proc: f64,
    pub tx_frequency_proc: TxFrequencyProc,
    #[serde(default)]
    pub segment_identifier: String,
    pub image_form_algo: ImageFormAlgo,
    #[serde(rename = "STBeamComp")]
    pub st_beam_comp: STBeamComp,
    pub image_beam_comp: ImageBeamComp,
    pub az_autofocus: AzAutofocus,
    pub rg_autofocus: RgAutofocus,
    #[serde(default)]
    pub processing: Vec<Processing>,
    #[serde(default)]
    pub polarization_calibration: PolCal,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct TxRcvPolarizationProc {
    #[serde(rename = "$text")]
    pub value: DualPolarization,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PolCal {
    pub distort_correction_applied: bool,
    pub distortion: Distortion,
}

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RgAzComp {
    #[serde(rename = "AzSF")]
    pub az_sf: f64,
    pub kaz_poly: Poly1D,
}

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct RMA {
    #[serde(rename = "RMAlgoType")]
    pub rm_algo_type: RMAlgoType,
    #[serde(rename = "ImageType")]
    pub image_type: ImageType,
    #[serde(default)]
    pub rmat: RMAlgo,
    #[serde(default)]
    pub rmcr: RMAlgo,
    #[serde(default)]
    pub inca: INCA,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct ImageType {
    #[serde(rename = "$text")]
    pub value: ImageTypeEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum ImageTypeEnum {
    RMAT,
    RMCR,
    INCA,
    #[default]
    UNKNOWN
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RMAlgo {
    pub pos_ref: XYZ,
    pub vel_ref: XYZ,
    pub dop_cone_ang_ref: f64,
}

#[cfg(test)]
mod tests {
    use super::ImageFormation;
    use quick_xml::de::from_str;

    #[test]
    fn test_image_formation() {
        let xml_str = r#"<ImageFormation><RcvChanProc><NumChanProc>1
            </NumChanProc><ChanIndex>1</ChanIndex></RcvChanProc>
            <TxRcvPolarizationProc>V:V</TxRcvPolarizationProc><TStartProc>0
            </TStartProc><TEndProc>0</TEndProc><TxFrequencyProc><MinProc>0
            </MinProc><MaxProc>0</MaxProc></TxFrequencyProc><ImageFormAlgo>
            PFA</ImageFormAlgo><STBeamComp>NO</STBeamComp><ImageBeamComp>NO
            </ImageBeamComp><AzAutofocus>NO</AzAutofocus><RgAutofocus>NO
            </RgAutofocus><Processing><Type>Processing</Type><Applied>true
            </Applied><Parameter name="param">true</Parameter></Processing>
            </ImageFormation>"#;
        assert!(match from_str::<ImageFormation>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
    // TODO: Test RgAzComp, RMA
}
