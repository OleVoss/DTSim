use core::num;

use ron::Number;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

use super::{number_box, NumberBox};

pub struct BoxListState {
    selected: Option<usize>,
}

impl Default for BoxListState {
    fn default() -> Self {
        Self { selected: None }
    }
}

impl BoxListState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

pub struct BoxList<'a> {
    block: Option<Block<'a>>,
    items: Vec<NumberBox<'a>>,
    style: Style,
}

impl<'a> BoxList<'a> {
    pub fn new<T>(items: T) -> Self
    where
        T: Into<Vec<NumberBox<'a>>>,
    {
        Self {
            block: None,
            style: Style::default(),
            items: items.into(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        return self;
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> StatefulWidget for BoxList<'a> {
    type State = BoxListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let mut list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for (i, mut num_box) in self.items.into_iter().enumerate() {
            let box_area = Rect::new(list_area.x, list_area.y, list_area.width, number_box::HIGHT);
            num_box.render(box_area, buf);
            list_area.y += number_box::HIGHT;
        }
    }
}
