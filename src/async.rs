use super::types::{DataSet, Instances};
use anyhow::Result;
use async_std::{
    fs::File,
    io::{prelude::*, BufReader},
};
use std::path::Path;

impl DataSet {
    pub async fn load_async<P>(dir: P, name: &str) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let image_dir = dir.join(name);
        let annotation_dir = dir.join("annotations");

        let instances_file = annotation_dir.join(format!("instances_{}.json", name));
        let instances: Instances = {
            let mut reader = BufReader::new(File::open(&instances_file).await?);
            let mut text = String::new();
            reader.read_to_string(&mut text).await?;
            async_std::task::spawn_blocking(move || serde_json::from_str(&text)).await?
        };

        let dataset = Self {
            instances,
            image_dir,
            instances_file,
        };

        Ok(dataset)
    }
}
