pub mod base;
pub mod consts;

mod scan_view;
pub use scan_view::FileViewError;

use base::*;
use scan_view::scan_view;
use crate::scan::{scan, Scan};
use iced::{
    Alignment, Length, Task,
    widget::{button, column, container, horizontal_rule, horizontal_space, row, text, text_input},
};
use std::path::Path;

pub fn setup() -> iced::Result {
    iced::application("File Info Scanner", update, view)
        .settings(iced::Settings {
            fonts: vec![include_bytes!("../assets/SourceHanSansSC-Regular.otf").into()],
            default_font: iced::Font::with_name("Source Han Sans SC"),
            default_text_size: iced::Pixels(13.),
            ..Default::default()
        })
        .window(iced::window::Settings {
            size: iced::Size::new(600., 150.),
            ..iced::window::Settings::default()
        })
        .run()
}

fn view(state: &State) -> Element<'_> {
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
    let scan_view = scan_view(state);
    let bottom = container(row![
        horizontal_space(),
        button("Scan").on_press(Message::ConfirmScan),
        button("Load").on_press(Message::ConfirmLoad),
        button("Clear").on_press(Message::ClearFileView)
    ])
    .align_bottom(Length::Shrink);
    column![top, horizontal_rule(2), scan_view, horizontal_rule(2), bottom].into()
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
            confirm_scan(state);
            Task::none()
        }
        Message::ConfirmLoad => {
            clear_file_view(state);
            confirm_load(state);
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
    state.scan = None;
    state.file_view_error = None;
    state.file_view_current = 0;
}

fn confirm_scan(state: &mut State) {
    let scan_path = Path::new(&state.scan_path);
    let save_path = Path::new(&state.save_path);
    match scan_path.is_dir() && save_path.is_file() {
        false => state.file_view_error = Some(FileViewError::InvalidScanPath),
        true => match scan(&scan_path, &state.scan_settings) {
            Err(err) => state.file_view_error = Some(FileViewError::FileIoError(err)),
            Ok(scan) => match std::fs::write(save_path, serde_json::to_string(&scan).unwrap()) {
                Err(err) => state.file_view_error = Some(FileViewError::FileIoError(err.to_string())),
                Ok(_) => state.scan = Some(scan),
            }
        },
    }
}

fn confirm_load(state: &mut State) {
    let load_path = Path::new(&state.load_path);
    match load_path.is_file() {
        false => state.file_view_error = Some(FileViewError::InvalidLoadPath),
        true => match std::fs::read(load_path) {
            Err(err) => state.file_view_error = Some(FileViewError::FileIoError(err.to_string())),
            Ok(content) => match String::from_utf8(content) {
                Err(_) => state.file_view_error = Some(FileViewError::InvalidLoadContent),
                Ok(string) => match serde_json::from_str::<Scan>(&string) {
                    Err(_) => state.file_view_error = Some(FileViewError::InvalidLoadContent),
                    Ok(scan) => state.scan = Some(scan),
                },
            },
        },
    }
}