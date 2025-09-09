use crate::{
    error::SanupResult,
    sanup::{Sanup, state::SanupState},
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout}, prelude::Backend, style::{Style, Stylize}, symbols::block, widgets::{Block, Paragraph, Tabs}, Frame, Terminal
};

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

    let body_block = Block::bordered().style(Style::default().white());

    f.render_widget(body_block, body_area);

}
