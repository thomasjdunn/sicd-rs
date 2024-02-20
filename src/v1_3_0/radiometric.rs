use super::Poly2D;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Radiometric {
    #[serde(default)]
    pub noise_level: NoiseLevel,
    #[serde(rename = "RCSSFPoly")]
    #[serde(default)]
    pub rcssf_poly: Poly2D,
    #[serde(rename = "SigmaZeroSFPoly")]
    #[serde(default)]
    pub sigma_zero_sf_poly: Poly2D,
    #[serde(rename = "BetaZeroSFPoly")]
    #[serde(default)]
    pub beta_zero_sf_poly: Poly2D,
    #[serde(rename = "GammaZeroSFPoly")]
    #[serde(default)]
    pub gamma_zero_sf_poly: Poly2D,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NoiseLevel {
    pub noise_level_type: NoiseLevelType,
    pub noise_poly: Poly2D,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub struct NoiseLevelType {
    #[serde(rename = "$text")]
    pub value: NoiseLevelTypeEnum,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
pub enum NoiseLevelTypeEnum {
    #[default]
    ABSOLUTE,
    RELATIVE,
}
