use crate::{App, Screen};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, ListState, Padding, Paragraph, Widget},
    Frame,
};
use Constraint::{Length, Max, Min};

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct ConfigLocalState {
    pub name: String,
    pub provider: String,
    pub block_explorer_api_key: String,
    pub editing: Mode,
}

impl Widget for Screen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self {
            Screen::Home => self.render_home(area, buf),
            Screen::Settings => self.render_settings(area, buf),
            Screen::Deploy => todo!(),
            Screen::Logs => todo!(),
        }
    }
}

const WELCOME: &'static str = "
██████  ███████ ██████  ██       ██████  ██    ██  ██████   ██████   ██████  ██████  
██   ██ ██      ██   ██ ██      ██    ██  ██  ██  ██    ██ ██    ██ ██    ██ ██   ██ 
██   ██ █████   ██████  ██      ██    ██   ████   ██    ██ ██    ██ ██    ██ ██████  
██   ██ ██      ██      ██      ██    ██    ██    ██    ██ ██    ██ ██    ██ ██   ██ 
██████  ███████ ██      ███████  ██████     ██     ██████   ██████   ██████  ██   ██ 
";

impl App {
    pub fn render_home_list(&mut self, frame: &mut Frame, area: Rect) {
        use Constraint::{Length, Max, Min};
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
        let area = crate::utils::center(
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
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, area, &mut self.home_list);
    }
}

impl Screen {
    fn render_home(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let vertical = Layout::vertical([Max(8), Min(8), Length(2)]);
        let [header_area, _, footer_area] = vertical.areas(area);

        Paragraph::new(WELCOME).centered().render(header_area, buf);

        Paragraph::new("Written By Crypdough.eth")
            .centered()
            .render(footer_area, buf);
    }
}
