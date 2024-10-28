use crate::{App, Mode, Screen};
use alloy::signers::local::LocalSigner;
use deployooor_core::keys::Keys;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, Padding, Paragraph, Widget},
    Frame,
};
use std::{path::Path, str::FromStr};
use Constraint::{Length, Min, Percentage};

#[derive(Default, Debug, Eq, PartialEq)]
pub struct KeystoreState {
    pub err_msg: Option<String>,
    pub name: String,
}

impl Screen {
    pub fn render_keystores(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let vertical = Layout::vertical([Length(1), Min(10), Length(1)]);
        let [_, _, footer_area] = vertical.areas(area);

        Paragraph::new("Q - Main Menu | Enter - Save")
            .centered()
            .bold()
            .render(footer_area, buf);
    }
}

impl App {
    pub fn render_keystore_list(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([Length(1), Min(10), Length(1)]);
        let [_, inner_area, _] = vertical.areas(area);
        let layout2 = Layout::vertical([Percentage(50), Length(1), Percentage(50)]);
        let [top, _, bottom] = layout2.areas(inner_area);
        let items: Vec<Text<'_>> = self
            .wallet_names
            .iter()
            .map(|e| {
                let signer = LocalSigner::from_str(&e.1).unwrap();
                let address = signer.address();
                let display = format!("{} | {}", e.0, address);
                Text::from(display)
            })
            .collect();

        match (self.editing, &self.keystore_state.err_msg) {
            (crate::Mode::Normal, None) => {
                Paragraph::new(format!(" Name: {}", self.keystore_state.name))
                    .block(
                        Block::bordered()
                            .bold()
                            .title("Create Keystore")
                            .title_bottom("<ESC> - Stop Editing | N - Edit Name")
                            .title_alignment(Alignment::Center),
                    )
                    .render(bottom, frame.buffer_mut());
            }
            (crate::Mode::Insert, None) => {
                Paragraph::new(format!(" Name: {}", self.keystore_state.name))
                    .block(
                        Block::bordered()
                            .border_style(Color::Blue)
                            .bold()
                            .title("Editing Name")
                            .title_bottom("<ESC> - Stop Editing | N - Edit Name")
                            .title_alignment(Alignment::Center),
                    )
                    .render(bottom, frame.buffer_mut());
            }
            (_, Some(err)) => {
                Paragraph::new(format!(" Name: {}", self.keystore_state.name))
                    .block(
                        Block::bordered()
                            .border_style(Color::Red)
                            .bold()
                            .title(format!("Error: {err}"))
                            .title_bottom("<ESC> - Stop Editing | N - Edit Name")
                            .title_alignment(Alignment::Center),
                    )
                    .render(bottom, frame.buffer_mut());
            }
        }
        let list = match self.keystore_list.selected() {
            Some(_selected) if self.wallet_names.len() > 0 => List::new(items)
                .block(
                    Block::bordered()
                        .green()
                        .title("Accounts")
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
                        .title("Accounts")
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

        frame.render_stateful_widget(list, top, &mut self.keystore_list);
    }

    pub fn handle_wallet_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && self.editing == Mode::Insert {
                match key.code {
                    KeyCode::Enter => {
                        self.editing = Mode::Normal;
                    }
                    KeyCode::Backspace => {
                        self.keystore_state.name.pop();
                    }
                    KeyCode::Char(char) => self.keystore_state.name.push(char),
                    KeyCode::Esc => self.editing = Mode::Normal,
                    _ => {}
                }
            } else if let Some(_err) = &self.keystore_state.err_msg {
                match key.code {
                    _ => self.keystore_state.err_msg = None,
                }
            } else {
                // normal mode
                match key.code {
                    // for setting index
                    KeyCode::Esc => self.keystore_list.select(None),
                    KeyCode::Char('n') => {
                        self.keystore_list.select(None);
                        self.editing = Mode::Insert;
                    }
                    KeyCode::Char('q') => {
                        self.screen = Screen::Home;
                        self.keystore_list.select(None);
                        self.keystore_state = KeystoreState::default();
                    }
                    // KeyCode::Char('d') => {
                    //     if let Some(selected) = self.settings_list.selected() {
                    //         self.config.networks.swap_remove(selected);
                    //         let file = std::fs::File::create("./deployooor-config.json").unwrap();
                    //         let mut file = BufWriter::new(file);
                    //
                    //         let res = file.write_all(
                    //             serde_json::to_string(&self.config.networks)
                    //                 .unwrap()
                    //                 .as_bytes(),
                    //         );
                    //
                    //         if let Err(err) = res {
                    //             self.network_settings_state.err_msg = Some(err.to_string());
                    //         }
                    //     }
                    // }
                    KeyCode::Up => self.keystore_list.select_previous(),
                    KeyCode::Down => self.keystore_list.select_next(),
                    KeyCode::Enter => {
                        Keys::new(
                            &Path::new("./").canonicalize().unwrap(),
                            &self.keystore_state.name,
                            &self.password,
                        )
                        .unwrap();
                        self.db
                            .record_key_metadata(
                                &Path::new("./").canonicalize().unwrap(),
                                &self.keystore_state.name,
                            )
                            .unwrap();
                        let key = Keys::decrypt_to_string(
                            Path::new(&format!("./{}", &self.keystore_state.name)),
                            &self.password,
                        )
                        .unwrap();
                        self.wallet_names
                            .push((self.keystore_state.name.to_string(), key));
                        self.keystore_state.name.clear();
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }
}
