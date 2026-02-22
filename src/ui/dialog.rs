use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::BrewAction;

/// Center a rect of `width x height` within `area`.
pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

pub fn render_confirm(f: &mut Frame, area: Rect, action: &BrewAction) {
    let popup = centered_rect(44, 7, area);
    f.render_widget(Clear, popup);

    let title = match action {
        BrewAction::Install { name, .. } => format!("Install {}?", name),
        BrewAction::Uninstall { name, .. } => format!("Uninstall {}?", name),
        BrewAction::Upgrade { name, .. } => format!("Upgrade {}?", name),
    };

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(&title, Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [y] Confirm  ", Style::default().fg(Color::Green)),
            Span::styled("[n/Esc] Cancel", Style::default().fg(Color::Red)),
        ]),
        Line::from(""),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Confirm")
        .style(Style::default().bg(Color::DarkGray));

    let para = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(para, popup);
}

pub fn render_running(f: &mut Frame, area: Rect, action: &BrewAction, output: &[String], done: bool) {
    let title = format!(
        "brew {} {}{}",
        action.verb(),
        action.name(),
        if done { " (done)" } else { " (running…)" }
    );

    let popup = centered_rect(80, 24, area);
    f.render_widget(Clear, popup);

    let items: Vec<ListItem> = output
        .iter()
        .map(|l| ListItem::new(l.as_str()))
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().bg(Color::Black));

    let list = List::new(items).block(block);

    // Scroll to bottom: use a temporary ListState pointing at last item.
    let mut ls = ratatui::widgets::ListState::default();
    if !output.is_empty() {
        ls.select(Some(output.len() - 1));
    }
    f.render_stateful_widget(list, popup, &mut ls);

    if done {
        // Show "press any key" hint at bottom of popup
        let hint_area = Rect {
            x: popup.x + 1,
            y: popup.y + popup.height.saturating_sub(2),
            width: popup.width.saturating_sub(2),
            height: 1,
        };
        let hint = Paragraph::new("Press any key to close")
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(hint, hint_area);
    }
}
