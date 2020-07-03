use crate::common::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataSet {
    pub instances: Instances,
    pub image_dir: PathBuf,
    pub instances_file: PathBuf,
}

impl DataSet {
    pub fn load<P>(dir: P, name: &str) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        use std::{fs::File, io::BufReader};

        let dir = dir.as_ref();
        let image_dir = dir.join(name);
        let annotation_dir = dir.join("annotations");

        let instances_file = annotation_dir.join(format!("instances_{}.json", name));
        let instances: Instances =
            serde_json::from_reader(BufReader::new(File::open(&instances_file)?))?;

        let dataset = Self {
            instances,
            image_dir,
            instances_file,
        };

        Ok(dataset)
    }

    #[cfg(feature = "async")]
    pub async fn load_async<P>(dir: P, name: &str) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        use async_std::{
            fs::File,
            io::{prelude::*, BufReader},
        };

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
    #[serde(with = "serde_iscrowd")]
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
    #[serde(with = "serde_date_captured")]
    pub date_captured: NaiveDateTime,
    pub file_name: String,
    #[serde(with = "serde_url")]
    pub coco_url: Url,
    #[serde(with = "serde_url")]
    pub flickr_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Info {
    pub contributor: String,
    #[serde(with = "serde_date_created")]
    pub date_created: NaiveDate,
    pub description: String,
    #[serde(with = "serde_url")]
    pub url: Url,
    pub version: String,
    pub year: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct License {
    pub id: usize,
    pub name: String,
    #[serde(with = "serde_url")]
    pub url: Url,
}

mod serde_url {
    use super::*;

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        url.as_str().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let url = Url::parse(&text)
            .map_err(|err| D::Error::custom(format!("failed to parse url: {:?}", err)))?;
        Ok(url)
    }
}

mod serde_date_captured {
    use super::*;

    pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2013-11-14 12:36:29
        format!("{}", datetime.format("%Y-%m-%d %H:%M:%S")).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let datetime =
            NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").map_err(|err| {
                D::Error::custom(format!(
                    "failed to parse the value of date_captured: {:?}",
                    err
                ))
            })?;
        Ok(datetime)
    }
}

mod serde_date_created {
    use super::*;

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2017/09/01
        format!("{}", date.format("%Y/%m/%d")).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let date = NaiveDate::parse_from_str(&text, "%Y/%m/%d").map_err(|err| {
            D::Error::custom(format!(
                "failed to parse the value of date_created: {:?}",
                err
            ))
        })?;
        Ok(date)
    }
}

mod serde_iscrowd {
    use super::*;

    pub fn serialize<S>(yes: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *yes { 1 } else { 0 }.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let yes = usize::deserialize(deserializer)? != 0;
        Ok(yes)
    }
}
