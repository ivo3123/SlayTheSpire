use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::sync::Arc;

use crate::core::{GameState, EntityId, Player};
use crate::core::base_state::{State, StatusType};
use crate::core::enemy::Enemy;
use crate::core::card::{Card, CardTargeting};
use super::super::rendering::{Theme, Assets, draw_card, CardRenderConfig};

pub struct CombatScreen {
    theme: Theme,
    assets: Arc<Assets>,
    game_state: GameState,
    
    selected_card_index: Option<usize>,
    hovering_card_index: Option<usize>,
    hovering_enemy_index: Option<usize>,
    animation_timer: f32,
    current_animation_frame: usize,
}

impl CombatScreen {
    pub fn new_with_state(mut game_state: GameState, assets: &Arc<Assets>) -> Self {
        game_state.start_player_turn();
        
        CombatScreen {
            theme: Theme::default(),
            assets: Arc::clone(assets),
            game_state,
            selected_card_index: None,
            hovering_card_index: None,
            hovering_enemy_index: None,
            animation_timer: 0.0,
            current_animation_frame: 0,
        }
    }
    
    pub fn get_player(&self) -> &Player {
        self.game_state.player()
    }
    
    pub fn get_all_cards(&self) -> Vec<Card> {
        let mut all_cards = Vec::new();
        all_cards.extend(self.game_state.hand().iter().cloned());
        all_cards.extend(self.game_state.draw_pile().iter().cloned());
        all_cards.extend(self.game_state.discard_pile().iter().cloned());
        all_cards.extend(self.game_state.exhaust_pile().iter().cloned());
        all_cards
    }
    
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<CombatAction> {
        self.animation_timer += ctx.time.delta().as_secs_f32();
        if self.animation_timer >= 0.15 {
            self.animation_timer = 0.0;
            self.current_animation_frame += 1;
        }
        
        Ok(CombatAction::None)
    }
    
    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.draw_player(ctx, canvas)?;
        self.draw_enemies(ctx, canvas)?;
        self.draw_hand(ctx, canvas)?;
        self.draw_energy(ctx, canvas)?;
        self.draw_pile_info(ctx, canvas)?;
        self.draw_end_turn_button(ctx, canvas)?;
        self.draw_hero_ability_button(ctx, canvas)?;
        if self.game_state.is_combat_over() || !self.game_state.player().is_alive() {
            self.draw_game_over_overlay(ctx, canvas)?;
        }
        
        Ok(())
    }
    
    fn draw_player(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let player = self.game_state.player();
        let player_x = 50.0;
        let player_y = 200.0;
        let player_w = 180.0;
        let player_h = 180.0;
        
        let animation_key = "player_idle";
        let frame_count = self.assets.get_animation_frame_count(animation_key);
        
        if frame_count > 0 {
            if let Some(image) = self.assets.get_animation_frame(animation_key, self.current_animation_frame) {
                canvas.draw(
                    image,
                    DrawParam::default()
                        .dest([player_x, player_y])
                        .scale([player_w / image.width() as f32, player_h / image.height() as f32])
                );
            }
        } else {
            let player_rect = Rect::new(player_x, player_y, player_w, player_h);
            let player_mesh = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                player_rect,
                Color::from_rgb(50, 100, 200),
            )?;
            canvas.draw(&player_mesh, DrawParam::default());
            
            let border_mesh = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::stroke(3.0),
                player_rect,
                self.theme.player_health,
            )?;
            canvas.draw(&border_mesh, DrawParam::default());
        }
        
        let health_y = player_y + player_h + 10.0;
        let health_bar_width = player_w;
        let health_bar_height = 20.0;
        
        let health_bg = Rect::new(player_x, health_y, health_bar_width, health_bar_height);
        let bg_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            health_bg,
            Color::from_rgb(60, 60, 60),
        )?;
        canvas.draw(&bg_mesh, DrawParam::default());
        
        let health_percent = player.get_current_health() as f32 / player.get_max_health() as f32;
        let health_fg = Rect::new(player_x, health_y, health_bar_width * health_percent, health_bar_height);
        let fg_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            health_fg,
            self.theme.player_health,
        )?;
        canvas.draw(&fg_mesh, DrawParam::default());
        
        let health_text = format!("{}/{}", player.get_current_health(), player.get_max_health());
        let mut health_label = Text::new(health_text);
        health_label.set_scale(14.0);
        canvas.draw(&health_label, DrawParam::default().dest([player_x + 25.0, health_y + 3.0]).color(self.theme.text));
        
        if player.get_block() > 0 {
            let block_x = player_x + health_bar_width + 10.0;
            let block_y = health_y;
            let block_size = 20.0;
            
            let block_rect = Rect::new(block_x, block_y, block_size, block_size);
            let block_mesh = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                block_rect,
                self.theme.block_color,
            )?;
            canvas.draw(&block_mesh, DrawParam::default());
            
            let mut block_text = Text::new(player.get_block().to_string());
            block_text.set_scale(14.0);
            canvas.draw(&block_text, DrawParam::default().dest([block_x + 3.0, block_y + 3.0]).color(Color::WHITE));
        }
        
        self.draw_player_statuses(ctx, canvas, player_x, player_y + player_h + 35.0)?;
        
        Ok(())
    }
    
    fn draw_player_statuses(&self, _ctx: &mut Context, canvas: &mut Canvas, x: f32, y: f32) -> GameResult {
        let player = self.game_state.player();
        let mut status_lines = Vec::new();
        
        let strength = player.get_status(&StatusType::Strength);
        if strength != 0 {
            status_lines.push(format!("Strength: {}{}", if strength > 0 { "+" } else { "" }, strength));
        }
        
        let vulnerable = player.get_status(&StatusType::Vulnerable);
        if vulnerable > 0 {
            status_lines.push(format!("Vulnerable: {}", vulnerable));
        }
        
        let weak = player.get_status(&StatusType::Weak);
        if weak > 0 {
            status_lines.push(format!("Weak: {}", weak));
        }
        
        if !status_lines.is_empty() {
            let status_text = status_lines.join("\n");
            let mut text = Text::new(status_text);
            text.set_scale(14.0);
            canvas.draw(&text, DrawParam::default().dest([x, y]).color(self.theme.text_secondary));
        }
        
        Ok(())
    }
    
    fn draw_enemies(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let enemies = self.game_state.enemies();
        let living_enemies: Vec<(usize, &Box<dyn Enemy>)> = enemies
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_alive())
            .collect();
        
        if living_enemies.is_empty() {
            return Ok(());
        }
        
        let enemy_w = 180.0;
        let enemy_h = 180.0;
        let total_width = living_enemies.len() as f32 * enemy_w + (living_enemies.len().saturating_sub(1)) as f32 * 30.0;
        let start_x = (1400.0 - total_width) / 2.0 + 200.0;
        
        for (visual_index, (actual_index, enemy)) in living_enemies.iter().enumerate() {
            let enemy_x = start_x + (visual_index as f32) * (enemy_w + 30.0);
            let enemy_y = 200.0;
            
            self.draw_enemy_intent(ctx, canvas, enemy, enemy_x, enemy_y - 50.0)?;
            
            let animation_key = "dragonling_idle";
            let frame_count = self.assets.get_animation_frame_count(animation_key);
            
            let hovering = self.hovering_enemy_index == Some(*actual_index);
            let scale = if hovering { 1.1 } else { 1.0 };
            let scaled_w = enemy_w * scale;
            let scaled_h = enemy_h * scale;
            let offset_x = (enemy_w - scaled_w) / 2.0;
            let offset_y = (enemy_h - scaled_h) / 2.0;
            
            if frame_count > 0 {
                if let Some(image) = self.assets.get_animation_frame(animation_key, self.current_animation_frame) {
                    canvas.draw(
                        image,
                        DrawParam::default()
                            .dest([enemy_x + offset_x, enemy_y + offset_y])
                            .scale([scaled_w / image.width() as f32, scaled_h / image.height() as f32])
                    );
                }
            } else {
                let enemy_rect = Rect::new(enemy_x + offset_x, enemy_y + offset_y, scaled_w, scaled_h);
                let mesh = Mesh::new_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    enemy_rect,
                    Color::from_rgb(200, 50, 50),
                )?;
                canvas.draw(&mesh, DrawParam::default());
                
                let border = Mesh::new_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::stroke(if hovering { 3.0 } else { 2.0 }),
                    enemy_rect,
                    if hovering { Color::WHITE } else { self.theme.enemy_health },
                )?;
                canvas.draw(&border, DrawParam::default());
            }
            
            self.draw_enemy_health(ctx, canvas, enemy, enemy_x, enemy_y + enemy_h + 10.0, enemy_w)?;
            
            self.draw_enemy_statuses(ctx, canvas, enemy, enemy_x, enemy_y + enemy_h + 35.0)?;
        }
        
        Ok(())
    }
    
    fn draw_enemy_intent(&self, _ctx: &mut Context, canvas: &mut Canvas, enemy: &Box<dyn Enemy>, x: f32, y: f32) -> GameResult {
        let turn_count = self.game_state.get_turn_count();
        let intent = enemy.get_intent(turn_count);
        let intent_text = intent.description();
        
        let mut text = Text::new(intent_text);
        text.set_scale(16.0);
        canvas.draw(&text, DrawParam::default().dest([x + 40.0, y]).color(self.theme.enemy_health));
        
        Ok(())
    }
    
    fn draw_enemy_health(&self, ctx: &mut Context, canvas: &mut Canvas, enemy: &Box<dyn Enemy>, x: f32, y: f32, width: f32) -> GameResult {
        let height = 20.0;
        
        let bg_rect = Rect::new(x, y, width, height);
        let bg_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            bg_rect,
            Color::from_rgb(60, 60, 60),
        )?;
        canvas.draw(&bg_mesh, DrawParam::default());
        
        let health_percent = enemy.get_current_health() as f32 / enemy.get_max_health() as f32;
        let fg_rect = Rect::new(x, y, width * health_percent, height);
        let fg_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            fg_rect,
            self.theme.enemy_health,
        )?;
        canvas.draw(&fg_mesh, DrawParam::default());
        
        let health_text = format!("{}/{}", enemy.get_current_health(), enemy.get_max_health());
        let mut text = Text::new(health_text);
        text.set_scale(14.0);
        canvas.draw(&text, DrawParam::default().dest([x + 50.0, y + 3.0]).color(self.theme.text));
        
        if enemy.get_block() > 0 {
            let block_x = x + width + 5.0;
            let block_size = 20.0;
            
            let block_rect = Rect::new(block_x, y, block_size, block_size);
            let block_mesh = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                block_rect,
                self.theme.block_color,
            )?;
            canvas.draw(&block_mesh, DrawParam::default());
            
            let mut block_text = Text::new(enemy.get_block().to_string());
            block_text.set_scale(12.0);
            canvas.draw(&block_text, DrawParam::default().dest([block_x + 3.0, y + 3.0]).color(Color::WHITE));
        }
        
        Ok(())
    }
    
    fn draw_enemy_statuses(&self, _ctx: &mut Context, canvas: &mut Canvas, enemy: &Box<dyn Enemy>, x: f32, y: f32) -> GameResult {
        let mut status_lines = Vec::new();
        
        let strength = enemy.get_status(&StatusType::Strength);
        if strength != 0 {
            status_lines.push(format!("Str: {}", strength));
        }
        
        let vulnerable = enemy.get_status(&StatusType::Vulnerable);
        if vulnerable > 0 {
            status_lines.push(format!("Vuln: {}", vulnerable));
        }
        
        let weak = enemy.get_status(&StatusType::Weak);
        if weak > 0 {
            status_lines.push(format!("Weak: {}", weak));
        }
        
        if !status_lines.is_empty() {
            let text_str = status_lines.join(", ");
            let mut text = Text::new(text_str);
            text.set_scale(12.0);
            canvas.draw(&text, DrawParam::default().dest([x + 10.0, y]).color(self.theme.text_secondary));
        }
        
        Ok(())
    }
    
    fn draw_hand(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let hand = self.game_state.hand();
        let card_width = 160.0;
        let card_height = 225.0;
        let card_spacing = 8.0;
        let start_x = 75.0;
        let start_y = 550.0;
        
        let player_strength = self.game_state.player().get_status(&StatusType::Strength);
        
        for (i, card) in hand.iter().enumerate() {
            let x = start_x + (i as f32) * (card_width + card_spacing);
            let config = CardRenderConfig::new(x, start_y, card_width, card_height, self.theme.clone())
                .with_selected(self.selected_card_index == Some(i))
                .with_hovering(self.hovering_card_index == Some(i))
                .with_player_strength(player_strength);
            
            draw_card(ctx, canvas, card, &config, &self.assets)?;
        }
        
        Ok(())
    }
    
    fn draw_energy(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let player = self.game_state.player();
        let energy_x = 50.0;
        let energy_y = 450.0;
        let orb_size = 40.0;
        
        let orbs_to_draw = player.get_energy().max(player.get_max_energy());
        for i in 0..orbs_to_draw {
            let x = energy_x + (i as f32) * (orb_size + 5.0);
            let filled = i < player.get_energy();
            
            let color = if filled {
                self.theme.energy_color
            } else {
                Color::from_rgba(100, 150, 200, 100)
            };
            
            let orb = Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                [x + orb_size / 2.0, energy_y + orb_size / 2.0],
                orb_size / 2.0,
                0.1,
                color,
            )?;
            canvas.draw(&orb, DrawParam::default());
        }
        
        let energy_text = format!("{}/{}", player.get_energy(), player.get_max_energy());
        let mut text = Text::new(energy_text);
        text.set_scale(18.0);
        canvas.draw(&text, DrawParam::default().dest([energy_x, energy_y + orb_size + 5.0]).color(self.theme.text));
        
        Ok(())
    }
    
    fn draw_pile_info(&self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let draw_pile_count = self.game_state.draw_pile().len();
        let discard_pile_count = self.game_state.discard_pile().len();
        let turn_count = self.game_state.get_turn_count();
        
        let x = 1250.0;
        let y = 100.0;
        
        let draw_text = format!("Draw: {}", draw_pile_count);
        let mut draw_label = Text::new(draw_text);
        draw_label.set_scale(16.0);
        canvas.draw(&draw_label, DrawParam::default().dest([x, y]).color(self.theme.text));
        
        let discard_text = format!("Discard: {}", discard_pile_count);
        let mut discard_label = Text::new(discard_text);
        discard_label.set_scale(16.0);
        canvas.draw(&discard_label, DrawParam::default().dest([x, y + 25.0]).color(self.theme.text));
        
        let turn_text = format!("Turn: {}", turn_count);
        let mut turn_label = Text::new(turn_text);
        turn_label.set_scale(16.0);
        canvas.draw(&turn_label, DrawParam::default().dest([x, y + 50.0]).color(self.theme.text));
        
        Ok(())
    }
    
    fn draw_end_turn_button(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let button_rect = self.get_end_turn_button_rect();
        
        let button_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            button_rect,
            self.theme.button,
        )?;
        canvas.draw(&button_mesh, DrawParam::default());
        
        let border_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::stroke(2.0),
            button_rect,
            self.theme.card_border,
        )?;
        canvas.draw(&border_mesh, DrawParam::default());
        
        let mut text = Text::new("End Turn");
        text.set_scale(20.0);
        canvas.draw(&text, DrawParam::default().dest([button_rect.x + 30.0, button_rect.y + 15.0]).color(self.theme.text));
        
        Ok(())
    }
    
    fn draw_hero_ability_button(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let button_rect = self.get_hero_ability_button_rect();
        let player = self.game_state.player();
        
        let disabled = player.hero_ability_used() || player.get_energy() < 1 || self.game_state.hand().is_empty();
        let color = if disabled {
            Color::from_rgb(40, 40, 50)
        } else {
            self.theme.button
        };
        
        let button_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            button_rect,
            color,
        )?;
        canvas.draw(&button_mesh, DrawParam::default());
        
        let border_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::stroke(2.0),
            button_rect,
            if disabled { Color::from_rgb(100, 100, 100) } else { self.theme.card_border },
        )?;
        canvas.draw(&border_mesh, DrawParam::default());
        
        let mut text = Text::new("Hero\nAbility");
        text.set_scale(14.0);
        canvas.draw(&text, DrawParam::default().dest([button_rect.x + 20.0, button_rect.y + 10.0]).color(if disabled { Color::from_rgb(100, 100, 100) } else { self.theme.text }));
        
        Ok(())
    }
    
    fn get_end_turn_button_rect(&self) -> Rect {
        Rect::new(1150.0, 510.0, 200.0, 50.0)
    }
    
    fn get_hero_ability_button_rect(&self) -> Rect {
        Rect::new(1150.0, 440.0, 200.0, 60.0)
    }
    
    fn draw_game_over_overlay(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let overlay_rect = Rect::new(0.0, 0.0, 1400.0, 800.0);
        let overlay_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            overlay_rect,
            Color::from_rgba(0, 0, 0, 180),
        )?;
        canvas.draw(&overlay_mesh, DrawParam::default());
        
        if !self.game_state.player().is_alive() {
            let mut text = Text::new("DEFEAT!");
            text.set_scale(72.0);
            canvas.draw(&text, DrawParam::default().dest([500.0, 300.0]).color(self.theme.enemy_health));
        } else {
            let mut text = Text::new("VICTORY!");
            text.set_scale(72.0);
            canvas.draw(&text, DrawParam::default().dest([480.0, 300.0]).color(self.theme.player_health));
        }
        
        let mut continue_text = Text::new("Click anywhere to continue");
        continue_text.set_scale(28.0);
        canvas.draw(&continue_text, DrawParam::default().dest([450.0, 420.0]).color(self.theme.text));
        
        Ok(())
    }
    
    pub fn mouse_button_down(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<CombatAction> {
        if button != MouseButton::Left {
            return Ok(CombatAction::None);
        }
        
        let is_showing_overlay = self.game_state.is_combat_over() || !self.game_state.player().is_alive();
        
        if is_showing_overlay {
            if !self.game_state.player().is_alive() {
                println!("Player dead - returning to menu");
                return Ok(CombatAction::Defeat);
            } else {
                println!("All enemies defeated - showing rewards");
                return Ok(CombatAction::Victory);
            }
        }
        
        let end_turn_rect = self.get_end_turn_button_rect();
        if x >= end_turn_rect.x && x <= end_turn_rect.x + end_turn_rect.w &&
           y >= end_turn_rect.y && y <= end_turn_rect.y + end_turn_rect.h {
            self.selected_card_index = None;
            
            self.game_state.end_player_turn();
            
            self.game_state.execute_all_enemy_turns();
            
            self.game_state.start_player_turn();
            
            return Ok(CombatAction::None);
        }
        
        let hero_ability_rect = self.get_hero_ability_button_rect();
        if x >= hero_ability_rect.x && x <= hero_ability_rect.x + hero_ability_rect.w &&
           y >= hero_ability_rect.y && y <= hero_ability_rect.y + hero_ability_rect.h {
            if let Err(err) = self.game_state.use_hero_ability() {
                println!("Hero ability failed: {}", err);
            }
            return Ok(CombatAction::None);
        }
        
        let card_width = 160.0;
        let card_height = 210.0;
        let card_spacing = 10.0;
        let start_x = 100.0;
        let start_y = 560.0;
        
        for i in 0..self.game_state.hand().len() {
            let card_x = start_x + (i as f32) * (card_width + card_spacing);
            let card_y = start_y;
            
            if x >= card_x && x <= card_x + card_width &&
               y >= card_y && y <= card_y + card_height {
                if self.selected_card_index == Some(i) {
                    let card = &self.game_state.hand()[i];
                    let needs_target = matches!(card.targeting(), CardTargeting::SingleEnemy);
                    
                    if needs_target {
                        return Ok(CombatAction::None);
                    } else {
                        if let Err(err) = self.game_state.play_card(i, None) {
                            println!("Failed to play card: {}", err);
                        }
                        self.selected_card_index = None;
                        return Ok(CombatAction::None);
                    }
                } else {
                    self.selected_card_index = Some(i);
                }
                return Ok(CombatAction::None);
            }
        }
        
        if let Some(card_index) = self.selected_card_index {
            let enemies = self.game_state.enemies();
            let living_enemies: Vec<usize> = enemies
                .iter()
                .enumerate()
                .filter(|(_, e)| e.is_alive())
                .map(|(i, _)| i)
                .collect();
            
            let enemy_w = 180.0;
            let enemy_h = 180.0;
            let total_width = living_enemies.len() as f32 * enemy_w + (living_enemies.len().saturating_sub(1)) as f32 * 30.0;
            let start_x = (1400.0 - total_width) / 2.0 + 200.0;
            
            for (visual_index, actual_index) in living_enemies.iter().enumerate() {
                let enemy_x = start_x + (visual_index as f32) * (enemy_w + 30.0);
                let enemy_y = 200.0;
                
                if x >= enemy_x && x <= enemy_x + enemy_w &&
                   y >= enemy_y && y <= enemy_y + enemy_h {
                    let target = EntityId::Enemy(*actual_index);
                    if let Err(err) = self.game_state.play_card(card_index, Some(target)) {
                        println!("Failed to play card: {}", err);
                    }
                    self.selected_card_index = None;
                    return Ok(CombatAction::None);
                }
            }
        }
        
        Ok(CombatAction::None)
    }
    
    pub fn mouse_move(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        let card_width = 160.0;
        let card_height = 210.0;
        let card_spacing = 10.0;
        let start_x = 100.0;
        let start_y = 560.0;
        
        self.hovering_card_index = None;
        for i in 0..self.game_state.hand().len() {
            let card_x = start_x + (i as f32) * (card_width + card_spacing);
            let card_y = start_y;
            
            if x >= card_x && x <= card_x + card_width &&
               y >= card_y && y <= card_y + card_height {
                self.hovering_card_index = Some(i);
                break;
            }
        }
        
        let enemies = self.game_state.enemies();
        let living_enemies: Vec<usize> = enemies
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_alive())
            .map(|(i, _)| i)
            .collect();
        
        let enemy_w = 180.0;
        let enemy_h = 180.0;
        let total_width = living_enemies.len() as f32 * enemy_w + (living_enemies.len().saturating_sub(1)) as f32 * 30.0;
        let start_x_enemies = (1400.0 - total_width) / 2.0 + 200.0;
        
        self.hovering_enemy_index = None;
        for (visual_index, actual_index) in living_enemies.iter().enumerate() {
            let enemy_x = start_x_enemies + (visual_index as f32) * (enemy_w + 30.0);
            let enemy_y = 200.0;
            
            if x >= enemy_x && x <= enemy_x + enemy_w &&
               y >= enemy_y && y <= enemy_y + enemy_h {
                self.hovering_enemy_index = Some(*actual_index);
                break;
            }
        }
    }
}

pub enum CombatAction {
    None,
    Victory,
    Defeat,
}
