use std::{error::Error, fmt::Display};

use self::{interface::Interface, window::Window};

pub mod interface;
pub mod window;

#[derive(Debug, Clone)]
pub enum InterfaceBuildError {
    NoWindows,
    InvalidSelection { selected: usize, windows: usize },
    InvalidIds(Vec<usize>),
}
impl Display for InterfaceBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterfaceBuildError::NoWindows => {
                write!(f, "can't build interface without windows")
            }
            InterfaceBuildError::InvalidSelection { selected, windows } => {
                write!(f, "can't select window {selected} out of {windows}")
            }
            InterfaceBuildError::InvalidIds(ids) => {
                write!(f, "multiple windows have the same id {:#?}", ids)
            }
        }
    }
}
impl Error for InterfaceBuildError {}

/// This will probably have more settings later
pub struct InterfaceBuilder<'a, Message, State>
where
    Message: Clone,
{
    windows: Vec<Box<dyn Window<Message, State> + 'a>>,
    selected: Option<usize>,
    state: State,
}
impl<'a, Message, State> InterfaceBuilder<'a, Message, State>
where
    Message: Clone,
{
    pub fn new(state: State) -> Self {
        Self {
            windows: vec![],
            selected: None,
            state,
        }
    }

    pub fn add(&mut self, window: impl Window<Message, State> + 'a) -> &mut Self {
        self.windows.push(Box::new(window));
        self
    }

    pub fn _add_multiple(
        &mut self,
        mut window: Vec<Box<dyn Window<Message, State> + 'a>>,
    ) -> &mut Self {
        self.windows.append(&mut window);
        self
    }

    pub fn select(&mut self, select: Option<usize>) -> &mut Self {
        self.selected = select;
        self
    }

    pub fn build(self) -> Result<Interface<'a, Message, State>, InterfaceBuildError> {
        let selection = self.selected.unwrap_or(0);
        if self.windows.is_empty() {
            Err(InterfaceBuildError::NoWindows)
        } else if selection >= self.windows.len() {
            Err(InterfaceBuildError::InvalidSelection {
                selected: selection,
                windows: self.windows.len(),
            })
        } else if self.windows.iter().enumerate().any(|(i, window)| {
            self.windows
                .iter()
                .enumerate()
                .any(|(j, window2)| i != j && window.id() == window2.id())
        }) {
            Err(InterfaceBuildError::InvalidIds(
                self.windows.iter().map(|x| x.id()).collect(),
            ))
        } else {
            Ok(Interface::new(self.windows, selection, self.state))
        }
    }
}
