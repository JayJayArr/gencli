use anyhow::Result;
use crossterm::event;
use ratatui::{DefaultTerminal, text::Line, widgets::Widget};

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

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Line::raw(format!(
            "Running gencli for System: {}",
            self.keyfile.accountId
        ))
        .render(area, buf);
    }
}

// fn render(frame: &mut Frame) {
//     frame.render_widget("hello world", frame.area());
// }
//

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    use super::App;

    #[test]
    fn test_render_app() {
        let app = App::default();
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}
