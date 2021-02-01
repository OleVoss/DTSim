use tui::{Frame, backend::Backend, layout::Rect};
use anyhow::Result;

use crate::app::App;

pub trait DrawableComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &App
    ) -> Result<()>;
}