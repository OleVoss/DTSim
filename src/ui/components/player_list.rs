use tui::widgets::Block;

use crate::{keys, style::SharedTheme, ui::widgets::DrawableComponent};

pub struct PlayerList {
    selected: bool,
    theme: SharedTheme,
}

impl PlayerList {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            selected: false,
            theme,
        }
    }
}

impl DrawableComponent for PlayerList {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        let title = format!("Player [{}]", keys::get_hint(app.key_config.player_list));

        let block = Block::default().title(title);
        Ok(())
    }
}
