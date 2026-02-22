use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::Tab;

pub fn render_tabs(f: &mut Frame, area: Rect, tab: Tab) {
    let titles = vec![
        "Installed: Formulae",
        "Installed: Casks",
        "Browse: Formulae",
        "Browse: Casks",
    ];
    let selected = match tab {
        Tab::InstalledFormulae => 0,
        Tab::InstalledCasks => 1,
        Tab::BrowseFormulae => 2,
        Tab::BrowseCasks => 3,
    };
    let tabs = Tabs::new(titles.iter().map(|t| Line::from(Span::raw(*t))).collect::<Vec<_>>())
        .block(Block::default().borders(Borders::ALL).title("taphouse 🍺"))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, area);
}
