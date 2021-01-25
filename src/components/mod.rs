use tui::{Frame, backend::Backend, layout::Rect};
use anyhow::Result;

pub mod description;

pub use description::DescriptionComponent;

pub trait DrawableComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()>;
}

pub trait Component {
    
}