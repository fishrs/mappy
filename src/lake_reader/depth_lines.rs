use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DepthLines {
    #[serde(rename = "displayFieldName")]
    field_name: String,
    #[serde(rename = "geometryType")]
    geometry_type: String,
    features: Vec<Feature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    attributes: Attributes,
    geometry: Geometry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attributes {
    #[serde(rename = "OBJECTID")]
    id: u32,
    #[serde(rename = "WaterDepth")]
    water_depth: f32,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "SourceDate")]
    source_date: String,
    #[serde(rename = "Equipment")]
    equipment: String,
    #[serde(rename = "Shape_Length")]
    shape_length: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    paths: Vec<Vec<(f64, f64)>>,
}
