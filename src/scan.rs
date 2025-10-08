mod file_item;
mod file_info;
mod file_times;
mod specific_info;
pub use file_item::{FileItem, FileType};
pub use file_info::{FileInfo, info_string};
pub use file_times::FileTimes;
pub use specific_info::{DirInfo, RegularInfo, SpecificInfo, SymlinkInfo};

use file_item::FileItemBuilder;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, path::{Path, PathBuf}};

#[derive(Serialize, Deserialize)]
pub struct Scan {
    pub items: Vec<FileItem>,
    pub description: String,
    pub warning: String
}

pub struct ScanSettings {
    pub scan_limit: Option<usize>,
    pub ffsm: bool
}

impl Default for ScanSettings {
    fn default() -> Self {
        Self { 
            scan_limit: None, 
            ffsm: false
        }
    }
}

pub fn scan(scan_path: &Path, settings: &ScanSettings) -> Result<Scan, String> {
    let mut bfs = vec![0];
    let mut items = vec![FileItemBuilder::new(0, scan_path.to_path_buf())];
    let mut count = 0;
    let mut path_to_id: HashMap<PathBuf, usize> = HashMap::new();
    // Message related
    let mut inaccessible = 0;
    let mut limit_reached = false;
    while !bfs.is_empty() {
        let id = bfs.pop().unwrap();
        let read = std::fs::read_dir(&items[id].path);
        if read.is_err() {
            let err = read.unwrap_err();
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                inaccessible += 1;
                items[id].info(SpecificInfo::Inaccessible(FileType::Dir));
                continue;
            } else {
                return Err(err.to_string());
            }
        }
        let dir = read.unwrap();
        let before = count;
        for entry in dir {
            if entry.is_err() {
                return Err(entry.unwrap_err().to_string());
            }
            let path = entry.unwrap().path();
            count += 1;
            if settings.scan_limit.is_some_and(|limit| count >= limit) {
                limit_reached = true;
                break;
            }
            let child_id = count;
            path_to_id.insert(path.clone(), child_id);
            if path.is_symlink() {
                items.push(FileItemBuilder::new(child_id, path));
                items[child_id].info(SpecificInfo::Symlink(SymlinkInfo { target: None }));
            } else if path.is_dir() {
                bfs.push(child_id);
                items.push(FileItemBuilder::new(child_id, path));
            } else {
                let info = match std::fs::read(&path) {
                    Err(err) => if err.kind() == std::io::ErrorKind::PermissionDenied {
                        inaccessible += 1;
                        SpecificInfo::Inaccessible(FileType::Regular)
                    } else {
                        return Err(err.to_string());
                    }
                    Ok(content) => {
                        let md5 = format!("{:?}", md5::compute(content));
                        let metas = if !settings.ffsm { 
                            Vec::new()
                        } else {
                            ez_ffmpeg::container_info::get_metadata(path.to_str().unwrap().to_owned()).unwrap_or_default()
                        };
                        SpecificInfo::Regular(RegularInfo { md5, metas })
                    }
                };
                items.push(FileItemBuilder::new(child_id, path));
                items[child_id].info(info);
            }
            items[child_id].parent(id);
        }
        let after = count;
        items[id].info(SpecificInfo::Dir(DirInfo::new(before + 1, after)));
    }
    for item in &mut items {
        if let Some(SpecificInfo::Symlink(_)) = item.info {
            let new_info = match std::fs::read_link(&item.path) {
                Ok(path) => SpecificInfo::Symlink(SymlinkInfo {
                    target: path_to_id.get(&path).cloned()
                }),
                Err(_) => SpecificInfo::Inaccessible(FileType::Symlink)
            };
            item.info(new_info);
        }
    }
    let items = items.into_iter().map(|builder| builder.build()).collect();
    let mut warning = String::new();
    if limit_reached != false {
        warning += "Limit reached. ";
    }
    if inaccessible != 0 {
        warning += &format!("Inaccessible items: {}. ", inaccessible);
    }
    let description = format!("FFSM {}. ", if settings.ffsm { "on" } else { "off" });
    log::info!("{}", warning);
    Ok(Scan { items, warning, description })
}
