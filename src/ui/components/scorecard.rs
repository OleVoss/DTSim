use crate::ui::widgets::DrawableComponent;

pub struct Scorecard {}

impl Scorecard {}

impl DrawableComponent for Scorecard {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {

    }
}
