use std::{collections::HashMap, io::stdout, path::PathBuf};

use color_eyre::{config::HookBuilder, Result};
use config::Config;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    terminal::Terminal,
    text::Text,
    widgets::{Block, List, ListDirection, ListState, Padding, Widget},
    Frame,
};
use ui::Screen;
use utils::center;

pub mod config;
pub mod database;
pub mod deploy;
pub mod errors;
pub mod keys;
pub mod solc;
pub mod ui;
pub mod utils;

#[derive(Default)]
struct App {
    state: AppState,
    wallet_names: HashMap<String, PathBuf>,
    config: Config,
    screen: Screen,
    home_list: ListState,
    
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
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
        }
    }
    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.state == AppState::Running {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_home_list(&mut self, frame: &mut Frame, area: Rect) {
        use Constraint::{Length, Max, Min, Percentage};
        let vertical = Layout::vertical([Max(8), Min(8), Length(2)]);
        let [_, inner_area, _] = vertical.areas(area);

        let items = [
            Text::from("Deploy Vyper               -    V").centered(),
            Text::from("Deploy Solidity            -    S").centered(),
            Text::from("Deploy Stylus              -    A").centered(),
            Text::from("View Deployment Logs       -    L").centered(),
            Text::from("Create Keystore            -    K").centered(),
            Text::from("Settings                   -    C").centered(),
        ];
        let area = center(
            inner_area,
            Constraint::Percentage(50),
            Constraint::Length(10),
        );
        let list = List::new(items)
            .block(
                Block::bordered()
                    .white()
                    .title("Get Started")
                    .padding(Padding::new(0, 0, 1, 0)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, area, &mut self.home_list);
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
                Screen::Settings => {},
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
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Up => self.home_list.select_previous(),
                    KeyCode::Down => self.home_list.select_next(),
                    KeyCode::Char('c') => self.screen = Screen::Settings,
                    _ => {},
                }
            }
        }

        Ok(())
    }

    fn handle_settings_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.screen = Screen::Home,
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

// impl Widget for &mut App {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         self.screen.render(area, buf);
//     }
// }
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
