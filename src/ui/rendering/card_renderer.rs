use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text, TextFragment};
use ggez::{Context, GameResult};
use regex::Regex;

use crate::core::card::{Card, Cost, CardType};
use super::theme::Theme;
use super::assets::Assets;

pub struct CardRenderConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub selected: bool,
    pub hovering: bool,
    pub scale: f32,
    pub theme: Theme,
    pub player_strength: i32,
}

impl CardRenderConfig {
    pub fn new(x: f32, y: f32, width: f32, height: f32, theme: Theme) -> Self {
        CardRenderConfig {
            x,
            y,
            width,
            height,
            selected: false,
            hovering: false,
            scale: 1.0,
            theme,
            player_strength: 0,
        }
    }
    
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    
    pub fn with_hovering(mut self, hovering: bool) -> Self {
        self.hovering = hovering;
        self
    }
    
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    
    pub fn with_player_strength(mut self, strength: i32) -> Self {
        self.player_strength = strength;
        self
    }
}

pub fn draw_card(
    ctx: &mut Context,
    canvas: &mut Canvas,
    card: &Card,
    config: &CardRenderConfig,
    assets: &Assets,
) -> GameResult {
    let final_width = config.width * config.scale;
    let final_height = config.height * config.scale;
    
    let y_offset = if config.hovering { -30.0 } else { 0.0 };
    let final_x = config.x;
    let final_y = config.y + y_offset;
    
    let card_rect = Rect::new(final_x, final_y, final_width, final_height);
    let bg_color = config.theme.card_bg;
    let bg_mesh = Mesh::new_rectangle(
        ctx,
        ggez::graphics::DrawMode::fill(),
        card_rect,
        bg_color,
    )?;
    canvas.draw(&bg_mesh, DrawParam::default());
    
    let card_id = card.id();
    if let Some(art) = assets.get_card_art(card_id) {
        let art_width = final_width * 0.80;
        let art_height = final_height * 0.40;
        let art_x = final_x + (final_width - art_width) / 2.0;
        let art_y = final_y + 35.0;
        
        canvas.draw(
            art,
            DrawParam::default()
                .dest([art_x, art_y])
                .scale([art_width / art.width() as f32, art_height / art.height() as f32])
        );
    }
    
    let cost_text = match card.base_cost() {
        Cost::Fixed(n) => n.to_string(),
        Cost::X => "X".to_string(),
        Cost::Free => "0".to_string(),
        Cost::Unplayable => "-".to_string(),
    };
    
    let cost_size = 25.0;
    let cost_circle_x = final_x + 15.0;
    let cost_circle_y = final_y + 15.0;
    
    let cost_circle = Mesh::new_circle(
        ctx,
        ggez::graphics::DrawMode::fill(),
        [cost_circle_x, cost_circle_y],
        cost_size / 2.0,
        0.1,
        config.theme.energy_color,
    )?;
    canvas.draw(&cost_circle, DrawParam::default());
    
    let cost_label = Text::new(TextFragment {
        text: cost_text,
        color: Some(Color::WHITE),
        font: None,
        scale: Some(ggez::graphics::PxScale::from(18.0)),
    });
    canvas.draw(&cost_label, DrawParam::default().dest([cost_circle_x - 5.0, cost_circle_y - 8.0]));
    
    let name_y = final_y + 8.0;
    let name_color = if card.is_upgraded() {
        Color::from_rgb(100, 255, 100)
    } else {
        config.theme.text
    };

    let px_scale = 15.0;
    let name_font_size = if card.name().len() > 10 {
        px_scale + 1.0
    } else {
        px_scale + 3.5
    };
    let name_text = Text::new(TextFragment {
        text: card.name().to_string(),
        color: Some(name_color),
        font: None,
        scale: Some(ggez::graphics::PxScale::from(name_font_size)),
    });
    canvas.draw(&name_text, DrawParam::default().dest([final_x + 45.0, name_y]));
    
    let card_type_text = match card.card_type() {
        CardType::Attack => "Attack",
        CardType::Skill => "Skill",
        CardType::Power => "Power",
    };
    let card_type_color = match card.card_type() {
        CardType::Attack => Color::from_rgb(255, 100, 100),
        CardType::Skill => Color::from_rgb(100, 150, 255),
        CardType::Power => Color::from_rgb(150, 100, 255),
    };
    let type_y = final_y + final_height * 0.58;
    let type_text = Text::new(TextFragment {
        text: card_type_text.to_string(),
        color: Some(card_type_color),
        font: None,
        scale: Some(ggez::graphics::PxScale::from(px_scale + 1.0)),
    });
    canvas.draw(&type_text, DrawParam::default().dest([final_x + 10.0, type_y]));
    
    let desc_y = final_y + final_height * 0.65;
    let desc_max_width = final_width - 20.0;
    
    if config.player_strength != 0 && *card.card_type() == CardType::Attack {
        draw_description_with_boosted_damage(
            canvas,
            card.description(),
            config.player_strength,
            final_x + 10.0,
            desc_y,
            desc_max_width,
            px_scale,
            config.theme.text_secondary
        );
    } else {
        let wrapped_description = word_wrap(card.description(), desc_max_width, px_scale);
        let desc_text = Text::new(TextFragment {
            text: wrapped_description,
            color: Some(config.theme.text_secondary),
            font: None,
            scale: Some(ggez::graphics::PxScale::from(px_scale)),
        });
        canvas.draw(&desc_text, DrawParam::default().dest([final_x + 10.0, desc_y]));
    }
    
    if card.exhaust() {
        let exhaust_text = Text::new(TextFragment {
            text: "Exhaust".to_string(),
            color: Some(Color::from_rgb(200, 100, 100)),
            font: None,
            scale: Some(ggez::graphics::PxScale::from(px_scale)),
        });
        canvas.draw(&exhaust_text, DrawParam::default().dest([final_x + 10.0, final_y + final_height - 18.0]));
    }
    
    let border_color = if config.selected {
        config.theme.card_selected
    } else if config.hovering {
        Color::from_rgb(255, 255, 255)
    } else {
        config.theme.card_border
    };
    
    let border_width = if config.selected || config.hovering { 3.0 } else { 2.0 };
    let border_mesh = Mesh::new_rectangle(
        ctx,
        ggez::graphics::DrawMode::stroke(border_width),
        card_rect,
        border_color,
    )?;
    canvas.draw(&border_mesh, DrawParam::default());
    
    Ok(())
}

fn word_wrap(text: &str, max_width: f32, font_size: f32) -> String {
    let max_chars = (max_width / (font_size * 0.6)) as usize;
    let words: Vec<&str> = text.split_whitespace().collect();
    
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        if test_line.len() > max_chars && !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    lines.join("\n")
}

fn draw_description_with_boosted_damage(
    canvas: &mut Canvas,
    description: &str,
    strength: i32,
    x: f32,
    y: f32,
    max_width: f32,
    font_size: f32,
    default_color: Color,
) {
    let wrapped = word_wrap(description, max_width, font_size);
    
    let re = Regex::new(r"(\d+)\s+(damage)").unwrap();
    let mut text = Text::new("");
    let mut last_end = 0;
    
    for cap in re.captures_iter(&wrapped) {
        let full_match = cap.get(0).unwrap();
        let num_str = cap.get(1).unwrap().as_str();
        let num: i32 = num_str.parse().unwrap();
        let boosted = num + strength;
        
        if full_match.start() > last_end {
            text.add(TextFragment {
                text: wrapped[last_end..full_match.start()].to_string(),
                color: Some(default_color),
                font: None,
                scale: Some(ggez::graphics::PxScale::from(font_size)),
            });
        }
        
        text.add(TextFragment {
            text: boosted.to_string(),
            color: Some(Color::from_rgb(100, 255, 100)),
            font: None,
            scale: Some(ggez::graphics::PxScale::from(font_size)),
        });
        
        text.add(TextFragment {
            text: format!(" {}", cap.get(2).unwrap().as_str()),
            color: Some(default_color),
            font: None,
            scale: Some(ggez::graphics::PxScale::from(font_size)),
        });
        
        last_end = full_match.end();
    }
    
    if last_end < wrapped.len() {
        text.add(TextFragment {
            text: wrapped[last_end..].to_string(),
            color: Some(default_color),
            font: None,
            scale: Some(ggez::graphics::PxScale::from(font_size)),
        });
    }
    
    canvas.draw(&text, DrawParam::default().dest([x, y]));
}
