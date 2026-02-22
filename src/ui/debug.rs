use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render_debug_panel(f: &mut Frame, area: Rect, logs: &[String]) {
    let items: Vec<ListItem> = logs
        .iter()
        .map(|l| ListItem::new(l.as_str()))
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Debug Log")
        .style(Style::default().bg(Color::Black));

    let list = List::new(items).block(block);

    // Scroll to bottom by selecting the last item.
    let mut ls = ratatui::widgets::ListState::default();
    if !logs.is_empty() {
        ls.select(Some(logs.len() - 1));
    }
    f.render_stateful_widget(list, area, &mut ls);
}
