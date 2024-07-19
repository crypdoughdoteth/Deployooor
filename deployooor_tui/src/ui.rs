use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, Padding, Paragraph, Widget},
};

use crate::{config::NetworkSettings, database::Deployment, deploy::Deploy};

// localize state to screens
// reset after changing screens
// global state is stored in App

#[derive(Default)]
pub enum Screen<'a> {
    #[default]
    Home,
    Settings(ConfigLocalState),
    Deploy(Deploy<'a>),
    Logs(Vec<Deployment>),
}

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

impl<'a> Widget for &mut Screen<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self {
            Screen::Home => self.render_home(area, buf),
            Screen::Settings(ls) => todo!(),
            Screen::Deploy(_) => todo!(),
            Screen::Logs(_) => todo!(),
        }
    }
}

impl<'a> Screen<'a> {
    fn render_home(&mut self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new("Welcome to Deployooor TUI!")
            .block(
                Block::bordered()
                    .white()
                    .title("Deployooor TUI".bold().into_right_aligned_line())
                    .title_bottom("Crypdough.eth • Developer DAO".bold())
                    .title_style(Style::new().add_modifier(Modifier::ITALIC))
                    .padding(Padding::new(0, 0, area.height / 8, 0)),
            )
            .centered()
            .render(area, buf);

        let items = [
            Text::from("Begin Deployment Process     -     <F5>").centered(),
            Text::from("                                       ").centered(),
            Text::from("View Deployment Logs         -     <F6>").centered(),
            Text::from("                                       ").centered(),
            Text::from("Deployooor Settings          -     <F7>").centered(),
        ];
        List::new(items)
            .block(Block::default().padding(Padding::new(0, 0, area.height / 2, 0)))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
            .render(area, buf);
    }
}
