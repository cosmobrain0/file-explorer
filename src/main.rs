use tui::InterfaceBuilder;

mod tui;

#[derive(Debug, Clone, Copy)]
struct State {}

fn main() {
    let mut builder = InterfaceBuilder::new(State {});
}
