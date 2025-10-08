use super::base::*;
use super::consts::{ERROR_COLOR, DIR_COLOR};
use crate::scan::{info_string, FileItem, FileType};
use iced::widget::{button, horizontal_space, scrollable, text, column, Column, Row};
use iced::Length;

#[derive(Debug, Clone)]
pub enum FileViewError {
    InvalidScanPath,
    InvalidLoadPath,
    InvalidLoadContent,
    ScanLimitReached,
    FileIoError(String),
}

pub fn scan_view(state: &State) -> Element<'_> {
    if state.file_view_error.is_some() {
        let text = match state.file_view_error.as_ref().unwrap() {
            FileViewError::InvalidScanPath => text("Scan path should be a folder and save path should be a file"),
            FileViewError::InvalidLoadPath => text("Load path should be a file"),
            FileViewError::InvalidLoadContent => text("Invalid content to be loaded"),
            FileViewError::ScanLimitReached => text(format!("Scan limit reached: {}", state.scan_settings.scan_limit.unwrap())),
            FileViewError::FileIoError(err) => text(format!("File IO error: {}", err)),
        }
        .color(ERROR_COLOR);
        Container::new(text).center(Length::Fill).into()
    } else if state.scan.is_none() {
        Container::new(text("File items or error will be printed here")).center(Length::Fill).into()
    } else {
        let mut cols = Vec::new();
        for _ in 0..state.file_view_infos.len() { cols.push(Vec::new()); }
        let scan = state.scan.as_ref().unwrap();
        let items = &scan.items;
        let curr = state.file_view_current;
        let range = items[curr].childs().unwrap();
        let mut items_view: Vec<_> = items[range].iter().collect();
        items_view.sort_by(|itema, itemb| cmp_by_type(itema, itemb));
        if let Some(parent) = items[curr].parent() {
            cols[0].push(dir_element("..".to_owned(), parent));
            for i in 1..state.file_view_infos.len() {
                cols[i].push(text("").into());
            }
        }
        let file_view = if items[curr].parent().is_none() && items_view.is_empty() {
            Container::new(text("No items")).center(Length::Fill)
        } else {
            for item in items_view {
                for i in 0..state.file_view_infos.len() {
                    let info = info_string(item, &state.file_view_infos[i]);
                    match item.file_type() {
                        FileType::Dir => cols[i].push(dir_element(info, item.id())),
                        _ => cols[i].push(text(info).wrapping(text::Wrapping::None).into())
                    }
                }
            }
            let elems: Vec<_> = cols.into_iter().map(|col| Element::from(
                Column::from_vec(col).padding(5).clip(true)
            )).collect();
            let scroll = scrollable(Row::from_vec(elems).push(horizontal_space()));
            Container::new(scroll).height(Length::Fill).clip(true)
        };
        column![file_view]
            .push_maybe((&scan.warning != "").then(|| text(&scan.warning).color(ERROR_COLOR)))
            .push(text(&scan.description))
            .into()
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
    button(text(content).wrapping(text::Wrapping::None).color(DIR_COLOR))
        .style(button::text)
        .padding(0)
        .on_press(Message::FileViewCurrent(target))
        .into()
}
