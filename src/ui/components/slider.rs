use anyhow::Result;
use tui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, Table, Widget},
};

use super::DrawableComponent;

#[derive(Debug, Clone)]
pub struct Slider<'a> {
    block: Option<Block<'a>>,
    label: Option<Span<'a>>,
    style: Style,
    from: f64,
    to: f64,
}

impl<'a> Default for Slider<'a> {
    fn default() -> Self {
        Self {
            block: None,
            label: None,
            style: Style::default(),
            from: 0.0,
            to: 10.0,
        }
    }
}

impl<'a> Slider<'a> {

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn get_percentage(&self) -> u16 {
        let percentage = (self.value / self.max * 100.0) as u16;
        return percentage;
    }

    pub fn set_value(&mut self, value: f64) -> Result<()> {
        if value <= self.min {
            self.value = 0.0;
        } else if value > self.max {
            self.value = self.max;
        } else {
            self.value = value;
        }

        Ok(())
    }
}

impl Widget for Slider {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let slider_area = match self.block.take() {

        }
        for x in area.left()..area.right() {
            buf.get_mut(x, 2).set_symbol(symbols::block::FULL);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Slider;

    #[test]
    fn get_percentage() {
        let mut test_slider = Slider::new("test", 0.0, 100.0);
        test_slider.value = 50.0;
        let percentage = test_slider.get_percentage();
        assert_eq!(percentage, 50);
    }
}
