use std::io::stdout;

use color_eyre::{config::HookBuilder, owo_colors::OwoColorize, Result};
use config::Config;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Layout, Rect},
    style::{
        palette::{
            material::WHITE,
            tailwind::{self, SLATE},
        },
        Color, Modifier, Style, Styled, Stylize,
    },
    symbols,
    terminal::Terminal,
    text::{Line, Text},
    widgets::{
        Block, HighlightSpacing, List, ListDirection, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Tabs, Widget,
    },
};
use ui::Screen;

pub mod config;
pub mod database;
pub mod deploy;
pub mod errors;
pub mod keys;
pub mod solc;
pub mod ui;

#[derive(Default)]
struct App<'a> {
    state: AppState,
    wallet_names: Vec<String>,
    config: Config,
    screen: Screen<'a>,
}

impl<'a> App<'a> {
    pub fn new(wallet_names: Vec<String>, config: Config) -> Self {
        App {
            state: AppState::Running,
            wallet_names,
            config,
            screen: Screen::Home,
        }
    }
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

impl<'a> App<'a> {
    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.state == AppState::Running {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(&mut *self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match &self.screen {
            Screen::Home => self.handle_home_events()?,
            Screen::Settings(_) => todo!(),
            Screen::Deploy(_) => todo!(),
            Screen::Logs(_) => todo!(),
        };

        Ok(())
    }

    fn handle_home_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.quit(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn handle_settings_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.quit(),
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

impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.screen.render(area, buf);
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
