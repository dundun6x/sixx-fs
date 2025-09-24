pub mod base;
pub mod file_view;
pub mod consts;

pub use base::*;
pub use file_view::FileViewError;

use self::file_view::file_view;
use crate::scan::{FileItem, scan};
use iced::{
    Alignment, Length, Task,
    widget::{button, column, container, horizontal_rule, horizontal_space, row, text, text_input},
};
use std::path::Path;

pub fn setup() -> iced::Result {
    iced::application("File Info Scanner", update, view)
        .window(iced::window::Settings {
            size: iced::Size::new(600., 150.),
            ..iced::window::Settings::default()
        })
        .run()
}

fn view(state: &State) -> Element<'_> {
    let file_view = file_view(state);
    let top = container(column![
        row![
            text("Scan at:").width(80),
            text_input("", &state.scan_path).on_input(Message::ScanPath),
            button("Choose").on_press(Message::ScanPathFileDialog)
        ]
        .align_y(Alignment::Center),
        row![
            text("Save to:").width(80),
            text_input("", &state.save_path).on_input(Message::SavePath),
            button("Choose").on_press(Message::SavePathFileDialog)
        ]
        .align_y(Alignment::Center),
        row![
            text("Load from:").width(80),
            text_input("", &state.load_path).on_input(Message::LoadPath),
            button("Choose").on_press(Message::LoadPathFileDialog)
        ]
        .align_y(Alignment::Center),
    ])
    .align_top(Length::Shrink);
    let middle = file_view;
    let bottom = container(row![
        horizontal_space(),
        button("Scan").on_press(Message::ConfirmScan),
        button("Load").on_press(Message::ConfirmLoad),
        button("Clear").on_press(Message::ClearFileView)
    ])
    .align_bottom(Length::Shrink);
    column![top, horizontal_rule(2), middle, horizontal_rule(2), bottom].into()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ScanPath(path) => {
            state.scan_path = path;
            Task::none()
        }
        Message::SavePath(path) => {
            state.save_path = path;
            Task::none()
        }
        Message::LoadPath(path) => {
            state.load_path = path;
            Task::none()
        }
        Message::ScanPathFileDialog => Task::future(rfd::AsyncFileDialog::new().pick_folder()).then(|handle| match handle {
            Some(handle) => Task::done(Message::ScanPath(handle.path().to_str().unwrap().to_owned())),
            None => Task::none(),
        }),
        Message::SavePathFileDialog => Task::future(rfd::AsyncFileDialog::new().pick_file()).then(|handle| match handle {
            Some(handle) => Task::done(Message::SavePath(handle.path().to_str().unwrap().to_owned())),
            None => Task::none(),
        }),
        Message::LoadPathFileDialog => Task::future(rfd::AsyncFileDialog::new().add_filter("Json", &["json"]).pick_file()).then(|handle| match handle {
            Some(handle) => Task::done(Message::LoadPath(handle.path().to_str().unwrap().to_owned())),
            None => Task::none(),
        }),
        Message::ConfirmScan => {
            clear_file_view(state);
            let scan_path = Path::new(&state.scan_path);
            let save_path = Path::new(&state.save_path);
            match scan_path.is_dir() && save_path.is_file() {
                false => state.file_view_error = Some(FileViewError::InvalidScanPath),
                true => match scan(&scan_path, state.scan_limit) {
                    Err(error) => state.file_view_error = Some(FileViewError::ScanError(error)),
                    Ok(file_items) => match std::fs::write(save_path, serde_json::to_string(&file_items).unwrap()) {
                        Err(error) => state.file_view_error = Some(FileViewError::FileIoError(error.to_string())),
                        Ok(_) => state.file_items = Some(file_items),
                    },
                },
            }
            Task::none()
        }
        Message::ConfirmLoad => {
            clear_file_view(state);
            let load_path = Path::new(&state.load_path);
            match load_path.is_file() {
                false => state.file_view_error = Some(FileViewError::InvalidLoadPath),
                true => match std::fs::read(load_path) {
                    Err(error) => state.file_view_error = Some(FileViewError::FileIoError(error.to_string())),
                    Ok(content) => match String::from_utf8(content) {
                        Err(_) => state.file_view_error = Some(FileViewError::InvalidLoadContent),
                        Ok(string) => match serde_json::from_str::<Vec<FileItem>>(&string) {
                            Err(_) => state.file_view_error = Some(FileViewError::InvalidLoadContent),
                            Ok(file_items) => state.file_items = Some(file_items),
                        },
                    },
                },
            }
            Task::none()
        }
        Message::ClearFileView => {
            clear_file_view(state);
            Task::none()
        }
        Message::FileViewCurrent(id) => {
            state.file_view_current = id;
            Task::none()
        }
    }
}

fn clear_file_view(state: &mut State) {
    state.file_items = None;
    state.file_view_error = None;
    state.file_view_current = 0;
}
