use std::{fs::File, io::BufReader};
use std::error::Error;

use mappy::lake_reader::depth_lines::DepthLines;

fn main() -> Result<(), Box<dyn Error>> {
    let mercator = File::open("LakeWaubesaWaterDepthMercator.json")?;
    let local = File::open("LakeWaubesaWaterDepthLines.json")?;

    let reader_merc = BufReader::new(mercator);
    let reader_local = BufReader::new(local);

    let mut mercator: DepthLines = serde_json::from_reader(reader_merc)?;
    let local: DepthLines = serde_json::from_reader(reader_local)?;

    mercator.merge_depths(&local);
    mercator.to_lat_lon();

    let out = File::create_new("waubesa-depth.json")?;
    serde_json::to_writer(out, &mercator)?;

    Ok(())
}
