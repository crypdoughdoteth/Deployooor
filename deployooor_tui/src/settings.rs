use crate::{App, Mode, Screen};
use arboard::Clipboard;
use deployooor_core::config::NetworkSettings;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, Padding, Paragraph, Widget},
    Frame,
};
use rayon::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::io::{BufWriter, Write};
use Constraint::{Length, Min, Percentage};

#[derive(Default, Debug, Eq, PartialEq)]
pub struct NetworkSettingsState {
    pub name: String,
    pub provider: String,
    pub etherscan_api_key: Option<String>,
    pub editing_index: Option<usize>,
    pub editing: NetworkSettingsField,
    pub err_msg: Option<String>,
    pub unsaved_changes: bool,
}

impl Screen {
    pub fn render_settings(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let vertical = Layout::vertical([Length(1), Min(10), Length(1)]);
        let [_, _, footer_area] = vertical.areas(area);
        //        Paragraph::new(SETTINGS).centered().render(header_area, buf);

        Paragraph::new("Q - Main Menu | Enter - Save")
            .centered()
            .bold()
            .render(footer_area, buf);
    }
}

impl App {
    pub fn render_config_list(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([Length(1), Min(10), Length(1)]);
        let [_, inner_area, _] = vertical.areas(area);
        let layout2 = Layout::vertical([Percentage(50), Length(1), Percentage(50)]);
        let [top, _, bottom] = layout2.areas(inner_area);
        let items: Vec<Text<'_>> = self
            .config
            .networks
            .par_iter()
            .map(|e| Text::from(e.to_string()))
            .collect();

        match (self.editing, &self.network_settings_state.err_msg) {
            (crate::Mode::Normal, None) => {
                Paragraph::new(self.network_settings_state.to_string())
                .block(
                    Block::bordered()
                        .bold()
                        .title("Add a network")
                        .title_bottom(
                            "<ESC> - Stop Editing | N - Edit Network | V - Edit Etherscan | P - Edit Provider | <F1> - Paste",
                        ).title_alignment(Alignment::Center),
                )
                .render(bottom, frame.buffer_mut());
            }
            (crate::Mode::Insert, None) => {
                Paragraph::new(self.network_settings_state.to_string())
                .block(
                    Block::bordered()
                        .border_style(Color::Blue)
                        .bold()
                        .title(format!("Editing: {:?}", self.network_settings_state.editing))
                        .title_bottom(
                            "<ESC> - Stop Editing | N - Edit Network | V - Edit Etherscan | P - Edit Provider | <F1> - Paste",
                        ).title_alignment(Alignment::Center),
                )
                .render(bottom, frame.buffer_mut());
            }
            (_, Some(err)) => {
                Paragraph::new(self.network_settings_state.to_string())
                .block(
                    Block::bordered()
                    .border_style(Color::Red)
                        .bold()
                        .title(format!("Error: {err}"))
                        .title_bottom(
                            "<ESC> - Stop Editing | N - Edit Network | V - Edit Etherscan | P - Edit Provider | <F1> - Paste",
                        ).title_alignment(Alignment::Center),
                )
                .render(bottom, frame.buffer_mut());
            }
        }
        let list = match self.settings_list.selected() {
            Some(_selected) if self.config.networks.len() > 0 => List::new(items)
                .block(
                    Block::bordered()
                        .green()
                        .title("Configured Networks")
                        .title_bottom("<ESC> - Deselect Network | ↑ - Scroll Up | ↓ - Scroll Down | D - Delete Highlighted")
                        .title_alignment(Alignment::Center)
                        .padding(Padding::new(0, 0, 1, 0)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .scroll_padding(1)
                .direction(ListDirection::TopToBottom),
            _ => List::new(items)
                .block(
                    Block::bordered()
                        .white()
                        .title("Configured Networks")
                        .title_bottom("<ESC> - Deselect Network | ↑ - Scroll Up | ↓ - Scroll Down")
                        .title_alignment(Alignment::Center)
                        .padding(Padding::new(0, 0, 1, 0)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .scroll_padding(1)
                .direction(ListDirection::TopToBottom),
        };

        frame.render_stateful_widget(list, top, &mut self.settings_list);
    }

    pub fn handle_settings_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && self.editing == Mode::Insert {
                match key.code {
                    KeyCode::Enter => {
                        self.network_settings_state.editing.next();
                    }
                    KeyCode::Backspace => self.network_settings_state.pop(),
                    KeyCode::F(1) => {
                        let mut clipboard = Clipboard::new().unwrap();
                        let contents = clipboard.get_text();
                        if let Ok(c) = contents {
                            self.network_settings_state.push_str(&c);
                        }
                    }
                    KeyCode::Char(char) => self.network_settings_state.push(char),
                    KeyCode::Esc => self.editing = Mode::Normal,
                    _ => {}
                }
            } else if let Some(_err) = &self.network_settings_state.err_msg {
                match key.code {
                    _ => self.network_settings_state.err_msg = None,
                }
            } else {
                // normal mode
                match key.code {
                    // for setting index
                    KeyCode::Esc => self.settings_list.select(None),
                    KeyCode::Char('v') => {
                        self.network_settings_state.editing = NetworkSettingsField::ApiKey;
                        self.settings_list.select(None);
                        self.editing = Mode::Insert;
                    }
                    KeyCode::Char('p') => {
                        self.network_settings_state.editing = NetworkSettingsField::Provider;
                        self.settings_list.select(None);
                        self.editing = Mode::Insert;
                    }
                    KeyCode::Char('n') => {
                        self.network_settings_state.editing = NetworkSettingsField::Name;
                        self.settings_list.select(None);
                        self.editing = Mode::Insert;
                    }
                    KeyCode::Char('q') => {
                        self.screen = Screen::Home;
                        self.settings_list.select(None);
                        self.network_settings_state = NetworkSettingsState::default();
                    }
                    KeyCode::Char('d') => {
                        if let Some(selected) = self.settings_list.selected() {
                            self.config.networks.swap_remove(selected);
                            let file = std::fs::File::create("./").unwrap();
                            let mut file = BufWriter::new(file);

                            let res = file.write_all(
                                serde_json::to_string(&self.config.networks)
                                    .unwrap()
                                    .as_bytes(),
                            );

                            if let Err(err) = res {
                                self.network_settings_state.err_msg = Some(err.to_string());
                            }
                        }
                    }
                    KeyCode::Up => self.settings_list.select_previous(),
                    KeyCode::Down => self.settings_list.select_next(),
                    KeyCode::Enter => {
                        let ns = NetworkSettings::from_str(
                            &self.network_settings_state.name,
                            &self.network_settings_state.provider,
                            self.network_settings_state.etherscan_api_key.as_deref(),
                        );

                        match ns {
                            Ok(ns) => {
                                self.config.push(ns);
                                self.network_settings_state = NetworkSettingsState::default();

                                let file = std::fs::File::create("./").unwrap();
                                let mut file = BufWriter::new(file);

                                let res = file.write_all(
                                    serde_json::to_string(&self.config.networks)
                                        .unwrap()
                                        .as_bytes(),
                                );

                                if let Err(err) = res {
                                    self.network_settings_state.err_msg = Some(err.to_string());
                                }
                            }
                            Err(msg) => self.network_settings_state.err_msg = Some(msg.to_string()),
                        }
                        // append to config array
                        // save json to config file
                        // change screen and or render success
                        // clear local state
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl Display for NetworkSettingsState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " Network: {}\n Provider: {}\n Etherscan: {:?}",
            self.name, self.provider, self.etherscan_api_key
        )
    }
}

impl NetworkSettingsState {
    fn push_str(&mut self, new: &str) {
        match self.editing {
            NetworkSettingsField::Name => self.name.push_str(new),
            NetworkSettingsField::Provider => self.provider.push_str(new),
            NetworkSettingsField::ApiKey => {
                if let Some(s) = self.etherscan_api_key.as_mut() {
                    s.push_str(new);
                } else {
                    self.etherscan_api_key = Some(new.to_string());
                }
            }
        }
    }

    fn push(&mut self, new: char) {
        match self.editing {
            NetworkSettingsField::Name => self.name.push(new),
            NetworkSettingsField::Provider => self.provider.push(new),
            NetworkSettingsField::ApiKey => {
                if let Some(s) = self.etherscan_api_key.as_mut() {
                    s.push(new);
                } else {
                    self.etherscan_api_key = Some(String::from(new));
                }
            }
        }
    }

    fn pop(&mut self) {
        match self.editing {
            NetworkSettingsField::Name => {
                self.name.pop();
            }
            NetworkSettingsField::Provider => {
                self.provider.pop();
            }
            NetworkSettingsField::ApiKey => {
                if let Some(s) = self.etherscan_api_key.as_mut() {
                    s.pop();
                }
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum NetworkSettingsField {
    #[default]
    Name,
    Provider,
    ApiKey,
}

impl NetworkSettingsField {
    fn next(&mut self) {
        match self {
            NetworkSettingsField::Name => *self = NetworkSettingsField::Provider,
            NetworkSettingsField::Provider => *self = NetworkSettingsField::ApiKey,
            NetworkSettingsField::ApiKey => {}
        }
    }

    fn previous(&mut self) {
        match self {
            NetworkSettingsField::Name => {}
            NetworkSettingsField::Provider => *self = NetworkSettingsField::Name,
            NetworkSettingsField::ApiKey => *self = NetworkSettingsField::Provider,
        }
    }
}
