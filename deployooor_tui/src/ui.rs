use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Paragraph, Widget},
};

#[derive(Default, Clone, Copy)]
pub enum Screen {
    #[default]
    Home,
    Settings,
    Deploy,
    Logs,
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

const SETTINGS: &'static str = "
███████ ███████ ████████ ████████ ██ ███    ██  ██████  ███████ 
██      ██         ██       ██    ██ ████   ██ ██       ██      
███████ █████      ██       ██    ██ ██ ██  ██ ██   ███ ███████ 
     ██ ██         ██       ██    ██ ██  ██ ██ ██    ██      ██ 
███████ ███████    ██       ██    ██ ██   ████  ██████  ███████ 
";

use Constraint::{Length, Max, Min, Percentage};

impl Screen {
    fn render_home(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let vertical = Layout::vertical([Max(8), Min(8), Length(2)]);
        let [header_area, _, footer_area] = vertical.areas(area);

        Paragraph::new(WELCOME).centered().render(header_area, buf);

        Paragraph::new("Written By Crypdough.eth")
            .centered()
            .render(footer_area, buf);
    }

    fn render_settings(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let vertical = Layout::vertical([Max(8), Min(8), Length(2)]);
        let [header_area, _, footer_area] = vertical.areas(area);

        Paragraph::new(SETTINGS).centered().render(header_area, buf);

        Paragraph::new("Press Q To Return Back To The Main Menu")
            .centered()
            .render(footer_area, buf);
    }
}
