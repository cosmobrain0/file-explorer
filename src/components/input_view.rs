use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{ContentStyle, StyledContent},
};

use crate::{tui::window::Window, Message, State};

pub struct InputView {
    to_send: Vec<String>,
    text: String,
    position: (usize, usize),
    id: usize,
}
impl InputView {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        Self {
            text: String::new(),
            id,
            position: (x, y),
            to_send: vec![],
        }
    }
}
impl Window<Message, State> for InputView {
    fn draw(&self, _selected: bool, _state: &State) -> Vec<StyledContent<String>> {
        vec![StyledContent::new(
            ContentStyle::default(),
            self.text.clone(),
        )]
    }

    fn id(&self) -> usize {
        self.id
    }

    fn title(&self, _state: &State) -> String {
        String::from("Input Box")
    }

    fn position(&self, _state: &State) -> (usize, usize) {
        self.position
    }

    fn key_input(&mut self, key: KeyEvent, _state: &mut State) {
        if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
            match key.code {
                KeyCode::Char(x) => {
                    self.text += &if key.modifiers.contains(KeyModifiers::SHIFT) {
                        x.to_uppercase().to_string()
                    } else {
                        x.to_string()
                    }
                }
                KeyCode::Enter => {
                    self.to_send.push(self.text.clone());
                    self.text = String::from("");
                }
                KeyCode::Backspace => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        match self.text.rsplit_once(' ') {
                            // TODO: `None` should clear text
                            None => {
                                self.text.clear();
                            }
                            Some((start, _end)) => {
                                // TODO: find a more efficient way which doesn't involve cloning
                                self.text.truncate(start.len());
                            }
                        }
                    } else if !self.text.is_empty() {
                        self.text = self
                            .text
                            .chars()
                            .rev()
                            .skip(1)
                            .collect::<Vec<char>>()
                            .iter()
                            .rev()
                            .collect::<String>();
                    }
                }
                _ => (),
            }
        }
    }

    fn update<'a>(
        &mut self,
        _selected: bool,
        _state: &mut State,
    ) -> (
        Vec<Box<dyn Window<Message, State> + 'a>>,
        Vec<(Message, usize)>,
    ) {
        let mut messages = Vec::with_capacity(self.to_send.len());
        while let Some(line) = self.to_send.pop() {
            messages.push(Message::Output(line));
        }
        (vec![], messages.into_iter().rev().map(|x| (x, 0)).collect())
    }
}
