use std::{fs::File, io::BufReader};
use std::error::Error;

use mappy::lake_reader::depth_lines::DepthLines;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("LakeWaubesaWaterDepthLines.json")?;

    let reader = BufReader::new(file);

    let data: DepthLines = serde_json::from_reader(reader)?;

    println!("{:#?}", data);

    Ok(())
}
