use tui::{
    style::Style,
    widgets::{Block, Borders},
};

use crate::{
    keys,
    models::StatType,
    style::SharedTheme,
    ui::widgets::{DrawableComponent, Slider, SliderList, SliderListState},
};

pub struct PlayerStats {
    selection: usize,
    player_index: usize,
    pub focus: bool,
    theme: SharedTheme,
}

impl PlayerStats {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            selection: 0,
            player_index: 0,
            focus: false,
            theme,
        }
    }
}

impl DrawableComponent for PlayerStats {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        // all possible sliders;
        // TODO read from config;
        // TODO generic slider;
        let player = app.player_roaster.by_index(self.player_index);

        let strength = match player {
            Some(p) => p.stat_value(StatType::Strength),
            None => 0,
        };

        let strength_slider = Slider::default()
            .ignore_bounds(false)
            .value(strength as f64)
            .highlight_style(Style::default().fg(self.theme.strength_hightlight))
            .label("Strength")
            .block(
                Block::default()
                    .border_style(self.theme.block(self.focus))
                    .borders(Borders::ALL)
            );

        let endurance = match player {
            Some(p) => p.stat_value(StatType::Endurance),
            None => 0,
        };
        let endurance_slider = Slider::default()
            .ignore_bounds(false)
            .value(endurance as f64)
            .highlight_style(Style::default().fg(self.theme.endurance_highlight))
            .label("Endurance")
            .block(
                Block::default()
                    .border_style(self.theme.block(self.focus))
                    .borders(Borders::ALL)
            );

        let precision = match player {
            Some(p) => p.stat_value(StatType::Precision),
            None => 0,
        };
        let precision_slider = Slider::default()
            .ignore_bounds(false)
            .value(precision as f64)
            .highlight_style(Style::default().fg(self.theme.precision_highlight))
            .label("Precision")
            .block(
                Block::default()
                    .border_style(self.theme.block(self.focus))
                    .borders(Borders::ALL)
            );

        let luck = match player {
            Some(p) => p.stat_value(StatType::Luck),
            None => 0,
        };
        let luck_slider = Slider::default()
            .ignore_bounds(false)
            .value(luck as f64)
            .highlight_style(Style::default().fg(self.theme.luck_highlight))
            .label("Luck")
            .block(
                Block::default()
                    .border_style(self.theme.block(self.focus))
                    .borders(Borders::ALL)
            );

        let title = format!("Stats [{}]", keys::get_hint(app.key_config.slider_list));

        let slider_list = SliderList::new(vec![
            strength_slider,
            endurance_slider,
            precision_slider,
            luck_slider,
        ])
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(self.theme.block(self.focus)),
        )
        .style(self.theme.block(self.focus));

        let mut state = SliderListState::default();
        state.select(Some(self.selection));

        f.render_stateful_widget(slider_list, rect, &mut state);
        Ok(())
    }
}
