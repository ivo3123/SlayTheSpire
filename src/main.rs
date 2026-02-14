use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult, ContextBuilder};
use std::sync::Arc;

use SlayTheSpire::ui::Assets;
use SlayTheSpire::ui::screens::{
    CombatScreen, CombatAction,
    menu::{MenuScreen, MenuAction},
    map::{MapScreen, MapAction, NodeType},
    card_reward::{CardRewardScreen, CardRewardAction},
};
use SlayTheSpire::core::player::{Player, STSClass};
use SlayTheSpire::core::base_state::State;
use SlayTheSpire::core::card::Card;
use SlayTheSpire::core::enemy::Enemy;
use SlayTheSpire::core::game_state::GameState as CombatState;
use SlayTheSpire::enemies::dragonling::Dragonling;
use SlayTheSpire::cards;

enum GameScreen {
    Menu,
    Map,
    Combat,
    CardReward,
}

struct GameState {
    assets: Arc<Assets>,
    current_screen: GameScreen,
    
    menu_screen: MenuScreen,
    map_screen: Option<MapScreen>,
    combat_screen: Option<CombatScreen>,
    card_reward_screen: Option<CardRewardScreen>,
    
    player: Player,
    deck: Vec<Card>,
    deck_before_combat: Vec<Card>,
}

impl GameState {
    fn create_starting_deck() -> Vec<Card> {
        vec![
            cards::strike(1, false),
            cards::strike(2, false),
            cards::defend(3, false),
            cards::defend(4, false),
            cards::defend(5, false),
            cards::inflame(6, false),
            cards::haste(7, false),
            cards::whirlwind(8, false),
            cards::barricade(9, false),
            cards::quick_strike(10, false),
        ]
    }
    
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let assets = Arc::new(Assets::new(ctx));
        let player = Player::new(STSClass::Ironclad, "Hero".to_string(), 100);
        
        Ok(GameState {
            assets,
            current_screen: GameScreen::Menu,
            menu_screen: MenuScreen::new(),
            map_screen: None,
            combat_screen: None,
            card_reward_screen: None,
            player,
            deck: Self::create_starting_deck(),
            deck_before_combat: Vec::new(),
        })
    }
    
    fn start_new_run(&mut self) {
        self.player = Player::new(STSClass::Ironclad, "Hero".to_string(), 100);
        self.deck = Self::create_starting_deck();
        
        self.map_screen = Some(MapScreen::new());
        self.current_screen = GameScreen::Map;
    }
    
    fn enter_combat(&mut self, node_type: NodeType) {
        let enemies: Vec<Box<dyn Enemy>> = match node_type {
            NodeType::NormalCombat => vec![
                Box::new(Dragonling::new()),
            ],
            NodeType::EliteCombat => vec![
                Box::new(Dragonling::new()),
                Box::new(Dragonling::new()),
            ],
            NodeType::Boss => vec![
                Box::new(Dragonling::new()),
                Box::new(Dragonling::new()),
                Box::new(Dragonling::new()),
            ],
            NodeType::RestSite => {
                let max_hp = self.player.get_max_health();
                let current_hp = self.player.get_current_health();
                let heal_amount = (max_hp as f32 * 0.3) as i32;
                let new_hp = (current_hp + heal_amount).min(max_hp);
                self.player.set_health(new_hp);
                
                if let Some(map) = &mut self.map_screen {
                    map.advance_floor();
                }
                self.current_screen = GameScreen::Map;
                return;
            }
        };
        
        self.deck_before_combat = self.deck.clone();
        
        let combat_state = CombatState::new_with_deck(self.player.clone(), enemies, self.deck.clone());
        self.combat_screen = Some(CombatScreen::new_with_state(combat_state, &self.assets));
        self.current_screen = GameScreen::Combat;
    }
    
    fn combat_victory(&mut self) {
        if let Some(combat) = &self.combat_screen {
            self.player = combat.get_player().clone();
            
            self.player.clear_all_statuses();
            self.player.clear_all_modifiers();
            
            self.deck = self.deck_before_combat.clone();
        }
        
        self.card_reward_screen = Some(CardRewardScreen::new(self.assets.clone()));
        self.current_screen = GameScreen::CardReward;
    }
    
    fn combat_defeat(&mut self) {
        self.current_screen = GameScreen::Menu;
        self.map_screen = None;
        self.combat_screen = None;
        self.card_reward_screen = None;
    }
    
    fn finish_reward(&mut self, card: Option<Card>) {
        if let Some(card) = card {
            self.deck.push(card);
        }
        
        let defeated_boss = if let Some(map) = &self.map_screen {
            map.is_on_last_node()
        } else {
            false
        };
        
        if defeated_boss {
            self.current_screen = GameScreen::Menu;
            self.map_screen = None;
            self.combat_screen = None;
            self.card_reward_screen = None;
        } else {
            if let Some(map) = &mut self.map_screen {
                map.advance_floor();
            }
            self.current_screen = GameScreen::Map;
            self.card_reward_screen = None;
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.current_screen {
            GameScreen::Menu => {
                let action = self.menu_screen.update(ctx)?;
                match action {
                    MenuAction::StartRun => {
                        self.start_new_run();
                    }
                    MenuAction::Quit => {
                        ctx.request_quit();
                    }
                    MenuAction::None => {}
                }
            }
            GameScreen::Map => {
                if let Some(map) = &mut self.map_screen {
                    let action = map.update(ctx)?;
                    match action {
                        MapAction::EnterNode(node_type) => {
                            self.enter_combat(node_type);
                        }
                        MapAction::None => {}
                    }
                }
            }
            GameScreen::Combat => {
                if let Some(combat) = &mut self.combat_screen {
                    let action = combat.update(ctx)?;
                    match action {
                        CombatAction::Victory => {
                            self.combat_victory();
                        }
                        CombatAction::Defeat => {
                            self.combat_defeat();
                        }
                        CombatAction::None => {}
                    }
                }
            }
            GameScreen::CardReward => {
                if let Some(reward) = &mut self.card_reward_screen {
                    let action = reward.update(ctx)?;
                    match action {
                        CardRewardAction::SelectCard(card) => {
                            self.finish_reward(Some(card));
                        }
                        CardRewardAction::Skip => {
                            self.finish_reward(None);
                        }
                        CardRewardAction::None => {}
                    }
                }
            }
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(20, 20, 30));
        
        match self.current_screen {
            GameScreen::Menu => {
                self.menu_screen.draw(ctx, &mut canvas)?;
            }
            GameScreen::Map => {
                if let Some(map) = &mut self.map_screen {
                    map.draw(ctx, &mut canvas, &self.player)?;
                }
            }
            GameScreen::Combat => {
                if let Some(combat) = &mut self.combat_screen {
                    combat.draw(ctx, &mut canvas)?;
                }
            }
            GameScreen::CardReward => {
                if let Some(reward) = &mut self.card_reward_screen {
                    reward.draw(ctx, &mut canvas)?;
                }
            }
        }
        
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match self.current_screen {
            GameScreen::Menu => {
                let action = self.menu_screen.mouse_button_down(ctx, button, x, y)?;
                match action {
                    MenuAction::StartRun => {
                        self.start_new_run();
                    }
                    MenuAction::Quit => {
                        ctx.request_quit();
                    }
                    MenuAction::None => {}
                }
            }
            GameScreen::Map => {
                if let Some(map) = &mut self.map_screen {
                    let action = map.mouse_button_down(ctx, button, x, y)?;
                    match action {
                        MapAction::EnterNode(node_type) => {
                            self.enter_combat(node_type);
                        }
                        MapAction::None => {}
                    }
                }
            }
            GameScreen::Combat => {
                if let Some(combat) = &mut self.combat_screen {
                    let action = combat.mouse_button_down(ctx, button, x, y)?;
                    match action {
                        CombatAction::Victory => {
                            self.combat_victory();
                        }
                        CombatAction::Defeat => {
                            self.combat_defeat();
                        }
                        CombatAction::None => {}
                    }
                }
            }
            GameScreen::CardReward => {
                if let Some(reward) = &mut self.card_reward_screen {
                    let action = reward.mouse_button_down(ctx, button, x, y)?;
                    match action {
                        CardRewardAction::SelectCard(card) => {
                            self.finish_reward(Some(card));
                        }
                        CardRewardAction::Skip => {
                            self.finish_reward(None);
                        }
                        CardRewardAction::None => {}
                    }
                }
            }
        }
        
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        match self.current_screen {
            GameScreen::Combat => {
                if let Some(combat) = &mut self.combat_screen {
                    combat.mouse_move(ctx, x, y);
                }
            }
            GameScreen::CardReward => {
                if let Some(reward) = &mut self.card_reward_screen {
                    reward.mouse_move(ctx, x, y);
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("slay_the_spire", "Ivaylo")
        .window_setup(ggez::conf::WindowSetup::default().title("Slay the Spire"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1400.0, 800.0))
        .add_resource_path("assets")
        .build()?;

    let game_state = GameState::new(&mut ctx)?;
    
    event::run(ctx, event_loop, game_state)
}
