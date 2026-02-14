use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use crate::ui::rendering::theme::Theme;
use crate::core::player::Player;
use crate::core::base_state::State;

#[derive(Clone, Debug)]
pub struct MapNode {
    pub node_type: NodeType,
    pub x: f32,
    pub y: f32,
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    NormalCombat,
    EliteCombat,
    Boss,
    RestSite,
}

pub struct MapScreen {
    theme: Theme,
    floor: usize,
    nodes: Vec<MapNode>,
    current_node_index: usize,
}

impl MapScreen {
    pub fn new() -> Self {
        let nodes = vec![
            MapNode {
                node_type: NodeType::NormalCombat,
                x: 200.0,
                y: 250.0,
                completed: false,
            },
            MapNode {
                node_type: NodeType::NormalCombat,
                x: 400.0,
                y: 250.0,
                completed: false,
            },
            MapNode {
                node_type: NodeType::RestSite,
                x: 600.0,
                y: 250.0,
                completed: false,
            },
            MapNode {
                node_type: NodeType::EliteCombat,
                x: 800.0,
                y: 250.0,
                completed: false,
            },
            MapNode {
                node_type: NodeType::NormalCombat,
                x: 1000.0,
                y: 250.0,
                completed: false,
            },
            MapNode {
                node_type: NodeType::Boss,
                x: 600.0,
                y: 450.0,
                completed: false,
            },
        ];

        MapScreen {
            theme: Theme::default(),
            floor: 1,
            nodes,
            current_node_index: 0,
        }
    }

    pub fn advance_floor(&mut self) {
        self.floor += 1;
        if self.current_node_index < self.nodes.len() {
            self.nodes[self.current_node_index].completed = true;
            self.current_node_index += 1;
        }
    }
    
    pub fn is_on_last_node(&self) -> bool {
        self.current_node_index == self.nodes.len() - 1
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<MapAction> {
        Ok(MapAction::None)
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, player: &Player) -> GameResult {
        let bg_rect = Rect::new(0.0, 0.0, 1400.0, 800.0);
        let bg_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            bg_rect,
            Color::from_rgb(20, 20, 30),
        )?;
        canvas.draw(&bg_mesh, DrawParam::default());

        let mut floor_text = Text::new(format!("Floor {}", self.floor));
        floor_text.set_scale(40.0);
        canvas.draw(&floor_text, DrawParam::default().dest([50.0, 30.0]).color(self.theme.text));

        let stats_text = format!(
            "HP: {}/{}",
            player.get_current_health(),
            player.get_max_health()
        );
        let mut stats = Text::new(stats_text);
        stats.set_scale(24.0);
        canvas.draw(&stats, DrawParam::default().dest([50.0, 90.0]).color(self.theme.text_secondary));

        for i in 0..self.nodes.len() - 1 {
            let node1 = &self.nodes[i];
            let node2 = &self.nodes[i + 1];
            
            let line_color = if node1.completed {
                Color::from_rgba(100, 150, 200, 150)
            } else {
                Color::from_rgba(60, 80, 100, 100)
            };

            let line = Mesh::new_line(
                ctx,
                &[
                    [node1.x + 75.0, node1.y + 75.0],
                    [node2.x + 75.0, node2.y + 75.0],
                ],
                3.0,
                line_color,
            )?;
            canvas.draw(&line, DrawParam::default());
        }

        for (i, node) in self.nodes.iter().enumerate() {
            let node_rect = Rect::new(node.x, node.y, 150.0, 150.0);
            
            let is_current = i == self.current_node_index && !node.completed;
            let is_available = i == self.current_node_index;
            
            let node_color = if node.completed {
                Color::from_rgba(100, 100, 100, 200)
            } else if is_current {
                match node.node_type {
                    NodeType::NormalCombat => Color::from_rgb(100, 150, 200),
                    NodeType::EliteCombat => Color::from_rgb(255, 200, 100),
                    NodeType::Boss => Color::from_rgb(255, 100, 100),
                    NodeType::RestSite => self.theme.player_health,
                }
            } else {
                Color::from_rgba(40, 40, 50, 180)
            };

            let node_mesh = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                node_rect,
                node_color,
            )?;
            canvas.draw(&node_mesh, DrawParam::default());

            let border_width = if is_available { 4.0 } else { 2.0 };
            let border_color = if is_available {
                Color::from_rgb(255, 255, 255)
            } else {
                self.theme.card_border
            };

            let border_mesh = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::stroke(border_width),
                node_rect,
                border_color,
            )?;
            canvas.draw(&border_mesh, DrawParam::default());

            let label = match node.node_type {
                NodeType::NormalCombat => "Normal Combat",
                NodeType::EliteCombat => "Elite Combat",
                NodeType::Boss => "Boss",
                NodeType::RestSite => "Rest Site",
            };

            let mut node_text = Text::new(label);
            node_text.set_scale(18.0);
            let text_color = if node.completed {
                Color::from_rgba(150, 150, 150, 200)
            } else {
                self.theme.text
            };
            canvas.draw(&node_text, DrawParam::default().dest([node.x + 10.0, node.y + 60.0]).color(text_color));
        }

        Ok(())
    }

    pub fn mouse_button_down(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<MapAction> {
        if button != MouseButton::Left {
            return Ok(MapAction::None);
        }

        if self.current_node_index < self.nodes.len() {
            let node = &self.nodes[self.current_node_index];
            if !node.completed {
                let node_rect = Rect::new(node.x, node.y, 150.0, 150.0);
                if x >= node_rect.x && x <= node_rect.x + node_rect.w &&
                   y >= node_rect.y && y <= node_rect.y + node_rect.h {
                    return Ok(MapAction::EnterNode(node.node_type.clone()));
                }
            }
        }

        Ok(MapAction::None)
    }
}

pub enum MapAction {
    None,
    EnterNode(NodeType),
}
