use anyhow::Result;
use tui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, Table, Widget},
};

use super::DrawableComponent;

pub struct Slider {
    name: &'static str,
    min: f64,
    max: f64, // should not be zero for various reasons
    pub value: f64,
}

impl Slider {
    pub fn new(name: &'static str, min: f64, max: f64) -> Self {
        Self {
            name,
            min,
            max,
            value: 0.0,
        }
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

impl DrawableComponent for Slider {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        _app: &crate::app::App,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
            // .margin(1)
            .split(rect);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain)
            .title(self.name);

        // render widgets
        f.render_widget(block, rect);

        Ok(())
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
