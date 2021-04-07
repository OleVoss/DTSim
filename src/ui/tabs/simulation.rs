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

use crate::{
    app::App,
    main,
    models::simulation::simulation::Score,
    style::SharedTheme,
    ui::{components::scorecard::Scorecard, widgets::DrawableComponent},
};

pub struct Simulation {
    pub visible: bool,
    pub scorecard: Scorecard,
}

impl Simulation {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            visible: false,
            scorecard: Scorecard::new(theme),
        }
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
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(7),
                    Constraint::Percentage(80),
                    Constraint::Percentage(13),
                ]
                .as_ref(),
            )
            .split(rect);

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

        let hole = app.simulation.get_active_hole();
        let rendered_string = match hole {
            Some(h) => {
                renderer.render(h, main_chunks[1].width as i64, main_chunks[1].height as i64)
            }
            None => vec![String::from("nothing to render")],
        };
        let mut text: Vec<Spans> = Vec::new();
        for s in rendered_string {
            text.push(Spans::from(vec![Span::raw(s.to_owned())]))
        }
        let para = Paragraph::new(text)
            .block(hole_view)
            // .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        f.render_widget(para, main_chunks[1]);
        self.scorecard.draw(f, main_chunks[2], app)?;

        Ok(())
    }
}
