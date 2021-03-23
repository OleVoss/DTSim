use anyhow::Result;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use DTSim_courses::components::{
    renderer::{self, PrintRenderer},
    Renderer,
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
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(rect);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
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

        let renderer = PrintRenderer::new(' ', 't', 'B', 'w', 'x', 'o');

        let hole = app.course.hole(1).unwrap();
        let rendered_string = renderer.render(
            hole,
            right_chunks[1].width as i64,
            right_chunks[1].height as i64,
        );
        let mut text: Vec<Spans> = Vec::new();
        for s in rendered_string {
            text.push(Spans::from(vec![Span::raw(s.to_owned())]))
        }
        let para = Paragraph::new(text)
            .block(hole_view)
            // .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        f.render_widget(scorecard_block, main_chunks[0]);

        f.render_widget(para, right_chunks[1]);

        Ok(())
    }
}
