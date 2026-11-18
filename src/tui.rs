use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::Block};

use crate::key::KeyFile;

#[derive(Clone, Debug, Default)]
pub struct App {
    keyfile: KeyFile,
    should_quit: bool,
}

impl App {
    pub fn new(keyfile: KeyFile) -> Self {
        Self {
            keyfile,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            // terminal.draw(|frame| frame.render_widget("Hello, world!", frame.area()))?;
            terminal.draw(|frame| {
                frame.render_widget(
                    Block::bordered()
                        .title(format!("Gencli for account: {}", self.keyfile.accountId)),
                    frame.area(),
                )
            })?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn handle_events(&mut self) -> Result<()> {
        //if nothing happens do nothing
        if !event::poll(std::time::Duration::from_millis(16))? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }
}
