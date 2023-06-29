use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use components::{InputView, OutputView};
use tui::InterfaceBuilder;

mod components;
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
