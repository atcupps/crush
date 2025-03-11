use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, style::{Style, Stylize}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame};

pub struct App {
    counter: u8,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            counter: 0,
            exit: false
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => { }
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => { self.exit = true },
            KeyCode::Left => {
                self.counter = self.counter.saturating_sub(1)
            },
            KeyCode::Right => {
                self.counter = self.counter.saturating_add(1)
            },
            _ => { }
        };
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let title = Line::from(vec![
            Span::styled(" Counter App ", Style::new().bold())
        ]);

        let keys_style = Style::new().blue().bold();
        let instructions = Line::from(vec![
            Span::raw(" Decrement "),
            Span::styled("<Left>", keys_style),
            Span::raw("  Increment "),
            Span::styled("<Right>", keys_style),
            Span::raw("  Quit "),
            Span::styled("<q> ", keys_style),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::DOUBLE);

        let counter_text = Text::from(vec![Line::from(vec![
            Span::raw("Value: "),
            Span::raw(format!("{}", self.counter)),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();
    app_result
}