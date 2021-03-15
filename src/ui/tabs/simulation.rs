use anyhow::Result;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, ui::widgets::DrawableComponent};

pub struct Simulation {
    pub visible: bool,
}

impl Simulation {
    pub fn new() -> Self {
        Self { visible: false }
    }
}

impl DrawableComponent for Simulation {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &App,
    ) -> Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(rect);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_chunks[1]);

        // TODO: call the draw() function of the specific components
        let scorecard_block = Block::default()
            .title("Scorecard")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let hole_view = Block::default()
            .title("Hole")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let course_view = Block::default()
            .title("View")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .border_type(BorderType::Plain);

        f.render_widget(scorecard_block, main_chunks[0]);

        f.render_widget(hole_view, right_chunks[0]);
        f.render_widget(course_view, right_chunks[1]);

        Ok(())
    }
}
