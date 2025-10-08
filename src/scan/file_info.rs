use super::{FileItem, FileType};

pub enum FileInfo {
    Name,
    Type,
    Created,
    Modified,
    Accessed,
    Md5,
    Media(String)
}

pub fn info_string(item: &FileItem, name: &FileInfo) -> String {
    match name {
        FileInfo::Name => item.name().to_owned(),
        FileInfo::Type => match item.file_type() {
            FileType::Regular => "Regular",
            FileType::Dir => "Directory",
            FileType::Symlink => "Symbol link"
        }.to_owned(),
        FileInfo::Created => stringify_time(item.times().created),
        FileInfo::Modified => stringify_time(item.times().modified),
        FileInfo::Accessed => stringify_time(item.times().accessed),
        FileInfo::Md5 => item.md5().unwrap_or_default(),
        FileInfo::Media(name) => item.meta(name).unwrap_or_default()
    }
}

fn stringify_time(time: Option<i64>) -> String {
    time.map(|time| chrono::DateTime::from_timestamp_nanos(time)
        .format("%Y/%m/%d %H:%M:%S").to_string())
        .unwrap_or_default()
}