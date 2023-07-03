use std::path::PathBuf;

use crossterm::style::{Color, Stylize};

use crate::{
    tui::window::{DrawData, Window},
    Message, State,
};

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
    fn draw(&self, _selected: bool, _state: &State) -> DrawData {
        let (width, height) = crossterm::terminal::size().unwrap();
        let (width, height): (usize, usize) = (width.into(), height.into());
        DrawData::with_strings(
            self.files
                .iter()
                .map(|x| x.file_name().unwrap().to_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            0,
            height - height / 2 - 2,
            width / 2 - 2,
        )
    }

    fn requires_redraw(&self, _state: &State) -> bool {
        self.redraw
    }

    fn id(&self) -> usize {
        self.id
    }

    fn title(&self, _state: &State) -> String {
        String::from("Files View")
    }

    fn position(&self, _state: &State) -> (usize, usize) {
        let height: usize = crossterm::terminal::size().unwrap().1.into();
        (0, height - height / 2)
    }

    fn receive_message(&mut self, message: &Message, _selected: bool, _state: &mut State) {
        match message {
            Message::FileList(list) => {
                self.files = list.clone();
                self.redraw = true;
            }
            _ => (),
        }
    }

    fn redrawn(&mut self, _selected: bool, _state: &mut State) {
        self.redraw = false;
    }
}
