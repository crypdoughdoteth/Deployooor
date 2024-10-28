use crate::{App, Mode, Screen};
use alloy::{
    providers::{Provider, ProviderBuilder},
    signers::local::LocalSigner,
};
use chrono::{DateTime, Local};
use deployooor_core::{
    database::Deployment,
    deploy::{ConstructorArguments, Deploy},
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{
        Alignment,
        Constraint::{Length, Min, Percentage},
        Layout, Rect,
    },
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, Padding, Paragraph, Widget},
    Frame,
};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;

// Walk through directories from provided root to locate smart contracts
pub fn get_contracts(directory: &Path) -> Vec<PathBuf> {
    let mut contracts: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(directory) {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if let Some(x) = ext.to_str() {
                    if x == "sol" || x == "vy" {
                        contracts.push(entry.into_path());
                    }
                }
            }
        }
    }
    contracts
}

// choose contract to deploy from vec -- display as list
// select network
// select keystore
// record deployment in sqlite
// deployment settings

#[derive(Default, Debug)]
pub struct DeployState {
    contracts: Vec<PathBuf>,
    container_focus: Option<Container>,
    wallet_balances: Arc<Mutex<(Vec<String>, DateTime<Local>)>>,
    render_step_2: bool,
    constructor_arg_literals: Vec<String>,
    constructor_solidity_types: Vec<String>,
    currently_editing_literal: String,
    currently_editing_ty: String,
    focused_field: Option<Field>,
    mode: Mode,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Field {
    Type,
    Literal,
    List,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Container {
    Networks,
    DetectedContracts,
    Keystores,
}

impl Screen {
    // render static assets
    pub fn render_deployment(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let vertical = Layout::vertical([Length(1), Min(10), Length(1)]);
        let [header_area, _, footer_area] = vertical.areas(area);
        Paragraph::new("Deploy").centered().render(header_area, buf);
        Paragraph::new("Q - Main Menu / Go Back")
            .centered()
            .bold()
            .render(footer_area, buf);
    }
}

impl<'a> App {
    // render stateful assets
    pub fn render_deployment_stateful(&mut self, frame: &mut Frame<'a>, area: Rect) {
        let vertical = Layout::vertical([Length(1), Min(10), Length(1)]);
        let [_, body, _] = vertical.areas(area);
        let horiz = Layout::horizontal([Percentage(50), Percentage(50)]);
        let half = Layout::vertical([Percentage(50), Percentage(50)]);
        let [left, right] = horiz.areas(body);
        let [tl, bl] = half.areas(left);
        let [tr, br] = half.areas(right);
        let [th, bh] = half.areas(body);
        self.deploy_state.contracts = get_contracts(Path::new("./"));
        let contracts = &self.deploy_state.contracts;
        if self.deploy_state.render_step_2 == true {
            // render the second step of the deployment process
            // contructor args
            // compiler args

            let [arg_color, input_color] = match self.deploy_state.mode {
                Mode::Insert => [Color::White, Color::Magenta],
                Mode::Normal => {
                    if let Some(Field::List) = self.deploy_state.focused_field {
                        [Color::Cyan, Color::White]
                    } else {
                        [Color::White, Color::White]
                    }
                }
            };

            let constructor_args = self
                .deploy_state
                .constructor_arg_literals
                .iter()
                .zip(self.deploy_state.constructor_solidity_types.iter())
                .map(|(lit, ty)| {
                    let display = format!("{} {}", ty, lit);
                    Text::from(display)
                })
                .collect::<Vec<Text<'_>>>();

            let args = List::new(constructor_args)
                .block(
                    Block::bordered()
                        .border_style(arg_color)
                        .title("Detected Contracts")
                        .title_bottom("'C' - Clear | 'D' - Del One | ↑↓ - Select")
                        .title_alignment(Alignment::Center)
                        .padding(Padding::new(0, 0, 1, 0)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .scroll_padding(1)
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(args, th, &mut self.constructor_arg_list);
            let editing = format!(
                "Type: {}\nArg: {}",
                self.deploy_state.currently_editing_ty, self.deploy_state.currently_editing_literal
            );
            let title = match self.deploy_state.focused_field {
                Some(Field::Type) => "Editing: Type",
                Some(Field::Literal) => "Editing: Arg",
                _ => "Add Constructor Args",
            };
            Paragraph::new(editing)
                .block(
                    Block::bordered()
                        .border_style(input_color)
                        .bold()
                        .title(title)
                        .title_bottom("I - Edit | ENTER - Next")
                        .title_alignment(Alignment::Center),
                )
                .render(bh, frame.buffer_mut());
        } else {
            if let Some(network) = self.settings_list.selected() {
                let wallet_names = self.wallet_names.clone();
                let wallet_balances = Arc::clone(&self.deploy_state.wallet_balances);
                let config = self.config.networks.get(network); //.provider.clone();
                if let Some(config) = config {
                    let config = config.provider.clone();
                    if Local::now().timestamp()
                        > (self
                            .deploy_state
                            .wallet_balances
                            .lock()
                            .unwrap()
                            .1
                            .timestamp()
                            + 200)
                    {
                        tokio::spawn(async move {
                            let mut handles: Vec<_> = Vec::with_capacity(wallet_names.len());
                            for e in wallet_names.into_iter() {
                                let config = config.clone();
                                let handle = tokio::spawn(async move {
                                    let signer = LocalSigner::from_str(&e.1).unwrap();
                                    let address = signer.address();
                                    let http_endpoint = config;
                                    let provider = ProviderBuilder::new()
                                        .with_recommended_fillers()
                                        .on_http(http_endpoint);
                                    let bal = provider
                                        .get_balance(address.clone())
                                        .await
                                        .unwrap_or_default();
                                    format!("{} | {} | {}", e.0.as_str(), signer.address(), bal)
                                });
                                handles.push(handle);
                            }
                            let mut res: Vec<String> = Vec::with_capacity(handles.len());
                            for h in handles {
                                res.push(h.await.unwrap())
                            }
                            *wallet_balances.lock().unwrap() = (res, Local::now());
                        });
                    }
                }
            }

            let keystore_list = match self.deploy_state.wallet_balances.try_lock() {
                // the match guard makes sure that the
                // keystore display is never empty
                // while fetching balances asynchronously
                // try_lock prevents any possible blocking on render
                Ok(bals) if !bals.0.is_empty() => bals
                    .0
                    .clone()
                    .into_iter()
                    .map(|e| Text::from(e))
                    .collect::<Vec<Text<'_>>>(),
                // we failed to get the lock OR first time rendering
                _ => self
                    .wallet_names
                    .iter()
                    .map(|e| Text::from(e.0.as_str()))
                    .collect::<Vec<Text<'_>>>(),
            };

            let contract_list: Vec<Text<'_>> = contracts
                .as_slice()
                .iter()
                .map(|e| Text::from(e.file_name().unwrap().to_str().unwrap()))
                .collect();

            let network_list: Vec<Text<'_>> = self
                .config
                .networks
                .iter()
                .map(|e| Text::from(e.to_string()))
                .collect();

            let [kl_color, nl_color, cl_color] = match self.deploy_state.container_focus {
                Some(Container::Keystores) => [Color::Red, Color::White, Color::White],
                Some(Container::Networks) => [Color::White, Color::Green, Color::White],
                Some(Container::DetectedContracts) => [Color::White, Color::White, Color::Blue],
                None => [Color::White, Color::White, Color::White],
            };

            let contract_list = List::new(contract_list)
                .block(
                    Block::bordered()
                        .border_style(cl_color)
                        .title("Detected Contracts")
                        .title_bottom("'C' - Focus | ↑↓ - Select")
                        .title_alignment(Alignment::Center)
                        .padding(Padding::new(0, 0, 1, 0)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .scroll_padding(1)
                .direction(ListDirection::TopToBottom);

            let network_list = List::new(network_list)
                .block(
                    Block::bordered()
                        .border_style(nl_color)
                        .title("Configured Networks")
                        .title_bottom("'N' - Focus | ↑↓ - Select")
                        .title_alignment(Alignment::Center)
                        .padding(Padding::new(0, 0, 1, 0)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .scroll_padding(1)
                .direction(ListDirection::TopToBottom);

            let keystore_list = List::new(keystore_list)
                .block(
                    Block::bordered()
                        .border_style(kl_color)
                        .title("Keystores")
                        .title_bottom("'K' - Focus | ↑↓ - Select")
                        .title_alignment(Alignment::Center)
                        .padding(Padding::new(0, 0, 1, 0)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .scroll_padding(1)
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(network_list, tl, &mut self.settings_list);
            frame.render_stateful_widget(contract_list, tr, &mut self.deploy_contract_list);
            frame.render_stateful_widget(keystore_list, bh, &mut self.keystore_list);
        }
    }

    pub fn handle_deploy_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.deploy_state.mode {
                Mode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => {
                            if self.deploy_state.render_step_2 {
                                self.deploy_state.render_step_2 = false;
                            } else {
                                self.screen = Screen::Home
                            }
                        }
                        KeyCode::Char('a') => {
                            if self.deploy_state.render_step_2 {
                                self.deploy_state.mode = Mode::Insert;
                                self.deploy_state.focused_field = Some(Field::Literal);
                            }
                        }
                        KeyCode::Char('t') => {
                            if self.deploy_state.render_step_2 {
                                self.deploy_state.mode = Mode::Insert;
                                self.deploy_state.focused_field = Some(Field::Type);
                            }
                        }

                        KeyCode::Char('n') => {
                            if !self.deploy_state.render_step_2 {
                                self.deploy_state.container_focus = Some(Container::Networks)
                            }
                        }
                        KeyCode::Char('c') => {
                            if !self.deploy_state.render_step_2 {
                                self.deploy_state.container_focus =
                                    Some(Container::DetectedContracts)
                            }
                        }
                        KeyCode::Char('k') => {
                            if !self.deploy_state.render_step_2 {
                                self.deploy_state.container_focus = Some(Container::Keystores)
                            }
                        }
                        KeyCode::Up => {
                            if !self.deploy_state.render_step_2 {
                                if let Some(container) = &self.deploy_state.container_focus {
                                    match container {
                                        Container::Networks => self.settings_list.select_previous(),
                                        Container::DetectedContracts => {
                                            self.deploy_contract_list.select_previous()
                                        }
                                        Container::Keystores => {
                                            self.keystore_list.select_previous()
                                        }
                                    }
                                }
                            } else {
                                self.constructor_arg_list.select_previous();
                                self.deploy_state.mode = Mode::Normal;
                                self.deploy_state.focused_field = Some(Field::List);
                            }
                        }
                        KeyCode::Down => {
                            if !self.deploy_state.render_step_2 {
                                if let Some(container) = &self.deploy_state.container_focus {
                                    match container {
                                        Container::Networks => self.settings_list.select_next(),
                                        Container::DetectedContracts => {
                                            self.deploy_contract_list.select_next()
                                        }
                                        Container::Keystores => self.keystore_list.select_next(),
                                    }
                                }
                            } else {
                                self.constructor_arg_list.select_next();
                                self.deploy_state.mode = Mode::Normal;
                                self.deploy_state.focused_field = Some(Field::List);
                            }
                        }
                        KeyCode::Esc => {
                            if !self.deploy_state.render_step_2 {
                                self.deploy_state.container_focus = None
                            } else {
                                self.deploy_state.focused_field = None;
                            }
                        }
                        KeyCode::Enter => {
                            if !self.deploy_state.render_step_2 {
                                if let (Some(n), Some(k), Some(c)) = (
                                    self.settings_list.selected(),
                                    self.keystore_list.selected(),
                                    self.deploy_contract_list.selected(),
                                ) {
                                    self.deploy_state.render_step_2 = true;
                                }
                            } else {
                                if !self.deploy_state.currently_editing_literal.is_empty()
                                    && !self.deploy_state.currently_editing_ty.is_empty()
                                {
                                    let lit = &self.deploy_state.currently_editing_literal;
                                    let ty = &self.deploy_state.currently_editing_ty;
                                    self.deploy_state
                                        .constructor_solidity_types
                                        .push(ty.to_string());
                                    self.deploy_state
                                        .constructor_arg_literals
                                        .push(lit.to_string());
                                    self.deploy_state.currently_editing_literal.clear();
                                    self.deploy_state.currently_editing_ty.clear();
                                }
                            }
                        }
                        KeyCode::Char('r') => {
                            if self.deploy_state.render_step_2 {
                                let lits = self.deploy_state.constructor_arg_literals.as_slice();
                                let ty = &self.deploy_state.constructor_solidity_types.as_slice();
                                let cargs = Some(ConstructorArguments::new(ty, lits));
                                let network = self.settings_list.selected().unwrap();
                                let config = &self.config.networks[network];
                                let provider = config.provider.as_str();
                                let key_num = self.keystore_list.selected().unwrap();
                                let (_, private_key) = &self.wallet_names[key_num];
                                let contract_idx = self.deploy_contract_list.selected().unwrap();
                                let contract_path =
                                    self.deploy_state.contracts[contract_idx].to_str().unwrap();
                                let deploy_info = Deploy::init(provider, private_key, contract_path, cargs).unwrap();
                                // self.screen = Screen::FinishDeployment
                            }
                        }
                        // KeyCode::Char(y) => {
                        //     // copy address of selected address
                        //     // to clipboard
                        // }
                        _ => {}
                    };
                }
                Mode::Insert => match key.code {
                    KeyCode::Esc => self.deploy_state.mode = Mode::Normal,
                    KeyCode::Enter => match self.deploy_state.focused_field {
                        Some(Field::Literal) => {
                            self.deploy_state.focused_field = None;
                            if !self.deploy_state.currently_editing_literal.is_empty()
                                && !self.deploy_state.currently_editing_ty.is_empty()
                            {
                                let lit = &self.deploy_state.currently_editing_literal;
                                let ty = &self.deploy_state.currently_editing_ty;
                                self.deploy_state
                                    .constructor_solidity_types
                                    .push(ty.to_string());
                                self.deploy_state
                                    .constructor_arg_literals
                                    .push(lit.to_string());
                                self.deploy_state.currently_editing_literal.clear();
                                self.deploy_state.currently_editing_ty.clear();
                            }
                        }
                        Some(Field::Type) => {
                            self.deploy_state.focused_field = Some(Field::Literal);
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => match self.deploy_state.focused_field {
                        Some(Field::Literal) => {
                            self.deploy_state.currently_editing_literal.pop();
                        }
                        Some(Field::Type) => {
                            self.deploy_state.currently_editing_ty.pop();
                        }
                        _ => {}
                    },
                    KeyCode::F(1) => {
                        //
                    }
                    KeyCode::Char(char) => match self.deploy_state.focused_field {
                        Some(Field::Literal) => {
                            self.deploy_state.currently_editing_literal.push(char)
                        }
                        Some(Field::Type) => self.deploy_state.currently_editing_ty.push(char),
                        _ => {}
                    },
                    _ => {}
                },
            }
        }
        Ok(())
    }
}
