use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, Mode};

use super::debug::render_debug_panel;
use super::detail::render_detail;
use super::dialog::{render_confirm, render_running};
use super::package_list::render_package_list;
use super::tabs::render_tabs;

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    let show_debug = app.debug_mode && app.show_debug;

    // Top-level vertical split: tabs | content | [debug] | help
    let chunks = if show_debug {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),      // tab bar
                Constraint::Percentage(60), // main content
                Constraint::Percentage(40), // debug panel
                Constraint::Length(1),      // help bar
            ])
            .split(size)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // tab bar
                Constraint::Min(0),    // main content
                Constraint::Length(1), // help bar
            ])
            .split(size)
    };

    render_tabs(f, chunks[0], app.tab);

    // Main content: left list | right detail
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    // We need a mutable clone of list_state for rendering
    let mut ls = app.list_state.clone();
    render_package_list(f, content_chunks[0], app, &mut ls);
    render_detail(f, content_chunks[1], app);

    if show_debug {
        render_debug_panel(f, chunks[2], &app.debug_logs);
        // Help bar is at index 3
        let help = help_text(app);
        let help_para = Paragraph::new(Line::from(help))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help_para, chunks[3]);
    } else {
        // Help bar is at index 2
        let help = help_text(app);
        let help_para = Paragraph::new(Line::from(help))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help_para, chunks[2]);
    }

    // Overlays
    match &app.mode {
        Mode::Confirm { action } => render_confirm(f, size, action),
        Mode::Running { action, output, done } => {
            render_running(f, size, action, output, *done)
        }
        _ => {}
    }
}

fn help_text(app: &App) -> Vec<Span<'static>> {
    match &app.mode {
        Mode::Search => vec![
            Span::raw(" Type to filter  "),
            Span::raw("Enter accept  "),
            Span::raw("Esc cancel"),
        ],
        Mode::Confirm { .. } => vec![
            Span::raw(" y confirm  "),
            Span::raw("n/Esc cancel"),
        ],
        Mode::Running { .. } => vec![
            Span::raw(" Waiting for brew…  "),
            Span::raw("Esc/Enter close when done"),
        ],
        Mode::Normal => {
            use crate::app::Tab;
            let mut spans = vec![
                Span::raw(" q quit  "),
                Span::raw("Tab/S-Tab switch  "),
                Span::raw("↑↓/jk navigate  "),
                Span::raw("/ search  "),
                Span::raw("r refresh  "),
            ];
            match app.tab {
                Tab::BrowseFormulae | Tab::BrowseCasks => {
                    spans.push(Span::styled("i install", Style::default().fg(Color::Green)));
                }
                Tab::InstalledFormulae | Tab::InstalledCasks => {
                    spans.push(Span::styled("u uninstall  ", Style::default().fg(Color::Red)));
                    spans.push(Span::styled("U upgrade", Style::default().fg(Color::Yellow)));
                }
            }
            if app.debug_mode {
                spans.push(Span::raw("  ?  debug"));
            }
            spans
        }
    }
}
