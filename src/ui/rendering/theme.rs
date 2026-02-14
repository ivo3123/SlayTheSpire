use ggez::graphics::Color;

#[derive(Clone, Debug)]
pub struct Theme {
    pub text: Color,
    pub text_secondary: Color,
    
    pub background: Color,
    pub button: Color,
    pub button_hover: Color,
    
    pub card_bg: Color,
    pub card_border: Color,
    pub card_selected: Color,
    
    pub player_health: Color,
    pub enemy_health: Color,
    
    pub block_color: Color,
    pub energy_color: Color,
    pub vulnerable_color: Color,
    pub weak_color: Color,
    pub strength_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            text: Color::from_rgb(240, 240, 240),
            text_secondary: Color::from_rgb(180, 180, 180),
            
            background: Color::from_rgb(20, 20, 30),
            button: Color::from_rgb(60, 60, 80),
            button_hover: Color::from_rgb(80, 80, 100),
            
            card_bg: Color::from_rgb(40, 40, 50),
            card_border: Color::from_rgb(200, 180, 100),
            card_selected: Color::from_rgb(255, 215, 0),
            
            player_health: Color::from_rgb(100, 200, 100),
            enemy_health: Color::from_rgb(200, 50, 50),
            
            block_color: Color::from_rgb(100, 150, 255),
            energy_color: Color::from_rgb(100, 200, 255),
            vulnerable_color: Color::from_rgb(100, 255, 100),
            weak_color: Color::from_rgb(255, 200, 100),
            strength_color: Color::from_rgb(255, 100, 100),
        }
    }
}
