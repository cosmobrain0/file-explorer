use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossterm::style::{ContentStyle, StyledContent};
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
    interface.add(OutputView::new(0, 0, 0));
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
            messages: vec![],
            height: 1,
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
            Message::Output(x) => self.messages.push(x.clone()),
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
