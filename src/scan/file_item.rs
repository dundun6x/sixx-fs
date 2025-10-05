use super::FileTimes;
use super::specific_info::SpecificInfo;
use serde::{Deserialize, Serialize};
use std::{ops::RangeInclusive, path::PathBuf};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Dir,
    Symlink,
}

#[derive(Serialize, Deserialize)]
pub struct FileItem {
    id: usize,
    name: String,
    parent: Option<usize>,
    info: SpecificInfo,
    times: FileTimes,
}

impl FileItem {
    pub fn id(&self) -> usize {
        self.id.clone()
    }
    pub fn parent(&self) -> Option<usize> {
        self.parent.clone()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn times(&self) -> &FileTimes {
        &self.times
    }
    pub fn file_type(&self) -> FileType {
        match &self.info {
            SpecificInfo::Regular(_) => FileType::Regular,
            SpecificInfo::Dir(_) => FileType::Dir,
            SpecificInfo::Symlink(_) => FileType::Symlink,
            SpecificInfo::Inaccessible(file_type) => file_type.clone(),
        }
    }
    pub fn is_dir(&self) -> bool {
        self.file_type() == FileType::Dir
    }
    pub fn childs(&self) -> Option<RangeInclusive<usize>> {
        match &self.info {
            SpecificInfo::Dir(dir) => Some(dir.childs.clone()),
            _ => None,
        }
    }
}

pub struct FileItemBuilder {
    pub id: usize,
    pub path: PathBuf,
    pub parent: Option<usize>,
    pub info: Option<SpecificInfo>,
}

impl FileItemBuilder {
    pub fn new(id: usize, path: PathBuf) -> Self {
        Self {
            id: id,
            path: path,
            parent: None,
            info: None,
        }
    }
    pub fn parent(&mut self, parent: usize) {
        self.parent = Some(parent);
    }
    pub fn info(&mut self, info: SpecificInfo) {
        self.info = Some(info);
    }
    pub fn build(self) -> FileItem {
        let name = self.path.file_name().unwrap().to_str().unwrap().to_owned();
        let times = self.path.metadata().ok().map(|meta| FileTimes::from(meta)).unwrap_or_default();
        FileItem {
            id: self.id,
            name: name,
            parent: self.parent,
            info: self.info.expect("[FileItemBuilder] `info` is required field"),
            times: times,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlainFileItem {
    id: usize,
    path: String,
    file_type: FileType,
    times: FileTimes,
}
