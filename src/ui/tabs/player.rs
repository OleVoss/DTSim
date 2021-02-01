use crossterm::cursor::MoveDown;
use tui::{layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, BorderType, Borders, Gauge, List, ListItem, ListState}};
use crate::{app::App, ui::components::DrawableComponent};

#[derive(Clone, Copy)]
pub struct Player {
    name: &'static str,
}

#[derive(Clone)]
pub struct PlayerTab {
    visible: bool,
    pub selected: usize,
}

impl PlayerTab {
    pub fn new() -> Self {
        Self {
            visible: false,
            selected: 0,
        }
    }
}

impl DrawableComponent for PlayerTab {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &App
    ) -> anyhow::Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                ]
                .as_ref()
            )
            .split(rect);

        // player section

        // TODO: call the draw() function of the specific components
        let player_block = Block::default()
            .title("Player")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let items: Vec<ListItem> = app.player_roaster.player_list
            .iter()
            .map(|p| ListItem::new(p.name))
            .collect();

        let player_list = List::new(items)
            .block(player_block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
            .highlight_symbol("[");

        let mut list_state = ListState::default();
        list_state.select(Some(self.selected));


        // stats section

        let stats_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref()
            )
            .split(main_chunks[1]);

        let stats_block = Block::default()
            .title("Stats")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let player_strength: i32 = match app.player_roaster.streng_by_index(self.selected) {
            Some(strength) => strength,
            None => 0,
        };

        let stats_gauge = Gauge::default()
            .block(stats_block)
            .gauge_style(Style::default().fg(Color::Red))
            .percent(player_strength as u16);

        // bag section

        let bag_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(75),
                    Constraint::Percentage(25),
                ].as_ref()
            )
            .split(main_chunks[2]);

        let bag_block = Block::default()
            .title("Bag")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let path_block = Block::default()
            .title("Path")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);


        // render section

        f.render_stateful_widget(player_list, main_chunks[0], &mut list_state);
        f.render_widget(stats_gauge.clone(), stats_chunks[0]);
        f.render_widget(stats_gauge.clone(), stats_chunks[1]);
        f.render_widget(stats_gauge.clone(), stats_chunks[2]);
        f.render_widget(stats_gauge, stats_chunks[3]);
        f.render_widget(bag_block, bag_chunks[1]);
        f.render_widget(path_block, bag_chunks[0]);


        Ok(())
    }
}