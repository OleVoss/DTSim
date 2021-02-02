use anyhow::Result;
use tui::{layout::{Constraint, Layout}, style::{Color, Style}, text::{Span, Spans}, widgets::{Axis, Block, BorderType, Borders, Gauge, Widget}};

use super::DrawableComponent;


pub struct Slider {
    name: &'static str,
    min: f64,
    max: f64,
    pub value: f64
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
        let percentage = match self.value {
            0.0 => 0,
            _ => {
                (self.value / self.max * 100.0) as u16
            }
        };
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
        app: &crate::app::App
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(90),
                ]
                .as_ref()
            )
            .split(rect);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain)
            .title(self.name);

        let gauge = Gauge::default()
            .block(block.clone())
            .gauge_style(Style::default().fg(Color::Red))
            .percent(self.get_percentage());

        let axis = Axis::default()
            .bounds([self.min, self.max]);

        // render widgets
        f.render_widget(block, rect);

        f.render_widget(gauge, chunks[1]);
        
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