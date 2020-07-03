use anyhow::Result;
use argh::FromArgs;
use coco::DataSet;
use std::path::PathBuf;

#[derive(Debug, Clone, FromArgs)]
/// COCO data set inspector
struct Args {
    #[argh(positional)]
    /// data set directory
    pub dataset_dir: PathBuf,
    #[argh(positional)]
    /// data set name
    pub name: String,
}

fn main() -> Result<()> {
    let Args { dataset_dir, name } = argh::from_env();
    let dataset = DataSet::load(&dataset_dir, &name)?;
    println!("{} images found", dataset.image_paths.len());
    Ok(())
}
