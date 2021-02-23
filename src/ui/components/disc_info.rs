use std::convert::TryInto;

use tui::{
    layout::Layout,
    widgets::{Block, Borders},
};

use crate::{keys, models::disc::stats, style::SharedTheme, ui::widgets::DrawableComponent};

use super::{disc_stats::DiscStats, flightpath::Flightpath};

pub struct DiscInfo {
    pub focus: bool,
    theme: SharedTheme,
    stats: DiscStats,
    flightpath: Flightpath,
}

impl DiscInfo {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            focus: false,
            theme: theme.clone(),
            stats: DiscStats::new(theme.clone()),
            flightpath: Flightpath::new(theme),
        }
    }
}

impl DrawableComponent for DiscInfo {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        let title = format!("Disc Info [{}]", keys::get_hint(app.key_config.disc_info));
        let block = Block::default().title(title).borders(Borders::ALL);
        let chunks = Layout::default();

        f.render_widget(block, rect);
        Ok(())
    }
}
