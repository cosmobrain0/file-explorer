use crossterm::{
    event::KeyEvent,
    style::{ContentStyle, StyledContent},
};

pub trait Window<Message, State>
where
    Message: Clone,
{
    fn requires_removal(&self, _state: &State) -> bool {
        false
    }
    fn requires_redraw(&self, _state: &State) -> bool {
        true
    }
    fn draw(&self, selected: bool, state: &State) -> DrawData;
    fn update<'a>(
        &mut self,
        _selected: bool,
        _state: &mut State,
    ) -> UpdateData<'a, Message, State> {
        UpdateData::default()
    }
    fn receive_message(&mut self, _message: &Message, _selected: bool, _state: &mut State) {}
    fn id(&self) -> usize;
    fn title(&self, state: &State) -> String;
    fn position(&self, state: &State) -> (usize, usize);
    fn key_input(&mut self, _key: KeyEvent, _state: &mut State) {}
    fn redrawn(&mut self, _selected: bool, _state: &mut State) {}
}

pub struct UpdateData<'a, Message, State> {
    pub new_windows: Vec<Box<dyn Window<Message, State> + 'a>>,
    pub new_messages: Vec<(Message, usize)>,
}
impl<'a, Message, State> UpdateData<'a, Message, State> {
    pub fn new(
        new_windows: Vec<Box<dyn Window<Message, State> + 'a>>,
        new_messages: Vec<(Message, usize)>,
    ) -> Self {
        Self {
            new_windows,
            new_messages,
        }
    }
}
impl<'a, Message, State> Default for UpdateData<'a, Message, State> {
    fn default() -> Self {
        Self {
            new_windows: vec![],
            new_messages: vec![],
        }
    }
}

pub struct DrawData {
    pub data: Vec<StyledContent<String>>,
    pub scroll: usize,
    pub height: usize,
    pub width: usize,
}
impl DrawData {
    pub fn new(
        data: Vec<StyledContent<String>>,
        scroll: usize,
        height: usize,
        width: usize,
    ) -> Self {
        Self {
            data,
            scroll,
            height,
            width,
        }
    }

    pub fn with_strings(data: Vec<String>, scroll: usize, height: usize, width: usize) -> Self {
        Self {
            data: data
                .into_iter()
                .map(|x| StyledContent::new(ContentStyle::default(), x))
                .collect(),
            scroll,
            height,
            width,
        }
    }
}
impl From<Vec<StyledContent<String>>> for DrawData {
    fn from(data: Vec<StyledContent<String>>) -> Self {
        Self {
            scroll: 0,
            height: data.len(),
            width: data.iter().map(|x| x.content().len()).max().unwrap_or(0),
            data,
        }
    }
}
impl From<Vec<String>> for DrawData {
    fn from(data: Vec<String>) -> Self {
        Self {
            scroll: 0,
            height: data.len(),
            width: data.iter().map(|x| x.len()).max().unwrap_or(0),
            data: data
                .into_iter()
                .map(|x| StyledContent::new(ContentStyle::default(), x))
                .collect(),
        }
    }
}
