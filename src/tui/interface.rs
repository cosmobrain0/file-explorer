use std::io::{Stdout, Write};

use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Color, PrintStyledContent, StyledContent, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use super::window::Window;

struct WindowData<'a, Message: Clone, State> {
    window: Box<dyn Window<Message, State> + 'a>,
    size: Option<(usize, usize)>,
    id: usize,
}
impl<'a, Message: Clone, State> WindowData<'a, Message, State> {
    fn new(window: Box<dyn Window<Message, State> + 'a>, id: usize) -> Self {
        Self {
            window,
            id,
            size: None,
        }
    }
}

pub struct Interface<'a, Message: Clone, State> {
    windows: Vec<WindowData<'a, Message, State>>,
    selected: usize,
    requires_redraw: bool,
    messages: Vec<(Message, usize)>,
    state: State,
}
impl<'a, Message: Clone, State> Interface<'a, Message, State> {
    pub fn new(
        mut windows: Vec<Box<dyn Window<Message, State> + 'a>>,
        selected: usize,
        state: State,
    ) -> Self {
        execute!(std::io::stdout(), EnterAlternateScreen).unwrap();
        execute!(std::io::stdout(), DisableBlinking, Hide).unwrap();
        let mut window_data = Vec::with_capacity(windows.len());
        while let Some(x) = windows.pop() {
            let id = x.id();
            window_data.insert(0, WindowData::new(x, id));
        }
        Self {
            windows: window_data,
            selected,
            requires_redraw: true,
            messages: vec![],
            state,
        }
    }

    pub fn update(&mut self) {
        if self.dead() {
            return;
        }
        for message in self.messages.iter() {
            let (message, id) = message;
            match self
                .windows
                .iter_mut()
                .enumerate()
                .find(|(_, x)| x.id == *id)
            {
                None => (),
                Some((i, x)) => {
                    x.window
                        .receive_message(message, self.selected == i, &mut self.state);
                }
            }
        }
        self.messages.clear();

        while event::poll(std::time::Duration::from_secs(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Tab && key.modifiers == KeyModifiers::empty() {
                    if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
                        if self.selected + 1 >= self.windows.len() {
                            self.selected = 0;
                        } else {
                            self.selected += 1;
                        }
                    }
                    self.requires_redraw = true;
                } else {
                    self.windows[self.selected]
                        .window
                        .key_input(key, &mut self.state);
                    self.remove_dead_windows();
                }
            }
        }

        let mut new_windows = vec![];
        self.windows.iter_mut().enumerate().for_each(|(i, x)| {
            let (windows, messages) = {
                let updated = x.window.update(i == self.selected, &mut self.state);
                (updated.new_windows, updated.new_messages)
            };
            new_windows.extend(windows);
            self.messages.extend(messages);
        });
        self.windows.extend(
            new_windows
                .into_iter()
                .map(|x| (x.id(), x))
                .map(|(id, x)| WindowData::new(x, id)),
        );
        self.remove_dead_windows();
    }

    fn remove_dead_windows(&mut self) {
        for i in (0..self.windows.len()).rev() {
            if self.windows[i].window.requires_removal(&self.state) {
                self.windows.remove(i);
                if self.selected >= i && i > 0 {
                    // TODO: make this cycle properly
                    // without crashing
                    self.selected -= 1;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        if self.dead() {
            return;
        }
        let mut stdout = std::io::stdout();
        if !(self.requires_redraw
            || self
                .windows
                .iter()
                .any(|x| x.window.requires_redraw(&self.state)))
        {
            return;
        }
        self.requires_redraw = false;
        for (_, selected, window) in self
            .windows
            .iter_mut()
            .enumerate()
            .map(|(i, window)| (i, i == self.selected, window))
        {
            if !self.requires_redraw && !window.window.requires_redraw(&self.state) {
                continue;
            }
            let data = window.window.draw(selected, &self.state);
            let ideal_height = data.height;
            let ideal_width = data.width;
            let mut data: Vec<_> = data
                .data
                .into_iter()
                .skip(data.scroll)
                .take(ideal_height)
                .map(|x| {
                    StyledContent::new(x.style().clone(), {
                        let mut result: String = x.content().chars().take(ideal_width).collect();
                        result += &" ".repeat(ideal_width - result.len());
                        result
                    })
                })
                .collect();
            for i in data.len()..ideal_height {
                data.push(StyledContent::new(
                    crossterm::style::ContentStyle::default(),
                    " ".repeat(ideal_width),
                ));
            }
            let (x, y) = window.window.position(&self.state);
            let (width, height) = (
                data.iter()
                    .map(|x| x.content().chars().count())
                    .max()
                    .unwrap_or(0)
                    + 2,
                data.len() + 2,
            );

            let (previous_width, previous_height) = window.size.unwrap_or((width, height));

            let border_colour = if selected {
                (1.0, 1.0, 1.0)
            } else {
                (0.3, 0.3, 0.3)
            };

            draw_bordered_rect(
                &mut stdout,
                x,
                y,
                width,
                height,
                border_colour.into(),
                Colour {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                },
            );
            for (i, line) in data.iter().enumerate() {
                draw_styled_text(&mut stdout, x + 1, y + i + 1, line, width - 2);
            }

            // draw_text(
            //     &mut stdout,
            //     x,
            //     y,
            //     format!(
            //         "{upper_left}{row}{upper_right}",
            //         row = horizontal.repeat(width - 2)
            //     ),
            //     width,
            //     border_colour.into(),
            //     (0.0, 0.0, 0.0).into(),
            // );
            // for (i, line) in data.iter().enumerate() {
            //     draw_text(
            //         &mut stdout,
            //         x,
            //         y + i + 1,
            //         &format!(
            //             "{vertical}{padding}{vertical}",
            //             padding = " ".repeat(width - 2)
            //         ),
            //         width,
            //         border_colour.into(),
            //         (0.0, 0.0, 0.0).into(),
            //     );
            //     draw_text(
            //         &mut stdout,
            //         x + 1,
            //         y + i + 1,
            //         &format!("{line:width$}", width = width - 2),
            //         width - 2,
            //         (0.75, 0.75, 0.75).into(),
            //         (0.0, 0.0, 0.0).into(),
            //     )
            // }
            // draw_text(
            //     &mut stdout,
            //     x,
            //     y + 1 + data.len(),
            //     // &format!("╭{row}╮", row = "━".repeat(width - 2)),
            //     &format!(
            //         "{lower_left}{row}{lower_right}",
            //         row = "\u{2500}".repeat(width - 2)
            //     ),
            //     width,
            //     border_colour.into(),
            //     (0.0, 0.0, 0.0).into(),
            // );
            window.window.redrawn(selected, &mut self.state);

            if previous_width > width {
                // draw a rect (x+width, y) (previous_width-width, previous_height)
                draw_rect(
                    &mut stdout,
                    x + width,
                    y,
                    previous_width - width,
                    previous_height,
                    (0.0, 0.0, 0.0).into(),
                );
            }
            if previous_height > height {
                // draw a rect (x, y+height) (previous_width, previous_height-height)
                draw_rect(
                    &mut stdout,
                    x,
                    y + height,
                    previous_width,
                    previous_height - height,
                    (0.0, 0.0, 0.0).into(),
                );
            }
            window.size = Some((width, height));
        }
        stdout.flush().unwrap();
    }

    pub fn dead(&self) -> bool {
        self.windows.is_empty()
    }
}

impl<'a, Message: std::clone::Clone, State> Drop for Interface<'a, Message, State> {
    fn drop(&mut self) {
        execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
    }
}

fn draw_styled_text(
    stdout: &mut Stdout,
    x: usize,
    y: usize,
    text: &StyledContent<String>,
    width: usize,
) {
    let mut content = format!("{:width$}", text.content());
    content = content.chars().take(width).collect();
    let styled = StyledContent::new(text.style().clone(), content);
    queue!(
        stdout,
        MoveTo(x as u16, y as u16),
        PrintStyledContent(styled)
    )
    .unwrap();
}

/// # Panics
/// Panics if `height < 0` or `width < 2`
fn draw_bordered_rect(
    stdout: &mut Stdout,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    colour: Colour,
    background: Colour,
) {
    let upper_left = "\u{250C}".to_string();
    let upper_right = "\u{2510}".to_string();
    let lower_left = "\u{2514}".to_string();
    let lower_right = "\u{2518}".to_string();
    let horizontal = "\u{2500}".to_string();
    let vertical = "\u{2502}".to_string();
    let _title_left = "\u{2524}".to_string();
    let _title_right = "\u{251C}".to_string();
    draw_styled_text(
        stdout,
        x,
        y,
        &format!(
            "{upper_left}{row}{upper_right}",
            row = horizontal.repeat(width - 2)
        )
        .on(background.into())
        .with(colour.into()),
        width,
    );

    let middle = format!(
        "{vertical}{padding}{vertical}",
        padding = " ".repeat(width - 2)
    )
    .on(background.into())
    .with(colour.into());
    for y in y + 1..y + (height - 1) {
        draw_styled_text(stdout, x, y, &middle, width);
    }

    draw_styled_text(
        stdout,
        x,
        y + height - 1,
        &format!(
            "{lower_left}{row}{lower_right}",
            row = horizontal.repeat(width - 2)
        )
        .on(background.into())
        .with(colour.into()),
        width,
    );
}

fn draw_rect(stdout: &mut Stdout, x: usize, y: usize, width: usize, height: usize, colour: Colour) {
    for y in y..y + height {
        draw_styled_text(
            stdout,
            x,
            y,
            &"".to_string().on(colour.into()).with(colour.into()),
            width,
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    r: f32,
    g: f32,
    b: f32,
}
impl From<(f32, f32, f32)> for Colour {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}
impl From<Colour> for Color {
    fn from(val: Colour) -> Self {
        Color::Rgb {
            r: (val.r * 255.0) as u8,
            g: (val.g * 255.0) as u8,
            b: (val.b * 255.0) as u8,
        }
    }
}
