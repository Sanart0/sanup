pub mod input;

use crate::{
    app::{sanup::Sanup, tabs::SanupTabs},
    error::SanupResult,
};
use log::info;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
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

    tabs(f, app, tabs_area);

    if app.tabs.is_settings() {
        settings_tab(f, app, body_area);
    }

    if app.input_form.is_active() {
        f.render_widget(&mut app.input_form, body_area);
        f.set_cursor_position(app.input_form.cursor_position());
    } else if app.focus.is_body() {
        f.render_widget(Clear, body_area);
    }
}

fn tabs(f: &mut Frame, app: &mut Sanup, tabs_area: Rect) {
    let tabs = Tabs::new(SanupTabs::into_vec_str())
        .block(Block::bordered().title(app.title))
        .style(Style::default().white())
        .highlight_style(Style::default().green())
        .select(app.tabs.into_idx())
        .divider(" ");

    f.render_widget(tabs, tabs_area);
}

fn settings_tab(f: &mut Frame, app: &mut Sanup, body_area: Rect) {
    // let items = Settings::field_names()
    //     .iter()
    //     .map(|s| ListItem::new(*s))
    //     .collect();
    //
    // let var_list = List::new(items)
    //     .block(Block::bordered().title("Variables"))
    //     .style(Style::default().white())
    //     .highlight_style(Style::default().green())
    //     .highlight_symbol("@")
    //     .repeat_highlight_symbol(true);
    //
    // f.render_stateful_widget(var_list, area, state);
}
