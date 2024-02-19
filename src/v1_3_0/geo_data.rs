use super::LL;
pub use crate::dep::v0_4_0::geo_data::{
    Desc, EarthModel, ImageCorners, Line, Polygon, ValidDataLL, SCP,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GeoData {
    pub earth_model: EarthModel,
    #[serde(rename = "SCP")]
    pub scp: SCP,
    pub image_corners: ImageCorners,
    pub valid_data: Option<ValidDataLL>,
    pub geo_info: Option<Vec<GeoInfo>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GeoInfo {
    #[serde(rename = "@name")]
    pub name: String,
    pub desc: Option<Vec<Desc>>,
    pub point: Option<LL>,
    pub line: Option<Line>,
    pub polygon: Option<Polygon>,
    pub geo_info: Option<Vec<GeoInfo>>,
}

#[cfg(test)]
mod tests {
    use super::{Desc, GeoData};
    use quick_xml::de::from_str;
    #[test]
    fn test_geo_data() {
        let xml_str = r#"<GeoData><EarthModel>WGS_84</EarthModel>
            <SCP><ECF><X>0</X><Y>0</Y><Z>0</Z></ECF><LLH><Lat>0</Lat><Lon>0
            </Lon><HAE>0</HAE></LLH></SCP><ImageCorners><ICP index="1:FRFC">
            <Lat>0</Lat><Lon>0</Lon></ICP><ICP index="2:FRLC"><Lat>0</Lat><Lon>
            0</Lon></ICP><ICP index="3:LRLC"><Lat>0</Lat><Lon>0</Lon></ICP>
            <ICP index="4:LRFC"><Lat>0</Lat><Lon>0</Lon></ICP></ImageCorners>
            <ValidData size="1"><Vertex index="1"><Lat>0</Lat><Lon>0</Lon>
            </Vertex></ValidData></GeoData>"#;
        assert!(match from_str::<GeoData>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
    #[test]
    fn test_empty_desc() {
        let xml_str = r#"<Desc name="foo">    </Desc>"#;
        assert!(match from_str::<Desc>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
