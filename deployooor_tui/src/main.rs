use color_eyre::{config::HookBuilder, Result};
use deployooor_core::{config::Config, database::Database, keys::Keys};
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
use std::{collections::HashMap, io::stdout};
use wallet::KeystoreState;

pub mod home;
pub mod settings;
pub mod utils;
pub mod wallet;

#[derive(Default)]
pub struct App {
    pub state: AppState,
    pub wallet_names: HashMap<String, String>,
    pub config: Config,
    pub screen: Screen,
    pub home_list: ListState,
    pub editing: Mode,
    pub network_settings_state: NetworkSettingsState,
    pub settings_list: ListState,
    pub keystore_list: ListState,
    pub keystore_state: KeystoreState,
    // for creating new keystores
    pub password: String,
    pub db: Database,
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
    Keystore,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}

#[tokio::main]
async fn main() -> Result<()> {
    let password = rpassword::prompt_password("\nWelcome to Deployooor\n\nPassword: ").unwrap();
    init_error_hooks()?;

    let mut terminal = init_terminal()?;
    terminal.clear().unwrap();
    // load key names into state
    // load networks into state from config
    let db = Database::default();
    let key_md = db.get_all_keys_metadata().unwrap();
    let keys: HashMap<String, String> = Keys::batch_decrypt(key_md, &password);
    let config = Config::from_default_file();
    // get balances for each network configure
    // for (k, v) in keys.par_iter() {
    //     Keys::get_balance(config.networks[0], v).await.unwrap();
    // }
    App::with_config_keys_pw(config, keys, password).run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    pub fn with_config_and_pw(config: Config, pw: String) -> Self {
        App {
            state: AppState::Running,
            wallet_names: <HashMap<String, String>>::default(),
            config,
            screen: Screen::Home,
            home_list: ListState::default(),
            editing: Mode::default(),
            network_settings_state: NetworkSettingsState::default(),
            settings_list: ListState::default(),
            password: pw,
            keystore_list: ListState::default(),
            keystore_state: KeystoreState::default(),
            db: Database::default(),
        }
    }

    pub fn with_config_keys_pw(config: Config, keys: HashMap<String, String>, pw: String) -> Self {
        App {
            state: AppState::Running,
            wallet_names: keys,
            config,
            screen: Screen::Home,
            home_list: ListState::default(),
            editing: Mode::default(),
            network_settings_state: NetworkSettingsState::default(),
            settings_list: ListState::default(),
            keystore_list: ListState::default(),
            keystore_state: KeystoreState::default(),
            password: pw,
            db: Database::default(),
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
                Screen::Keystore => {
                    self.render_keystore_list(frame, frame.size());
                }
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
            Screen::Keystore => self.handle_wallet_events()?,
        };

        Ok(())
    }

    fn handle_home_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        if let Some(selected) = self.home_list.selected() {
                            match selected {
                                5 => self.screen = Screen::Settings,
                                4 => self.screen = Screen::Keystore,
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Up => self.home_list.select_previous(),
                    KeyCode::Down => self.home_list.select_next(),
                    KeyCode::Char('c') => self.screen = Screen::Settings,
                    KeyCode::Char('k') => self.screen = Screen::Keystore,
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
