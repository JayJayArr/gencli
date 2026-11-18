use anyhow::Result;
use crossterm::event;
use ratatui::DefaultTerminal;

use crate::key::KeyFile;

#[derive(Clone, Debug)]
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
                    format!("Running gencli for System: {}", self.keyfile.accountId),
                    frame.area(),
                )
            })?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn handle_events(&mut self) -> Result<()> {
        if event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }
}

// fn render(frame: &mut Frame) {
//     frame.render_widget("hello world", frame.area());
// }
