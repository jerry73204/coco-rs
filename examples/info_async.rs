use anyhow::Result;

#[cfg(feature = "async")]
mod example {
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

    pub async fn main() -> Result<()> {
        let Args { dataset_dir, name } = argh::from_env();
        let dataset = DataSet::load_async(&dataset_dir, &name).await?;
        println!("{} images found", dataset.image_paths.len());
        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_std::main]
async fn main() -> Result<()> {
    example::main().await
}

#[cfg(not(feature = "async"))]
fn main() {
    panic!(r#"please enable "async" feature to run this example"#);
}
