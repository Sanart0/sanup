use crate::{error::SanupResult, sanup::Sanup};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::Backend, Frame, Terminal};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: Sanup) -> SanupResult<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char(c) => app.on_key(c),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &Sanup) {
    let size = f.size();
}
