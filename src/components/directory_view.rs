use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    time::Instant,
};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind},
    style::{Color, ContentStyle, StyledContent, Stylize},
};

use crate::{
    tui::{interface::Colour, window::Window},
    Message, State,
};

// TODO: there's a lot of stuff going
// on here. Maybe this should be broken
// down? I kinda feel like `State`
// should be doing some more work here.

const UPDATE_TIME: u128 = 60 * 1000; // ms
pub struct DirectoryView {
    position: (usize, usize),
    root: PathBuf,
    folders: Vec<PathBuf>,
    files: Vec<PathBuf>,
    redraw: bool,
    previous_update: Instant,
    id: usize,
    file_view_id: usize,
    output_view_id: usize,
    selected_directory: Option<usize>,
    messages: Vec<(Message, usize)>,
}
impl DirectoryView {
    pub fn new(
        id: usize,
        x: usize,
        y: usize,
        root: PathBuf,
        file_view_id: usize,
        output_view_id: usize,
    ) -> Self {
        let mut view = Self {
            id,
            position: (x, y),
            root,
            folders: vec![],
            files: vec![],
            redraw: true,
            file_view_id,
            previous_update: Instant::now(),
            selected_directory: None,
            output_view_id,
            messages: vec![],
        };
        view.load_data();
        view
    }

    fn load_data(&mut self) {
        for item in read_directory(&self.root) {
            match item.file_type() {
                Err(_) => (),
                Ok(x) => {
                    if x.is_dir() {
                        &mut self.folders
                    } else {
                        &mut self.files
                    }
                    .push(item.path());
                }
            }
        }
        self.redraw = true;
        self.selected_directory = match (
            self.folders.len() > 0,
            self.selected_directory
                .is_some_and(|x| x < self.folders.len()),
            self.selected_directory,
        ) {
            (false, _, _) => None,
            (_, true, _) => self.selected_directory,
            (_, false, None) => Some(0),
            (_, false, _) => Some(self.folders.len() - 1),
        };
        self.previous_update = Instant::now();
    }

    fn set_root(&mut self, root: PathBuf) {
        self.root = root;
        self.files.clear();
        self.folders.clear();
        self.load_data();
    }

    fn name(&self) -> String {
        self.root.file_name().unwrap().to_str().unwrap().to_string()
    }
}
impl Window<Message, State> for DirectoryView {
    fn requires_redraw(&self, _state: &State) -> bool {
        self.redraw
    }
    fn draw(
        &self,
        _selected: bool,
        _statee: &State,
    ) -> Vec<crossterm::style::StyledContent<String>> {
        let mut lines = Vec::with_capacity(self.folders.len() + 1);
        lines.push(StyledContent::new(ContentStyle::default(), self.name()));

        let foreground: Colour = (0.75, 0.75, 0.75).into();
        let background: Colour = (0.0, 0.0, 0.0).into();
        let style_highlight = |x: String| x.on(foreground.into()).with(background.into());
        let style_normal = |x: String| x.on(background.into()).with(foreground.into());

        // TODO: make this iter through folders
        for (i, folder) in self.folders.iter().enumerate() {
            let highlighted = self.selected_directory.is_some_and(|x| x == i);
            let data = format!(
                "  {}",
                folder.file_name().unwrap().to_str().unwrap().to_string()
            );
            lines.push(if highlighted {
                style_highlight(data)
            } else {
                style_normal(data)
            });
        }
        lines
    }
    fn redrawn(&mut self, _selected: bool, _state: &mut State) {
        self.redraw = false;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn title(&self, _state: &State) -> String {
        String::from("Directory View")
    }

    fn position(&self, _state: &State) -> (usize, usize) {
        self.position
    }

    fn update<'a>(
        &mut self,
        _selected: bool,
        _state: &mut State,
    ) -> (
        Vec<Box<dyn Window<Message, State> + 'a>>,
        Vec<(Message, usize)>,
    ) {
        if self.previous_update.elapsed().as_millis() >= UPDATE_TIME {
            self.set_root(self.root.clone()); // TODO: fix this: this is silly
            self.messages.push((
                Message::FileList(
                    self.files
                        .iter()
                        .map(|x| x.to_str().unwrap().to_string())
                        .collect(),
                ),
                self.file_view_id,
            ));
        }
        let messages = self.messages.clone();
        self.messages.clear();
        (vec![], messages)
    }

    fn key_input(&mut self, key: KeyEvent, _state: &mut State) {
        if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                    if self.selected_directory.is_none() {
                        return;
                    }
                    self.redraw = true;
                    let selected_directory = self.selected_directory.unwrap();
                    if selected_directory > 0 {
                        self.selected_directory = Some(selected_directory - 1);
                    } else {
                        self.selected_directory = Some(self.folders.len() - 1);
                    }
                }
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                    if self.selected_directory.is_none() {
                        return;
                    }
                    self.redraw = true;
                    let selected_directory = self.selected_directory.unwrap();
                    if selected_directory < self.folders.len() - 1 {
                        self.selected_directory = Some(selected_directory + 1);
                    } else {
                        self.selected_directory = Some(0);
                    }
                }
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                    self.set_root(self.folders[self.selected_directory.unwrap()].clone());
                }
                KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => {
                    match self.root.as_path().parent() {
                        None => self.messages.push((
                            Message::Output(String::from("This path has no parent")),
                            self.output_view_id,
                        )), // TODO: make this output an error message
                        Some(x) => {
                            self.set_root(x.to_path_buf());
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn read_directory(root: &PathBuf) -> Vec<DirEntry> {
    let items = match fs::read_dir(root) {
        Err(_) => vec![],
        Ok(values) => values.collect(),
    };
    items
        .into_iter()
        .filter(|x| match x {
            &Err(_) => false,
            Ok(_) => true,
        })
        .map(|x| x.unwrap())
        .collect()
}
