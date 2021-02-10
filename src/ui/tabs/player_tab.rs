use crate::{
    app::App,
    keys::{self},
    ui::widgets::{DrawableComponent, Slider, SliderList, SliderListState},
};

use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerTabSections {
    Player,
    Stats,
    Bag,
}
pub struct PlayerTab {
    visible: bool,
    pub selected: usize,
    pub slider_test_value: f64,
}

impl PlayerTab {
    pub fn new() -> Self {
        Self {
            visible: false,
            selected: 0,
            slider_test_value: 0.0,
        }
    }
}

impl DrawableComponent for PlayerTab {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &App,
    ) -> anyhow::Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(rect);

        // player section

        // TODO: call the draw() function of the specific components
        let title = format!("Player [{}]", keys::get_hint(app.key_config.player_list));
        let player_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let items: Vec<ListItem> = if app.player_roaster.player_count() > 0 {
            app.player_roaster
                .player_list
                .iter()
                .map(|p| ListItem::new(p.name.to_string()))
                .collect()
        } else {
            vec![ListItem::new("no players existing")]
        };

        let player_list = List::new(items)
            .block(player_block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
            .highlight_symbol("[");

        let mut list_state = ListState::default();
        list_state.select(Some(self.selected));

        // stats section

        let strength_slider = Slider::default()
            .ignore_bounds(false)
            .value(self.slider_test_value)
            .highlight_style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Strength"));

        let precision_slider = Slider::default()
            .ignore_bounds(false)
            .value(self.slider_test_value)
            .highlight_style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL).title("Precision"));

        let endurance_slider = Slider::default()
            .ignore_bounds(false)
            .value(self.slider_test_value)
            .highlight_style(Style::default().fg(Color::Blue))
            .block(Block::default().borders(Borders::ALL).title("Endurance"));

        let luck_slider = Slider::default()
            .ignore_bounds(false)
            .value(self.slider_test_value)
            .highlight_style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Luck"));

        let title = format!("Stats [{}]", keys::get_hint(app.key_config.slider_list));

        let slider_list = SliderList::new(vec![
            strength_slider,
            precision_slider,
            endurance_slider,
            luck_slider,
        ])
        .block(Block::default().title(title).borders(Borders::ALL));

        // bag section

        let bag_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
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
        f.render_stateful_widget(slider_list, main_chunks[1], &mut SliderListState::default());
        f.render_widget(bag_block, bag_chunks[1]);
        f.render_widget(path_block, bag_chunks[0]);

        Ok(())
    }
}
