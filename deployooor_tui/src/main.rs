use color_eyre::{config::HookBuilder, Result};
use deployooor_core::config::Config;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    terminal::Terminal,
    widgets::ListState,
};
use settings::NetworkSettingsState;
use std::{collections::HashMap, io::stdout, path::PathBuf};

pub mod home;
pub mod settings;
pub mod utils;

#[derive(Default)]
pub struct App {
    pub state: AppState,
    pub wallet_names: HashMap<String, PathBuf>,
    pub config: Config,
    pub screen: Screen,
    pub home_list: ListState,
    pub editing: Mode,
    pub network_settings_state: NetworkSettingsState,
    pub settings_list: ListState,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Default, Clone, Copy)]
pub enum Screen {
    #[default]
    Home,
    Settings,
    Deploy,
    Logs,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}

fn main() -> Result<()> {
    init_error_hooks()?;
    let mut terminal = init_terminal()?;
    // load key names into state
    // load networks into state from config
    App::default().run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    pub fn new(wallet_names: HashMap<String, PathBuf>, config: Config) -> Self {
        App {
            state: AppState::Running,
            wallet_names,
            config,
            screen: Screen::Home,
            home_list: ListState::default(),
            editing: Mode::default(),
            network_settings_state: NetworkSettingsState::default(),
            settings_list: ListState::default(),
        }
    }

    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.state == AppState::Running {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self.screen, frame.size());
            // render stateful pieces after screens
            match self.screen {
                Screen::Home => {
                    // render list
                    self.render_home_list(frame, frame.size());
                }
                Screen::Settings => {
                    self.render_config_list(frame, frame.size());
                }
                Screen::Deploy => todo!(),
                Screen::Logs => todo!(),
            }
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match &self.screen {
            Screen::Home => self.handle_home_events()?,
            Screen::Settings => self.handle_settings_events()?,
            Screen::Deploy => todo!(),
            Screen::Logs => todo!(),
        };

        Ok(())
    }

    fn handle_home_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        if let Some(selected) = self.home_list.selected() {

                            // if there is a selected element, get index
                            // get screen to transition to from index
                        }
                    }
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Up => self.home_list.select_previous(),
                    KeyCode::Down => self.home_list.select_next(),
                    KeyCode::Char('c') => self.screen = Screen::Settings,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
