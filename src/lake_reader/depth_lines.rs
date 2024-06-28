use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DepthLines {
    displayFieldName: String,
    fieldAliases: FieldAliases,
    geometryType: String,
    spatialReference: SpatialReference,
    fields: Vec<Field>,
    features: Vec<Feature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldAliases {
    OBJECTID: String,
    WaterDepth: String,
    Source: String,
    SourceDate: String,
    Equipment: String,
    Shape_Length: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpatialReference {
    wkid: u32,
    latestWkid: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
    alias: String,
    length: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    attributes: Attributes,
    geometry: Geometry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attributes {
    OBJECTID: u32,
    WaterDepth: f32,
    Source: String,
    SourceDate: String,
    Equipment: String,
    Shape_Length: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    paths: Vec<Vec<(f64, f64)>>,
}
