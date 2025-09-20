pub mod inputfield;
pub mod inputfieldtype;
pub mod inputform;

use crate::{
    error::SanupResult,
    sanup::{Sanup, state::SanupState},
};
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    prelude::Backend,
    style::{Style, Stylize},
    widgets::{Block, Clear, Tabs},
};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: Sanup) -> SanupResult<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            app.on_key(key);
        }
    }
}

fn ui(f: &mut Frame, app: &Sanup) {
    let area = f.area();
    let vertical_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
    let [tabs_area, body_area] = vertical_layout.areas(area);

    let tabs = Tabs::new(SanupState::into_vec_str())
        .block(Block::bordered().title("Sanup"))
        .style(Style::default().white())
        .highlight_style(Style::default().light_green())
        .select(app.state.into_idx())
        .divider(" ");

    f.render_widget(tabs, tabs_area);

    f.render_widget(Clear, body_area);
}

fn create_backup(f: &mut Frame, app: &Sanup) {}
