mod file_item;
mod file_info;

pub use file_item::{FileItem, FileType};
pub use file_info::{FileInfos, file_info};

use file_item::{DirInfo, FileItemBuilder, RegularInfo, SpecificInfo, SymlinkInfo};
use std::path::Path;

#[derive(Clone, Debug)]
pub enum ScanError {
    IoError(String),
    LimitReached
}

pub fn scan(scan_path: &Path, scan_limit: usize) -> Result<Vec<FileItem>, ScanError> {
    let mut bfs = vec![0];
    let mut scan = vec![FileItemBuilder::new(0, scan_path.to_path_buf())];
    let mut count = 0;
    let mut inaccessible = 0;
    while !bfs.is_empty() {
        let id = bfs.pop().unwrap();
        let read = std::fs::read_dir(&scan[id].path);
        if read.is_err() {
            let error = read.unwrap_err();
            if error.kind() == std::io::ErrorKind::PermissionDenied {
                inaccessible += 1;
                scan[id].info(SpecificInfo::Inaccessible(FileType::Dir));
                continue;
            } else {
                return Err(ScanError::IoError(error.to_string()));
            }
        }
        let dir = read.unwrap();
        let before = count;
        for entry in dir {
            if entry.is_err() {
                return Err(ScanError::IoError(entry.unwrap_err().to_string()));
            }
            count += 1;
            if count >= scan_limit {
                return Err(ScanError::LimitReached);
            }
            let child_id = count;
            let path = entry.unwrap().path();
            if path.is_symlink() {
                scan.push(FileItemBuilder::new(child_id, path));
                scan[child_id].info(SpecificInfo::Symlink(SymlinkInfo::new(None)));
            } else if path.is_dir() {
                bfs.push(child_id);
                scan.push(FileItemBuilder::new(child_id, path));
            } else {
                let info = match std::fs::read(&path) {
                    Err(error) => if error.kind() == std::io::ErrorKind::PermissionDenied {
                        inaccessible += 1;
                        SpecificInfo::Inaccessible(FileType::Regular)
                    } else {
                        return Err(ScanError::IoError(error.to_string()));
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
    let info = scan.into_iter().map(|builder| builder.build()).collect();
    log::info!("Inaccessible items: {}", inaccessible);
    Ok(info)
}
