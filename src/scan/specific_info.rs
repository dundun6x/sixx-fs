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
    pub md5: String
}

impl RegularInfo {
    pub fn new(md5: String) -> Self {
        Self {
            md5: md5
        }
    }
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

impl SymlinkInfo {
    pub fn new(target: Option<usize>) -> Self {
        Self {
            target: target
        }
    }
}