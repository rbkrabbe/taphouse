use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Tab};
use crate::brew::types::RemoteData;

pub fn render_detail(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default().borders(Borders::ALL).title("Detail");

    match app.tab {
        Tab::BrowseFormulae => {
            if let Some(info) = app.selected_formula_info() {
                let installed = app.installed_formulae.contains(&info.name);
                let version = info.versions.stable.as_deref().unwrap_or("?");
                let lines = vec![
                    Line::from(vec![
                        Span::styled("Name:    ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(&info.name),
                    ]),
                    Line::from(vec![
                        Span::styled("Desc:    ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(&info.desc),
                    ]),
                    Line::from(vec![
                        Span::styled("Version: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(version),
                    ]),
                    Line::from(vec![
                        Span::styled("License: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(info.license.as_deref().unwrap_or("N/A")),
                    ]),
                    Line::from(vec![
                        Span::styled("Tap:     ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(&info.tap),
                    ]),
                    Line::from(vec![
                        Span::styled("Deps:    ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(if info.dependencies.is_empty() {
                            "none".to_string()
                        } else {
                            info.dependencies.join(", ")
                        }),
                    ]),
                    Line::from(vec![
                        Span::styled("Homepage:", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(format!(" {}", info.homepage)),
                    ]),
                    Line::from(""),
                    Line::from(if installed {
                        Span::styled("✓ Installed", Style::default().fg(Color::Green))
                    } else {
                        Span::styled("[i] to install", Style::default().fg(Color::Cyan))
                    }),
                ];
                let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
                f.render_widget(para, area);
            } else {
                match &app.browse_formulae {
                    RemoteData::Loading => {
                        let para = Paragraph::new("Loading formulae…").block(block);
                        f.render_widget(para, area);
                    }
                    RemoteData::Failed(e) => {
                        let para = Paragraph::new(format!("Error: {e}")).block(block);
                        f.render_widget(para, area);
                    }
                    _ => {
                        let para = Paragraph::new("Select a package").block(block);
                        f.render_widget(para, area);
                    }
                }
            }
        }
        Tab::BrowseCasks => {
            if let Some(info) = app.selected_cask_info() {
                let installed = app.installed_casks.contains(&info.token);
                let display_name = info.name.first().map(|s| s.as_str()).unwrap_or(&info.token);
                let lines = vec![
                    Line::from(vec![
                        Span::styled("Name:    ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(display_name),
                    ]),
                    Line::from(vec![
                        Span::styled("Token:   ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(&info.token),
                    ]),
                    Line::from(vec![
                        Span::styled("Desc:    ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(&info.desc),
                    ]),
                    Line::from(vec![
                        Span::styled("Version: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(&info.version),
                    ]),
                    Line::from(vec![
                        Span::styled("Homepage:", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(format!(" {}", info.homepage)),
                    ]),
                    Line::from(""),
                    Line::from(if installed {
                        Span::styled("✓ Installed", Style::default().fg(Color::Green))
                    } else {
                        Span::styled("[i] to install", Style::default().fg(Color::Cyan))
                    }),
                ];
                let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
                f.render_widget(para, area);
            } else {
                match &app.browse_casks {
                    RemoteData::Loading => {
                        let para = Paragraph::new("Loading casks…").block(block);
                        f.render_widget(para, area);
                    }
                    RemoteData::Failed(e) => {
                        let para = Paragraph::new(format!("Error: {e}")).block(block);
                        f.render_widget(para, area);
                    }
                    _ => {
                        let para = Paragraph::new("Select a package").block(block);
                        f.render_widget(para, area);
                    }
                }
            }
        }
        Tab::InstalledFormulae | Tab::InstalledCasks => {
            if let Some(name) = app.selected_name() {
                let lines = vec![
                    Line::from(vec![
                        Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(&name),
                    ]),
                    Line::from(""),
                    Line::from(Span::styled("[u] Uninstall  [U] Upgrade", Style::default().fg(Color::Cyan))),
                ];
                let para = Paragraph::new(lines).block(block);
                f.render_widget(para, area);
            } else {
                let para = Paragraph::new("No packages installed").block(block);
                f.render_widget(para, area);
            }
        }
    }
}
