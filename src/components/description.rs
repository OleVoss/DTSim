use tui::{Frame, backend::Backend, layout::{Alignment, Rect}, style::{Color, Style}, text::Text, widgets::{Block, Paragraph}};
use anyhow::Result;
use super::DrawableComponent;



pub struct DescriptionComponent {
    text: String,
}

impl DescriptionComponent {

    pub fn new() -> Self {
        Self {
           text: String::from("Test description.\nThis will later be loaded from an description file."), 
        }
    }
}

impl DrawableComponent for DescriptionComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        r: Rect,
    ) -> Result<()> {
        let block = Block::default();
        let mut text = Text::from(self.text.clone());
        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        f.render_widget(paragraph, r);

        Ok(())
    }
}