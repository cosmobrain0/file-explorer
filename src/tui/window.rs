use crossterm::{event::KeyEvent, style::StyledContent};

pub trait Window<Message, State>
where
    Message: Clone,
{
    fn requires_removal(&self, state: &State) -> bool {
        false
    }
    fn requires_redraw(&self, state: &State) -> bool {
        true
    }
    fn draw(&self, selected: bool, state: &State) -> Vec<StyledContent<String>>;
    /// Returns (new_windows, Vec<(message, recipient_id)>)
    fn update<'a>(
        &mut self,
        selected: bool,
        state: &mut State,
    ) -> (
        Vec<Box<dyn Window<Message, State> + 'a>>,
        Vec<(Message, usize)>,
    ) {
        (vec![], vec![])
    }
    fn receive_message(&mut self, message: &Message, selected: bool, state: &mut State) {}
    fn id(&self) -> usize;
    fn title(&self, state: &State) -> String;
    fn position(&self, state: &State) -> (usize, usize);
    fn key_input(&mut self, key: KeyEvent, state: &mut State) {}
    fn redrawn(&mut self, selected: bool, state: &mut State) {}
}
