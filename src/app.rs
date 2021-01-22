use std::default;
use crate::{components::DrawableComponent, draw, tabs::Overview};
use tui::{Frame, backend::Backend, layout::{Constraint, Direction, Layout, Margin, Rect}, style::{Color, Style}, text::{Span, Spans}, widgets::{Block, BorderType, Borders, List, ListItem, Tabs}};
use anyhow::{Result, bail};



pub struct App<'a> {
    should_quit: bool,
    pub title: &'a str,
    tab: usize,
    overview_tab: Overview,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            tab: 0,
            overview_tab: Overview::new(),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Min(2),
                ]
                .as_ref(),
            )
            .split(fsize);

        self.draw_tabs(f, fsize);

        match self.tab {
            0 => self.overview_tab.draw(f, chunks_main[1])?,
            _ => bail!("unknown tab"),
        };

        Ok(())
    }

    fn draw_tabs<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        // TODO: https://github.com/extrawurst/gitui/blob/master/src/app.rs Zeile 641-strings editable with config usw.
        let tabs = vec![Spans::from(vec![
            Span::raw("Overview [1]"),
        ])];

        f.render_widget(
            Tabs::new(tabs)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
            )
            .divider("|")
            .select(self.tab),
            r,
        );

    }
}