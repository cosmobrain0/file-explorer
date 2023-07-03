use crossterm::style::{ContentStyle, StyledContent};

use crate::{
    tui::window::{DrawData, Window},
    Message, State,
};

pub struct OutputView {
    messages: Vec<String>,
    height: usize,
    id: usize,
    redraw: bool,
    position: (usize, usize),
}
impl OutputView {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        Self {
            messages: vec![String::from("hi"), String::from("there")],
            height: 6,
            id,
            redraw: true,
            position: (x, y),
        }
    }
}
impl Window<Message, State> for OutputView {
    fn requires_removal(&self, _state: &State) -> bool {
        false
    }

    fn requires_redraw(&self, _state: &State) -> bool {
        self.redraw
    }

    fn draw(&self, _selected: bool, _state: &State) -> DrawData {
        DrawData::with_strings(
            self.messages.clone(),
            self.messages.len().saturating_sub(self.height),
            self.height,
            crossterm::terminal::size().unwrap().1 as usize / 2 - 2,
        )
    }

    fn receive_message(&mut self, message: &Message, _selected: bool, _state: &mut State) {
        match message {
            Message::Output(x) => {
                self.messages.push(x.clone());
                self.redraw = true;
            }
            _ => (),
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn title(&self, _state: &State) -> String {
        String::from("Output View")
    }

    fn position(&self, _state: &State) -> (usize, usize) {
        self.position
    }

    fn redrawn(&mut self, _selected: bool, _state: &mut State) {
        self.redraw = false;
    }
}
