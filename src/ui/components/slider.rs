use anyhow::Result;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
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
    pub value: f64,
    ignore_bounds: bool,
}

impl<'a> Default for Slider<'a> {
    fn default() -> Self {
        Self {
            block: None,
            label: None,
            style: Style::default(),
            from: 0.0,
            to: 10.0,
            value: 0.0,
            ignore_bounds: false,
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

    pub fn value(mut self, value: f64) -> Self {
        if value >= self.to && !self.ignore_bounds {
            self.value = self.to;
        } else if value <= self.from && !self.ignore_bounds {
            self.value = self.from;
        } else {
            self.value = value;
        }
        self
    }

    // TODO: fix call order; evaluations should be done in render()
    /// Should be called before value
    pub fn ignore_bounds(mut self, ignore: bool) -> Self {
        self.ignore_bounds = ignore;
        self
    }

    pub const HIGHT: u16 = 4;
}

impl<'a> Widget for Slider<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let slider_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        // TODO: axis
        // ? Implemented when needed

        // value indicators
        //  0         12    20
        // [###########      ]
        let width = self.from.to_string().chars().count() as u16;
        buf.set_span(
            slider_area.left(),
            slider_area.top(),
            &Span::from(self.from.to_string()),
            width,
        );

        let width = self.to.to_string().chars().count() as u16;
        buf.set_span(
            slider_area.right() - width,
            slider_area.top(),
            &Span::from(self.to.to_string()),
            width,
        );

        let ratio = slider_area.width as f64 / self.to;
        let value_x = slider_area.left() + (self.value * ratio) as u16;
        let width = self.value.to_string().chars().count() as u16;
        if value_x < slider_area.right() || self.ignore_bounds {
            buf.set_span(
                value_x
                    .max(slider_area.left())
                    .min(slider_area.right() - width),
                slider_area.top(),
                &Span::from(self.value.to_string()),
                width,
            );
        }

        buf.get_mut(value_x.min(slider_area.right() - 1), slider_area.top() + 1)
            .set_fg(Color::Red);
        // slider
        for x in slider_area.left()..slider_area.right() {
            buf.get_mut(x, slider_area.top() + 1)
                .set_symbol(symbols::block::SEVEN_EIGHTHS);
            if x < value_x {
                buf.get_mut(x, slider_area.top() + 1).set_fg(Color::Red);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Slider;

    #[test]
    fn set_value() {
        let slider = Slider::default().value(5.0);
        assert!(slider.value == 5.0);
    }

    #[test]
    fn ignore_bounds() {
        let slider = Slider::default().ignore_bounds(true).value(15.0);
        assert!(slider.value == 15.0);
    }
}
