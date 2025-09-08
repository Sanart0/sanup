use crate::error::SanupResult;
use crossterm::event::{self, Event, KeyCode};
use tui::{Frame, Terminal, backend::Backend};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> SanupResult<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {}
