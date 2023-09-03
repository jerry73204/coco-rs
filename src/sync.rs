use super::types::DataSet;
use crate::types::Instances;
use anyhow::Result;
use std::{fs::File, io::BufReader, path::Path};

impl DataSet {
    pub fn load<P>(dir: P, name: &str) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let image_dir = dir.join(name);
        let annotation_dir = dir.join("annotations");

        let instances_file = annotation_dir.join(format!("instances_{}.json", name));
        let reader = BufReader::new(File::open(&instances_file)?);
        let instances: Instances = serde_json::from_reader(reader)?;

        let dataset = Self {
            instances,
            image_dir,
            instances_file,
        };

        Ok(dataset)
    }
}
