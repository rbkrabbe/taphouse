mod app;
mod brew;
mod ui;

use std::io;

use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use app::{App, AppEvent};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal).await;

    // Terminal teardown (always restore, even on error)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let debug_mode = std::env::args().any(|a| a == "--debug");

    let (event_tx, mut event_rx) = mpsc::channel::<AppEvent>(256);

    let mut app = App::new(event_tx.clone(), debug_mode);

    // Load installed packages at startup
    {
        let tx = event_tx.clone();
        tokio::spawn(async move {
            let formulae = match tokio::task::spawn_blocking(brew::installed::list_formulae).await {
                Ok(Ok(v)) => v,
                Ok(Err(e)) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] {e}"))).await; vec![] }
                Err(e) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] spawn failed: {e}"))).await; vec![] }
            };
            let casks = match tokio::task::spawn_blocking(brew::installed::list_casks).await {
                Ok(Ok(v)) => v,
                Ok(Err(e)) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] {e}"))).await; vec![] }
                Err(e) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] spawn failed: {e}"))).await; vec![] }
            };
            let _ = tx.send(AppEvent::InstalledLoaded { formulae, casks }).await;
        });
    }

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        tokio::select! {
            // Poll crossterm events with a small timeout so the loop stays responsive
            key_result = tokio::task::spawn_blocking(|| {
                if event::poll(std::time::Duration::from_millis(50))? {
                    Ok::<_, io::Error>(Some(event::read()?))
                } else {
                    Ok(None)
                }
            }) => {
                if let Ok(Ok(Some(Event::Key(key)))) = key_result {
                    if key.kind == KeyEventKind::Press {
                        app.handle(AppEvent::Key(key));
                        if app.should_quit {
                            break;
                        }
                    }
                }
            }

            Some(msg) = event_rx.recv() => {
                app.handle(msg);
                if app.should_quit {
                    break;
                }
            }
        }
    }

    Ok(())
}
