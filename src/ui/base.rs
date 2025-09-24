use super::FileViewError;
use crate::scan::{FileItem, FileInfos};

pub struct State {
    pub scan_path: String,
    pub save_path: String,
    pub load_path: String,
    pub scan_limit: usize,
    pub file_items: Option<Vec<FileItem>>,
    pub file_view_error: Option<FileViewError>,
    pub file_view_current: usize,
    pub file_view_infos: Vec<FileInfos>
}

impl Default for State {
    fn default() -> Self {
        Self {
            scan_path: String::new(),
            save_path: String::new(),
            load_path: String::new(),
            scan_limit: 10000,
            file_items: None,
            file_view_error: None,
            file_view_current: 0,
            file_view_infos: vec![FileInfos::Name, FileInfos::Created, FileInfos::Modified, FileInfos::Accessed]
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ScanPath(String),
    SavePath(String),
    LoadPath(String),
    ScanPathFileDialog,
    SavePathFileDialog,
    LoadPathFileDialog,
    ConfirmScan,
    ConfirmLoad,
    ClearFileView,
    FileViewCurrent(usize),
}

pub type Element<'a> = iced::Element<'a, Message>;
pub type Container<'a> = iced::widget::Container<'a, Message>;
pub type Button<'a> = iced::widget::Button<'a, Message>;
