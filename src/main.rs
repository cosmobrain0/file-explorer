use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use components::{DirectoryView, InputView, OutputView};
use tui::InterfaceBuilder;

mod components;
mod tui;

#[derive(Debug, Clone, Copy)]
struct State {}
#[derive(Debug, Clone)]
enum Message {
    Output(String),
    FileList(Vec<String>),
}

fn main() {
    let dead = Arc::new(AtomicBool::new(false));
    {
        let dead = Arc::clone(&dead);
        ctrlc::set_handler(move || dead.store(true, Ordering::Relaxed)).unwrap();
    }

    let mut interface = InterfaceBuilder::new(State {});
    interface
        .add(DirectoryView::new(
            0,
            0,
            0,
            env::current_dir().unwrap(),
            1,
            2,
        ))
        .add(OutputView::new(2, 0, 20));
    let mut interface = interface.build().expect("Failed to build interface :(");

    while !interface.dead() && !dead.load(Ordering::Relaxed) {
        interface.update();
        interface.draw();
        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}
