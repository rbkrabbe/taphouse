use ratatui::widgets::ListState;
use tokio::sync::mpsc;

use crate::brew::types::{CaskInfo, FormulaInfo, PackageKind, RemoteData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    InstalledFormulae,
    InstalledCasks,
    BrowseFormulae,
    BrowseCasks,
}

impl Tab {
    pub fn next(self) -> Self {
        match self {
            Tab::InstalledFormulae => Tab::InstalledCasks,
            Tab::InstalledCasks => Tab::BrowseFormulae,
            Tab::BrowseFormulae => Tab::BrowseCasks,
            Tab::BrowseCasks => Tab::InstalledFormulae,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Tab::InstalledFormulae => Tab::BrowseCasks,
            Tab::InstalledCasks => Tab::InstalledFormulae,
            Tab::BrowseFormulae => Tab::InstalledCasks,
            Tab::BrowseCasks => Tab::BrowseFormulae,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BrewAction {
    Install { name: String, kind: PackageKind },
    Uninstall { name: String, kind: PackageKind },
    Upgrade { name: String, kind: PackageKind },
}

impl BrewAction {
    pub fn verb(&self) -> &str {
        match self {
            BrewAction::Install { .. } => "install",
            BrewAction::Uninstall { .. } => "uninstall",
            BrewAction::Upgrade { .. } => "upgrade",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            BrewAction::Install { name, .. } => name,
            BrewAction::Uninstall { name, .. } => name,
            BrewAction::Upgrade { name, .. } => name,
        }
    }

    pub fn kind(&self) -> PackageKind {
        match self {
            BrewAction::Install { kind, .. } => *kind,
            BrewAction::Uninstall { kind, .. } => *kind,
            BrewAction::Upgrade { kind, .. } => *kind,
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    Search,
    Confirm { action: BrewAction },
    Running { action: BrewAction, output: Vec<String>, done: bool },
}

pub enum AppEvent {
    Key(crossterm::event::KeyEvent),
    InstalledLoaded { formulae: Vec<String>, casks: Vec<String> },
    BrowseFormulaeLoaded(Vec<FormulaInfo>),
    BrowseCasksLoaded(Vec<CaskInfo>),
    ActionOutput(String),
    ActionDone(bool),
    Error(String),
    DebugLog(String),
}

pub struct App {
    pub tab: Tab,
    pub mode: Mode,
    pub installed_formulae: Vec<String>,
    pub installed_casks: Vec<String>,
    pub browse_formulae: RemoteData<Vec<FormulaInfo>>,
    pub browse_casks: RemoteData<Vec<CaskInfo>>,
    pub list_state: ListState,
    pub search: String,
    pub should_quit: bool,
    pub event_tx: mpsc::Sender<AppEvent>,
    pub debug_mode: bool,
    pub show_debug: bool,
    pub debug_logs: Vec<String>,
}

impl App {
    pub fn new(event_tx: mpsc::Sender<AppEvent>, debug_mode: bool) -> Self {
        Self {
            tab: Tab::InstalledFormulae,
            mode: Mode::Normal,
            installed_formulae: vec![],
            installed_casks: vec![],
            browse_formulae: RemoteData::NotLoaded,
            browse_casks: RemoteData::NotLoaded,
            list_state: ListState::default(),
            search: String::new(),
            should_quit: false,
            event_tx,
            debug_mode,
            show_debug: false,
            debug_logs: vec![],
        }
    }

    /// Returns the filtered list of names visible in the current tab.
    pub fn visible_items(&self) -> Vec<String> {
        let query = self.search.to_lowercase();
        match self.tab {
            Tab::InstalledFormulae => self
                .installed_formulae
                .iter()
                .filter(|n| n.to_lowercase().contains(&query))
                .cloned()
                .collect(),
            Tab::InstalledCasks => self
                .installed_casks
                .iter()
                .filter(|n| n.to_lowercase().contains(&query))
                .cloned()
                .collect(),
            Tab::BrowseFormulae => match &self.browse_formulae {
                RemoteData::Loaded(v) => v
                    .iter()
                    .filter(|f| f.name.to_lowercase().contains(&query))
                    .map(|f| f.name.clone())
                    .collect(),
                _ => vec![],
            },
            Tab::BrowseCasks => match &self.browse_casks {
                RemoteData::Loaded(v) => v
                    .iter()
                    .filter(|c| c.token.to_lowercase().contains(&query))
                    .map(|c| c.token.clone())
                    .collect(),
                _ => vec![],
            },
        }
    }

    pub fn select_next(&mut self) {
        let len = self.visible_items().len();
        if len == 0 {
            return;
        }
        let i = self.list_state.selected().map(|i| (i + 1).min(len - 1)).unwrap_or(0);
        self.list_state.select(Some(i));
    }

    pub fn select_prev(&mut self) {
        let len = self.visible_items().len();
        if len == 0 {
            return;
        }
        let i = self.list_state.selected().map(|i| i.saturating_sub(1)).unwrap_or(0);
        self.list_state.select(Some(i));
    }

    fn reset_list(&mut self) {
        let len = self.visible_items().len();
        if len > 0 {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    pub fn selected_name(&self) -> Option<String> {
        let items = self.visible_items();
        self.list_state.selected().and_then(|i| items.get(i).cloned())
    }

    pub fn selected_formula_info(&self) -> Option<&FormulaInfo> {
        let name = self.selected_name()?;
        match &self.browse_formulae {
            RemoteData::Loaded(v) => v.iter().find(|f| f.name == name),
            _ => None,
        }
    }

    pub fn selected_cask_info(&self) -> Option<&CaskInfo> {
        let name = self.selected_name()?;
        match &self.browse_casks {
            RemoteData::Loaded(v) => v.iter().find(|c| c.token == name),
            _ => None,
        }
    }

    pub fn handle(&mut self, event: AppEvent) {
        use crossterm::event::{KeyCode, KeyModifiers};

        match event {
            AppEvent::Key(key) => {
                // Ctrl-C always quits
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    self.should_quit = true;
                    return;
                }

                match &self.mode {
                    Mode::Normal => self.handle_normal_key(key),
                    Mode::Search => self.handle_search_key(key),
                    Mode::Confirm { .. } => self.handle_confirm_key(key),
                    Mode::Running { done, .. } => {
                        if *done {
                            if key.code == KeyCode::Esc || key.code == KeyCode::Enter || key.code == KeyCode::Char('q') {
                                self.mode = Mode::Normal;
                            }
                        }
                    }
                }
            }

            AppEvent::InstalledLoaded { formulae, casks } => {
                let nf = formulae.len();
                let nc = casks.len();
                self.installed_formulae = formulae;
                self.installed_casks = casks;
                self.debug_logs.push(format!("[INFO] Loaded {nf} formulae, {nc} casks"));
                if matches!(self.tab, Tab::InstalledFormulae | Tab::InstalledCasks) {
                    self.reset_list();
                }
            }

            AppEvent::BrowseFormulaeLoaded(data) => {
                self.debug_logs.push(format!("[INFO] Loaded {} browse formulae", data.len()));
                self.browse_formulae = RemoteData::Loaded(data);
                if self.tab == Tab::BrowseFormulae {
                    self.reset_list();
                }
            }

            AppEvent::BrowseCasksLoaded(data) => {
                self.debug_logs.push(format!("[INFO] Loaded {} browse casks", data.len()));
                self.browse_casks = RemoteData::Loaded(data);
                if self.tab == Tab::BrowseCasks {
                    self.reset_list();
                }
            }

            AppEvent::ActionOutput(line) => {
                if let Mode::Running { output, .. } = &mut self.mode {
                    output.push(line);
                }
            }

            AppEvent::ActionDone(success) => {
                if let Mode::Running { output, done, .. } = &mut self.mode {
                    let msg = if success {
                        "✓ Done. Press any key to continue.".to_string()
                    } else {
                        "✗ Failed. Press any key to continue.".to_string()
                    };
                    output.push(msg);
                    *done = true;
                }
                // Refresh installed list after action
                let tx = self.event_tx.clone();
                tokio::spawn(async move {
                    let formulae = match tokio::task::spawn_blocking(crate::brew::installed::list_formulae).await {
                        Ok(Ok(v)) => v,
                        Ok(Err(e)) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] {e}"))).await; vec![] }
                        Err(e) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] spawn failed: {e}"))).await; vec![] }
                    };
                    let casks = match tokio::task::spawn_blocking(crate::brew::installed::list_casks).await {
                        Ok(Ok(v)) => v,
                        Ok(Err(e)) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] {e}"))).await; vec![] }
                        Err(e) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] spawn failed: {e}"))).await; vec![] }
                    };
                    let _ = tx.send(AppEvent::InstalledLoaded { formulae, casks }).await;
                });
            }

            AppEvent::Error(msg) => {
                self.debug_logs.push(format!("[ERROR] {msg}"));
                // Surface errors to running output or just store for display
                if let Mode::Running { output, done, .. } = &mut self.mode {
                    output.push(format!("Error: {msg}"));
                    *done = true;
                }
            }

            AppEvent::DebugLog(msg) => {
                self.debug_logs.push(msg);
            }
        }
    }

    fn handle_normal_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('?') if self.debug_mode => {
                self.show_debug = !self.show_debug;
            }
            KeyCode::Tab => {
                self.tab = self.tab.next();
                self.search.clear();
                self.reset_list();
                self.trigger_browse_load_if_needed();
            }
            KeyCode::BackTab => {
                self.tab = self.tab.prev();
                self.search.clear();
                self.reset_list();
                self.trigger_browse_load_if_needed();
            }
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.select_prev(),
            KeyCode::Char('/') => {
                self.mode = Mode::Search;
                self.search.clear();
                self.reset_list();
            }
            KeyCode::Char('r') => {
                let tx = self.event_tx.clone();
                tokio::spawn(async move {
                    let formulae = match tokio::task::spawn_blocking(crate::brew::installed::list_formulae).await {
                        Ok(Ok(v)) => v,
                        Ok(Err(e)) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] {e}"))).await; vec![] }
                        Err(e) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] spawn failed: {e}"))).await; vec![] }
                    };
                    let casks = match tokio::task::spawn_blocking(crate::brew::installed::list_casks).await {
                        Ok(Ok(v)) => v,
                        Ok(Err(e)) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] {e}"))).await; vec![] }
                        Err(e) => { let _ = tx.send(AppEvent::DebugLog(format!("[ERROR] spawn failed: {e}"))).await; vec![] }
                    };
                    let _ = tx.send(AppEvent::InstalledLoaded { formulae, casks }).await;
                });
            }
            KeyCode::Char('i') if matches!(self.tab, Tab::BrowseFormulae | Tab::BrowseCasks) => {
                if let Some(name) = self.selected_name() {
                    let kind = match self.tab {
                        Tab::BrowseCasks => PackageKind::Cask,
                        _ => PackageKind::Formula,
                    };
                    self.mode = Mode::Confirm {
                        action: BrewAction::Install { name, kind },
                    };
                }
            }
            KeyCode::Char('u') if matches!(self.tab, Tab::InstalledFormulae | Tab::InstalledCasks) => {
                if let Some(name) = self.selected_name() {
                    let kind = match self.tab {
                        Tab::InstalledCasks => PackageKind::Cask,
                        _ => PackageKind::Formula,
                    };
                    self.mode = Mode::Confirm {
                        action: BrewAction::Uninstall { name, kind },
                    };
                }
            }
            KeyCode::Char('U') if matches!(self.tab, Tab::InstalledFormulae | Tab::InstalledCasks) => {
                if let Some(name) = self.selected_name() {
                    let kind = match self.tab {
                        Tab::InstalledCasks => PackageKind::Cask,
                        _ => PackageKind::Formula,
                    };
                    self.mode = Mode::Confirm {
                        action: BrewAction::Upgrade { name, kind },
                    };
                }
            }
            _ => {}
        }
    }

    fn handle_search_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.search.clear();
                self.reset_list();
            }
            KeyCode::Enter => {
                self.mode = Mode::Normal;
                self.reset_list();
            }
            KeyCode::Backspace => {
                self.search.pop();
                self.reset_list();
            }
            KeyCode::Char(c) => {
                self.search.push(c);
                self.reset_list();
            }
            _ => {}
        }
    }

    fn handle_confirm_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;
        let action = match &self.mode {
            Mode::Confirm { action } => action.clone(),
            _ => return,
        };
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                let tx = self.event_tx.clone();
                let action_clone = action.clone();
                tokio::spawn(async move {
                    crate::brew::actions::run_brew_action(
                        action_clone.verb(),
                        action_clone.name(),
                        action_clone.kind(),
                        tx,
                    )
                    .await;
                });
                self.mode = Mode::Running {
                    action,
                    output: vec![],
                    done: false,
                };
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            _ => {}
        }
    }

    fn trigger_browse_load_if_needed(&mut self) {
        match self.tab {
            Tab::BrowseFormulae => {
                if matches!(self.browse_formulae, RemoteData::NotLoaded) {
                    self.browse_formulae = RemoteData::Loading;
                    let tx = self.event_tx.clone();
                    tokio::spawn(async move {
                        let client = reqwest::Client::new();
                        match crate::brew::api::fetch_formulae(&client).await {
                            Ok(data) => {
                                let _ = tx.send(AppEvent::BrowseFormulaeLoaded(data)).await;
                            }
                            Err(e) => {
                                let _ = tx.send(AppEvent::Error(e)).await;
                            }
                        }
                    });
                }
            }
            Tab::BrowseCasks => {
                if matches!(self.browse_casks, RemoteData::NotLoaded) {
                    self.browse_casks = RemoteData::Loading;
                    let tx = self.event_tx.clone();
                    tokio::spawn(async move {
                        let client = reqwest::Client::new();
                        match crate::brew::api::fetch_casks(&client).await {
                            Ok(data) => {
                                let _ = tx.send(AppEvent::BrowseCasksLoaded(data)).await;
                            }
                            Err(e) => {
                                let _ = tx.send(AppEvent::Error(e)).await;
                            }
                        }
                    });
                }
            }
            _ => {}
        }
    }
}
