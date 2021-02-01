use tui::{Frame, backend::Backend, layout::{Constraint, Direction, Layout, Margin, Rect}, style::{Modifier, Style}, text::{Span, Spans}, widgets::{Block, Borders, Tabs}};
use anyhow::{Result, bail};
use crate::app::App;

use super::{components::DrawableComponent, tabs::{Overview, PlayerTab, Simulation}};

pub struct UI {
    overview_tab: Overview,
    simulation_tab: Simulation,
    pub player_tab: PlayerTab,
}

impl UI {
    pub fn new() -> Self {
        Self {
            overview_tab: Overview::new(),
            simulation_tab: Simulation::new(),
            player_tab: PlayerTab::new(),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, app: &App) -> Result<()> {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Min(2),
                    Constraint::Length(5),

                ]
                .as_ref(),
            )
            .split(fsize);

        self.draw_tabs(f, chunks_main[0], app);

        match app.tab {
            0 => self.overview_tab.draw(f, chunks_main[1], app)?,
            1 => self.simulation_tab.draw(f, chunks_main[1], app)?,
            3 => self.player_tab.draw(f, chunks_main[1], app)?,
            _ => bail!("unknown tab"),
        };

        Ok(())
    }

    fn draw_tabs<B: Backend>(&self, f: &mut Frame<B>, r: Rect, app: &App) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        // TODO: https://github.com/extrawurst/gitui/blob/master/src/app.rs Zeile 641-strings editable with config usw.
        let tabs = [
            Span::raw("Overview [1]"),
            Span::raw("Simulation [2]"),
            Span::raw("Config [3]"),
            Span::raw("Player [4]"),
            Span::raw("Discs [5]"),
        ]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();

        f.render_widget(
            Tabs::new(tabs)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default())
            )
            .style(Style::default().add_modifier(Modifier::DIM))
            .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED).remove_modifier(Modifier::DIM))
            .divider("|")
            .select(app.tab),
            r,
        );

    }
}