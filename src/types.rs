use chrono::{NaiveDate, NaiveDateTime};
use noisy_float::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataSet {
    pub instances: Instances,
    pub image_dir: PathBuf,
    pub instances_file: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Instances {
    pub annotations: Vec<Annotation>,
    pub categories: Vec<Category>,
    pub images: Vec<Image>,
    pub info: Info,
    pub licenses: Vec<License>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Annotation {
    pub area: R64,
    pub bbox: [R64; 4],
    pub category_id: usize,
    pub id: usize,
    pub image_id: usize,
    #[serde(with = "crate::utils::serde_iscrowd")]
    pub iscrowd: bool,
    pub segmentation: Segmentation,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub supercategory: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Segmentation {
    Verbose {
        counts: Vec<usize>,
        size: [usize; 2],
    },
    Simple(Vec<Vec<R64>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Image {
    pub id: usize,
    pub license: usize,
    pub height: usize,
    pub width: usize,
    #[serde(with = "crate::utils::serde_date_captured")]
    pub date_captured: NaiveDateTime,
    pub file_name: String,
    #[serde(with = "crate::utils::serde_url")]
    pub coco_url: Url,
    #[serde(with = "crate::utils::serde_url")]
    pub flickr_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Info {
    pub contributor: String,
    #[serde(with = "crate::utils::serde_date_created")]
    pub date_created: NaiveDate,
    pub description: String,
    #[serde(with = "crate::utils::serde_url")]
    pub url: Url,
    pub version: String,
    pub year: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct License {
    pub id: usize,
    pub name: String,
    #[serde(with = "crate::utils::serde_url")]
    pub url: Url,
}
