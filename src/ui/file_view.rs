use super::base::*;
use super::consts::{ERROR_COLOR, DIR_COLOR};
use crate::scan::{file_info, FileItem, FileType, ScanError};
use iced::widget::{button, horizontal_space, scrollable, text, Column, Row};
use iced::Length;

#[derive(Debug, Clone)]
pub enum FileViewError {
    InvalidScanPath,
    InvalidLoadPath,
    InvalidLoadContent,
    ScanError(ScanError),
    FileIoError(String),
}

pub fn file_view(state: &State) -> Container<'_> {
    if state.file_view_error.is_some() {
        let text = match state.file_view_error.as_ref().unwrap() {
            FileViewError::InvalidScanPath => text("Scan path should be a folder and save path should be a file"),
            FileViewError::InvalidLoadPath => text("Load path should be a file"),
            FileViewError::InvalidLoadContent => text("Invalid content to be loaded"),
            FileViewError::ScanError(error) => match error {
                ScanError::IoError(error) => text(format!("File IO error: {}", error)),
                ScanError::LimitReached => text(format!("Scan limit reached: {}", state.scan_limit)),
            },
            FileViewError::FileIoError(error) => text(format!("File IO error: {}", error)),
        }
        .color(ERROR_COLOR);
        Container::new(text).center(Length::Fill)
    } else if state.file_items.is_none() {
        Container::new(text("File items or error will be printed here")).center(Length::Fill)
    } else {
        let mut vectors = Vec::new();
        for _ in 0..state.file_view_infos.len() {
            vectors.push(Vec::new());
        }
        let items = state.file_items.as_ref().unwrap();
        let id = state.file_view_current;
        let range = items[id].childs().unwrap();
        let mut part: Vec<_> = items[range].iter().collect();
        part.sort_by(|itema, itemb| cmp_by_type(itema, itemb));
        if let Some(parent) = items[id].parent() {
            vectors[0].push(dir_element("..".to_owned(), parent));
            for i in 1..state.file_view_infos.len() {
                vectors[i].push(text("").into());
            }
        } else if part.is_empty() {
            return Container::new(text("No items")).center(Length::Fill);
        };
        for item in part {
            for i in 0..state.file_view_infos.len() {
                let info = file_info(item, &state.file_view_infos[i]);
                match item.file_type() {
                    FileType::Dir => vectors[i].push(dir_element(info, item.id())),
                    _ => vectors[i].push(text(info).into())
                }
            }
        }
        let columns: Vec<_> = vectors
            .into_iter()
            .map(|vector| Column::from_vec(vector.into_iter().map(|widget| Element::from(widget)).collect()).padding(10))
            .collect();
        let elements = columns.into_iter().map(|column| Element::from(column)).collect();
        let scroll = scrollable(Row::from_vec(elements).push(horizontal_space()));
        Container::new(scroll).height(Length::Fill).clip(true)
    }
}

fn cmp_by_type(itema: &FileItem, itemb: &FileItem) -> std::cmp::Ordering {
    match (itema.is_dir(), itemb.is_dir()) {
        (true, true) => itema.name().cmp(&itemb.name()),
        (false, false) => {
            let (namea, exta) = itema.name().rsplit_once('.').unwrap_or((itema.name(), ""));
            let (nameb, extb) = itemb.name().rsplit_once('.').unwrap_or((itemb.name(), ""));
            exta.cmp(extb).then_with(|| namea.cmp(nameb))
        }
        (boola, boolb) => boola.cmp(&boolb).reverse(),
    }
}

fn dir_element(content: String, target: usize) -> Element<'static> {
    button(text(content).color(DIR_COLOR))
        .style(button::text)
        .padding(0)
        .on_press(Message::FileViewCurrent(target))
        .into()
}
