use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{App, Mode, Tab};
use crate::brew::types::RemoteData;

pub fn render_package_list(f: &mut Frame, area: Rect, app: &App, list_state: &mut ListState) {
    let items: Vec<ListItem> = app
        .visible_items()
        .into_iter()
        .map(|n| ListItem::new(n))
        .collect();

    let title = match app.tab {
        Tab::InstalledFormulae => "Installed Formulae",
        Tab::InstalledCasks => "Installed Casks",
        Tab::BrowseFormulae => match &app.browse_formulae {
            RemoteData::Loading => "Browse Formulae (loading…)",
            RemoteData::Failed(_) => "Browse Formulae (error)",
            _ => "Browse Formulae",
        },
        Tab::BrowseCasks => match &app.browse_casks {
            RemoteData::Loading => "Browse Casks (loading…)",
            RemoteData::Failed(_) => "Browse Casks (error)",
            _ => "Browse Casks",
        },
    };

    let search_hint = if matches!(app.mode, Mode::Search) {
        format!(" [/{}]", app.search)
    } else if !app.search.is_empty() {
        format!(" [/{}]", app.search)
    } else {
        String::new()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("{title}{search_hint}"));

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, list_state);
}
