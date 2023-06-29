use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{ContentStyle, StyledContent},
};
use tui::{window::Window, InterfaceBuilder};

mod tui;

#[derive(Debug, Clone, Copy)]
struct State {}
#[derive(Debug, Clone)]
enum Message {
    Output(String),
}

fn main() {
    let dead = Arc::new(AtomicBool::new(false));
    {
        let dead = Arc::clone(&dead);
        ctrlc::set_handler(move || dead.store(true, Ordering::Relaxed)).unwrap();
    }

    let mut interface = InterfaceBuilder::new(State {});
    interface
        .add(OutputView::new(0, 0, 0))
        .add(InputView::new(1, 0, 30));
    let mut interface = interface.build().expect("Failed to build interface :(");

    while !interface.dead() && !dead.load(Ordering::Relaxed) {
        interface.update();
        interface.draw();
        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}

struct OutputView {
    messages: Vec<String>,
    height: usize,
    id: usize,
    redraw: bool,
    position: (usize, usize),
}
impl OutputView {
    fn new(id: usize, x: usize, y: usize) -> Self {
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

    fn draw(&self, _selected: bool, _state: &State) -> Vec<StyledContent<String>> {
        self.messages
            .iter()
            .rev()
            .take(self.height)
            .cloned()
            .map(|x| StyledContent::new(ContentStyle::default(), x))
            .collect()
    }

    fn receive_message(&mut self, message: &Message, _selected: bool, _state: &mut State) {
        match message {
            Message::Output(x) => {
                self.messages.push(x.clone());
                self.redraw = true;
            }
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

struct InputView {
    to_send: Vec<String>,
    text: String,
    position: (usize, usize),
    id: usize,
}
impl InputView {
    fn new(id: usize, x: usize, y: usize) -> Self {
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
