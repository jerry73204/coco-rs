use anyhow::Result;
use clap::Parser;
use coco::DataSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
/// COCO data set inspector
struct Opts {
    /// data set directory
    pub dataset_dir: PathBuf,
    /// data set name
    pub name: String,
}

#[async_std::main]
pub async fn main() -> Result<()> {
    let Opts { dataset_dir, name } = Opts::parse();
    let dataset = DataSet::load_async(&dataset_dir, &name).await?;
    println!("{} images found", dataset.instances.images.len());
    Ok(())
}
