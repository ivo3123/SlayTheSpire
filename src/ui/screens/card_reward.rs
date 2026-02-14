use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::sync::Arc;
use rand::seq::SliceRandom;

use crate::ui::rendering::theme::Theme;
use crate::ui::rendering::assets::Assets;
use crate::ui::rendering::card_renderer::{draw_card, CardRenderConfig};
use crate::core::card::Card;
use crate::cards;

pub struct CardRewardScreen {
    theme: Theme,
    assets: Arc<Assets>,
    card_choices: Vec<Card>,
    selected_card: Option<usize>,
    hovering_card: Option<usize>,
}

impl CardRewardScreen {
    pub fn new(assets: Arc<Assets>) -> Self {
        let mut rng = rand::thread_rng();
        let available_cards = vec![
            cards::strike(0, false),
            cards::defend(0, false),
            cards::inflame(0, false),
            cards::haste(0, false),
            cards::whirlwind(0, false),
            cards::barricade(0, false),
        ];

        let card_choices: Vec<Card> = available_cards
            .choose_multiple(&mut rng, 3)
            .cloned()
            .collect();

        CardRewardScreen {
            theme: Theme::default(),
            assets,
            card_choices,
            selected_card: None,
            hovering_card: None,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<CardRewardAction> {
        Ok(CardRewardAction::None)
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let bg_rect = Rect::new(0.0, 0.0, 1400.0, 800.0);
        let bg_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            bg_rect,
            Color::from_rgb(20, 20, 30),
        )?;
        canvas.draw(&bg_mesh, DrawParam::default());

        let mut title = Text::new("Choose a Card Reward");
        title.set_scale(48.0);
        canvas.draw(&title, DrawParam::default().dest([400.0, 50.0]).color(self.theme.text));

        let skip_button_rect = Rect::new(600.0, 650.0, 200.0, 60.0);
        let skip_button_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            skip_button_rect,
            self.theme.button,
        )?;
        canvas.draw(&skip_button_mesh, DrawParam::default());

        let skip_border = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::stroke(2.0),
            skip_button_rect,
            self.theme.card_border,
        )?;
        canvas.draw(&skip_border, DrawParam::default());

        let mut skip_text = Text::new("Skip");
        skip_text.set_scale(28.0);
        canvas.draw(&skip_text, DrawParam::default().dest([665.0, 670.0]).color(self.theme.text));

        let card_width = 200.0;
        let card_height = 270.0;
        let card_spacing = 100.0;
        let start_x = 350.0;
        let start_y = 250.0;

        for (i, card) in self.card_choices.iter().enumerate() {
            let x = start_x + i as f32 * (card_width + card_spacing);
            let y = start_y;

            let config = CardRenderConfig::new(
                x,
                y,
                card_width,
                card_height,
                self.theme.clone()
            )
            .with_selected(Some(i) == self.selected_card)
            .with_hovering(Some(i) == self.hovering_card);

            draw_card(ctx, canvas, card, &config, &self.assets)?;
        }

        if self.selected_card.is_some() {
            let mut hint_text = Text::new("Click again to confirm");
            hint_text.set_scale(20.0);
            canvas.draw(&hint_text, DrawParam::default().dest([550.0, 580.0]).color(Color::from_rgb(255, 215, 0)));
        }

        Ok(())
    }

    pub fn mouse_button_down(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<CardRewardAction> {
        if button != MouseButton::Left {
            return Ok(CardRewardAction::None);
        }

        let skip_button_rect = Rect::new(600.0, 650.0, 200.0, 60.0);
        if x >= skip_button_rect.x && x <= skip_button_rect.x + skip_button_rect.w &&
           y >= skip_button_rect.y && y <= skip_button_rect.y + skip_button_rect.h {
            return Ok(CardRewardAction::Skip);
        }

        let card_width = 200.0;
        let card_height = 270.0;
        let card_spacing = 100.0;
        let start_x = 350.0;
        let start_y = 250.0;

        for i in 0..self.card_choices.len() {
            let card_x = start_x + i as f32 * (card_width + card_spacing);
            let card_y = start_y;
            let card_rect = Rect::new(card_x, card_y, card_width, card_height);

            if x >= card_rect.x && x <= card_rect.x + card_rect.w &&
               y >= card_rect.y && y <= card_rect.y + card_rect.h {
                if Some(i) == self.selected_card {
                    // second click confirms selection
                    return Ok(CardRewardAction::SelectCard(self.card_choices[i].clone()));
                } else {
                    // first click
                    self.selected_card = Some(i);
                    return Ok(CardRewardAction::None);
                }
            }
        }

        Ok(CardRewardAction::None)
    }

    pub fn mouse_move(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        let card_width = 200.0;
        let card_height = 270.0;
        let card_spacing = 100.0;
        let start_x = 350.0;
        let start_y = 250.0;

        self.hovering_card = None;
        for i in 0..self.card_choices.len() {
            let card_x = start_x + i as f32 * (card_width + card_spacing);
            let card_y = start_y;
            let card_rect = Rect::new(card_x, card_y, card_width, card_height);

            if x >= card_rect.x && x <= card_rect.x + card_rect.w &&
               y >= card_rect.y && y <= card_rect.y + card_rect.h {
                self.hovering_card = Some(i);
                break;
            }
        }
    }
}

pub enum CardRewardAction {
    None,
    SelectCard(Card),
    Skip,
}
