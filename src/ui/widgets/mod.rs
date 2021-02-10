mod slider;
mod slider_list;

pub use slider::Slider;
pub use slider_list::SliderList;
pub use slider_list::SliderListState;

use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::app::App;

pub trait DrawableComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &App) -> Result<()>;
}
