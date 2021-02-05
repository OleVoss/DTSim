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

        buf.set_style(slider_area, self.style);

        for x in slider_area.left()..slider_area.right() {
            buf.get_mut(x, 4).set_symbol(symbols::block::THREE_QUARTERS);
        }
    }
}

#[cfg(test)]
mod tests {}
