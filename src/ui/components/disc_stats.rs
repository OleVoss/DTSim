use tui::widgets::{Block, Borders};

use crate::{style::SharedTheme, ui::widgets::DrawableComponent};

pub struct DiscStats {
    pub focus: bool,
    theme: SharedTheme,
}

impl DiscStats {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            focus: false,
            theme,
        }
    }
}

impl DrawableComponent for DiscStats {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        let title = "Stats";
        let block = Block::default().title(title).borders(Borders::ALL);
        Ok(())
    }
}
