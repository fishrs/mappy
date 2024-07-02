use std::f64::consts::PI;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DepthLines {
    #[serde(rename = "displayFieldName")]
    field_name: String,
    #[serde(rename = "geometryType")]
    geometry_type: String,
    features: Vec<Feature>,
}

impl DepthLines {
    pub fn to_lat_lon(&mut self) {
        for feature in &mut self.features {
            feature.to_lat_lon();
        }
    }

    pub fn merge_depths(&mut self, other: &Self) {
        if self.features.len() != other.features.len() {
            panic!("Will make this an error later but I'm lazy rn");
        }

        for i in 0..self.features.len() {
            self.features[i].attributes = other.features[i].attributes.clone();
        }
    }
}



#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    attributes: Attributes,
    geometry: Geometry,
}

impl Feature {
    pub fn to_lat_lon(&mut self) {
        self.geometry.to_lat_lon()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Attributes {
    #[serde(rename = "OBJECTID")]
    id: u32,
    #[serde(rename = "WaterDepth")]
    water_depth: Option<f32>,
    #[serde(rename = "Source")]
    source: Option<String>,
    #[serde(rename = "SourceDate")]
    source_date: Option<String>,
    #[serde(rename = "Equipment")]
    equipment: Option<String>,
    #[serde(rename = "Shape_Length")]
    shape_length: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    paths: Vec<Vec<(f64, f64)>>,
}

impl Geometry {
    pub fn to_lat_lon(&mut self) {
        for path in &mut self.paths {
            for mercator in path {
                let res = Self::mercator_to_lat_lon(mercator.clone());
                mercator.0 = res.0;
                mercator.1 = res.1;
            }
       }
    }

    fn mercator_to_lat_lon(mercator: (f64, f64)) -> (f64, f64) {
        let (x, y) = mercator;

        let lon = x / 20037508.34 * 180.0;

        // Convert y to latitude
        let lat_rad = y / 20037508.34 * PI;
        let lat = 180.0 / PI * (2.0 * lat_rad.exp().atan() - PI / 2.0);

        (lon, lat)
    }
}
