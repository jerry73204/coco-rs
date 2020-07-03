use anyhow::Result;
use argh::FromArgs;
use std::path::PathBuf;

#[derive(Debug, Clone, FromArgs)]
/// COCO data set inspector
struct Args {
    #[argh(positional)]
    /// data set directory
    pub dataset_dir: PathBuf,
}

fn main() -> Result<()> {
    let Args { dataset_dir } = argh::from_env();

    coco::load(&dataset_dir)?;

    Ok(())
}
