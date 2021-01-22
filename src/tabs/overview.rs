use tui::{Frame, layout::{Constraint, Direction, Layout, Rect}};
use anyhow::Result;
use crate::components::{DescriptionComponent, DrawableComponent};

pub struct Overview {
    visible: bool,
    description: DescriptionComponent,
}

impl Overview {

    pub fn new() -> Self {
        Self {
            visible: false,
            description: DescriptionComponent::new(),
        }
    }
}

impl DrawableComponent for Overview {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(rect);

        Ok(())
    }
}