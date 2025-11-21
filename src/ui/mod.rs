pub mod inputfield;
pub mod inputfieldtype;
pub mod inputform;

use crate::{
    app::{Sanup, tabs::SanupTabs},
    error::SanupResult,
};
use log::info;
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
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            app.on_key(key);
        }
    }
}

fn ui(f: &mut Frame, app: &mut Sanup) {
    let area = f.area();
    let vertical_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
    let [tabs_area, body_area] = vertical_layout.areas(area);

    let tabs = Tabs::new(SanupTabs::into_vec_str())
        .block(Block::bordered().title(app.title))
        .style(Style::default().white())
        .highlight_style(Style::default().green())
        .select(app.tabs.into_idx())
        .divider(" ");

    f.render_widget(tabs, tabs_area);

    info!("APP FOCUS: {}", app.focus);
    if app.input_form.is_active() {
        f.render_widget(&mut app.input_form, body_area);
        f.set_cursor_position(app.input_form.cursor_position());
        info!("ACTIVE");
    } else if app.input_form.is_cancelled() {
        info!("CANCELLED");
    } else if app.focus.is_body() {
        app.body_text = format!("{:?}", std::time::Instant::now());
        f.render_widget(app.body_text.clone(), body_area);
    }
}
