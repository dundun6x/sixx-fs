use super::FileType;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Serialize, Deserialize)]
pub enum SpecificInfo {
    Inaccessible(FileType),
    Regular(RegularInfo),
    Dir(DirInfo),
    Symlink(SymlinkInfo)
}

#[derive(Serialize, Deserialize)]
pub struct RegularInfo {
    pub md5: String,
    pub metas: Vec<(String, String)>
}

#[derive(Serialize, Deserialize)]
pub struct DirInfo {
    pub childs: RangeInclusive<usize>,
}

impl DirInfo {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            childs: start..=end
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SymlinkInfo {
    pub target: Option<usize>
}