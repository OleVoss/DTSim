use std::any;

use anyhow::Result;
use tui::{Frame, layout::{Constraint, Direction, Layout, Rect}};

use crate::components::DrawableComponent;



pub struct Simulation {
    visible: bool,
}

impl Simulation {

    pub fn new() -> Self {
        Self {
            visible: false,
        }
    }

}

impl DrawableComponent for Simulation {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
                ]
                .as_ref()
            )
            .split(f.size());
        
        let left_chunks = 70;
        let right_chunks = 70;

        Ok(())
    }
}