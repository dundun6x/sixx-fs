mod file_item;
mod file_info;
mod file_times;
mod specific_info;
pub use file_item::{FileItem, FileType};
pub use file_info::{FileInfo, info_string};
pub use file_times::FileTimes;
pub use specific_info::{DirInfo, RegularInfo, SpecificInfo, SymlinkInfo};

use file_item::FileItemBuilder;
use std::{collections::HashMap, path::{Path, PathBuf}};

#[derive(Clone, Debug)]
pub enum ScanError {
    FileIoError(String),
    LimitReached
}

pub fn scan(scan_path: &Path, scan_limit: usize) -> Result<Vec<FileItem>, ScanError> {
    let mut bfs = vec![0];
    let mut scan = vec![FileItemBuilder::new(0, scan_path.to_path_buf())];
    let mut count = 0;
    let mut inaccessible = 0;
    let mut path_to_id: HashMap<PathBuf, usize> = HashMap::new();
    while !bfs.is_empty() {
        let id = bfs.pop().unwrap();
        let read = std::fs::read_dir(&scan[id].path);
        if read.is_err() {
            let err = read.unwrap_err();
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                inaccessible += 1;
                scan[id].info(SpecificInfo::Inaccessible(FileType::Dir));
                continue;
            } else {
                return Err(ScanError::FileIoError(err.to_string()));
            }
        }
        let dir = read.unwrap();
        let before = count;
        for entry in dir {
            if entry.is_err() {
                return Err(ScanError::FileIoError(entry.unwrap_err().to_string()));
            }
            let path = entry.unwrap().path();
            count += 1;
            if count >= scan_limit {
                return Err(ScanError::LimitReached);
            }
            let child_id = count;
            path_to_id.insert(path.clone(), child_id);
            if path.is_symlink() {
                scan.push(FileItemBuilder::new(child_id, path));
                scan[child_id].info(SpecificInfo::Symlink(SymlinkInfo::new(None)))
            } else if path.is_dir() {
                bfs.push(child_id);
                scan.push(FileItemBuilder::new(child_id, path));
            } else {
                let info = match std::fs::read(&path) {
                    Err(err) => if err.kind() == std::io::ErrorKind::PermissionDenied {
                        inaccessible += 1;
                        SpecificInfo::Inaccessible(FileType::Regular)
                    } else {
                        return Err(ScanError::FileIoError(err.to_string()));
                    }
                    Ok(content) => {
                        let md5 = format!("{:?}", md5::compute(content));
                        SpecificInfo::Regular(RegularInfo::new(md5))
                    }
                };
                scan.push(FileItemBuilder::new(child_id, path));
                scan[child_id].info(info);
            }
            scan[child_id].parent(id);
        }
        let after = count;
        scan[id].info(SpecificInfo::Dir(DirInfo::new(before + 1, after)));
    }
    for item in &mut scan {
        if let Some(SpecificInfo::Symlink(_)) = item.info {
            let new_info = match std::fs::read_link(&item.path) {
                Ok(path) => SpecificInfo::Symlink(SymlinkInfo::new(path_to_id.get(&path).cloned())),
                Err(_) => SpecificInfo::Inaccessible(FileType::Symlink)
            };
            item.info(new_info);
        }
    }
    let info = scan.into_iter().map(|builder| builder.build()).collect();
    log::info!("Inaccessible items: {}", inaccessible);
    Ok(info)
}
