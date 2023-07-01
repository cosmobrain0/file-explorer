use std::path::PathBuf;

use crossterm::style::{Color, Stylize};

use crate::{tui::window::Window, Message, State};

pub struct DirFilesView {
    files: Vec<PathBuf>,
    redraw: bool,
    position: (usize, usize),
    id: usize,
}
impl DirFilesView {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        Self {
            files: vec![],
            redraw: false,
            position: (x, y),
            id,
        }
    }
}
impl Window<Message, State> for DirFilesView {
    fn draw(
        &self,
        _selected: bool,
        _state: &State,
    ) -> Vec<crossterm::style::StyledContent<String>> {
        self.files
            .iter()
            .map(|x| {
                x.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .on(Color::Black)
            })
            .collect()
    }

    fn requires_redraw(&self, _state: &State) -> bool {
        true
    }

    fn id(&self) -> usize {
        self.id
    }

    fn title(&self, _state: &State) -> String {
        String::from("Files View")
    }

    fn position(&self, _state: &State) -> (usize, usize) {
        self.position
    }

    fn receive_message(&mut self, message: &Message, _selected: bool, _state: &mut State) {
        match message {
            Message::FileList(list) => {
                self.files = list.clone();
            }
            _ => (),
        }
    }
}
