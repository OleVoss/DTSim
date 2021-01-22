use std::default;

use tui::{Frame, backend::Backend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, BorderType, Borders, List, ListItem}};
use anyhow::Result;


pub struct App<'a> {
    pub title: &'a str,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    
                ].as_ref(),
            )
            .split(fsize);

            let block = Block::default()
                .title("Config")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded);


            let list = List::new([ListItem::new("config 1")])
                .block(block);

            f.render_widget(list, fsize);

        Ok(())
    }
}