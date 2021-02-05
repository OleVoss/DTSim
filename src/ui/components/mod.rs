mod slider;

pub use slider::Slider;

use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::app::App;

pub trait DrawableComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &App) -> Result<()>;
}
