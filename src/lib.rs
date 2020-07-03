use anyhow::{anyhow, ensure, Result};
use itertools::izip;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageMeta {
    pub path: PathBuf,
    pub height: usize,
    pub width: usize,
}

pub fn load<P>(dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    let prefixes = ["train2017", "test-dev2017", "val2017"];

    for prefix in prefixes.iter().cloned() {
        let list_file = dir.join(format!("{}.txt", prefix));
        let shape_file = dir.join(format!("{}.shapes", prefix));
        let metas = load_from_list_shape(list_file, shape_file)?;
    }

    Ok(())
}

fn load_from_list_shape<P1, P2>(list_file: P1, shape_file: P2) -> Result<Vec<ImageMeta>>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let list_file = list_file.as_ref();
    let base_dir = list_file
        .parent()
        .ok_or(anyhow!("invalid list file {:?}", list_file))?;

    let paths_iter = BufReader::new(File::open(list_file)?)
        .lines()
        .map(|result| -> Result<_> {
            let line = result?;
            let path = base_dir.join(PathBuf::from(line));
            Ok(path)
        });
    let shapes_iter = BufReader::new(File::open(shape_file)?)
        .lines()
        .map(|result| -> Result<_> {
            let line = result?;
            let mut token_iter = line.split_whitespace();
            let width: usize = token_iter.next().ok_or(anyhow!(""))?.parse()?;
            let height: usize = token_iter.next().ok_or(anyhow!(""))?.parse()?;
            ensure!(token_iter.next().is_none(), "");
            Ok((height, width))
        });

    let meta_iter =
        izip!(paths_iter, shapes_iter).map(|(path_result, shape_result)| -> Result<_> {
            let path = path_result?;
            let (height, width) = shape_result?;
            let meta = ImageMeta {
                path,
                height,
                width,
            };
            Ok(meta)
        });

    meta_iter.collect()
}
