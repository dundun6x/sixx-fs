use super::FileItem;
use std::time::SystemTime;

pub enum FileInfos {
    Name,
    Created,
    Modified,
    Accessed,
    Media(String)
}

pub fn file_info(item: &FileItem, name: &FileInfos) -> String {
    match name {
        FileInfos::Name => item.name().to_string(),
        FileInfos::Created => stringify_time(item.times().created.clone()),
        FileInfos::Modified => stringify_time(item.times().modified.clone()),
        FileInfos::Accessed => stringify_time(item.times().accessed.clone()),
        FileInfos::Media(_) => "Not supported yet".to_owned()
    }
}

fn stringify_time(time: Option<SystemTime>) -> String {
    time.map(|time| chrono::DateTime::<chrono::Local>::from(time)
        .format("%Y/%m/%d %H:%M:%S").to_string())
        .unwrap_or_default()
}