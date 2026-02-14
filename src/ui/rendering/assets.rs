use ggez::graphics::Image;
use ggez::Context;
use std::collections::HashMap;

pub struct Assets {
    images: HashMap<String, Image>,
    animations: HashMap<String, Vec<Image>>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Self {
        let mut assets = Assets {
            images: HashMap::new(),
            animations: HashMap::new(),
        };
        
        assets.load_all(ctx);
        assets
    }
    
    fn load_all(&mut self, ctx: &mut Context) {
        self.load_card_art(ctx, "strike");
        self.load_card_art(ctx, "defend");
        self.load_card_art(ctx, "barricade");
        self.load_card_art(ctx, "haste");
        self.load_card_art(ctx, "inflame");
        self.load_card_art(ctx, "whirlwind");
        self.load_card_art(ctx, "quick_strike");
        
        self.load_animation(ctx, "dragonling_idle", "enemies/art/dragonling", 4);
        self.load_animation(ctx, "player_idle", "player/art", 10);
    }
    
    fn load_card_art(&mut self, ctx: &mut Context, card_name: &str) {
        let path = format!("/cards/art/{}.png", card_name);
        if let Ok(image) = Image::from_path(ctx, &path) {
            self.images.insert(format!("card_{}", card_name), image);
        } else {
            eprintln!("Warning: Failed to load card art: {}", path);
        }
    }
    
    fn load_animation(&mut self, ctx: &mut Context, anim_name: &str, base_path: &str, frame_count: usize) {
        let mut frames = Vec::new();
        for i in 1..=frame_count {
            let path = format!("/{}/idle{}.png", base_path, i);
            if let Ok(image) = Image::from_path(ctx, &path) {
                frames.push(image);
            } else {
                eprintln!("Warning: Failed to load animation frame: {}", path);
            }
        }
        if !frames.is_empty() {
            self.animations.insert(anim_name.to_string(), frames);
        }
    }
    
    pub fn get_image(&self, key: &str) -> Option<&Image> {
        self.images.get(key)
    }
    
    pub fn get_card_art(&self, card_id: &str) -> Option<&Image> {
        self.images.get(&format!("card_{}", card_id))
    }
    
    pub fn get_animation_frame(&self, anim_name: &str, frame_index: usize) -> Option<&Image> {
        self.animations.get(anim_name).and_then(|frames| {
            if frames.is_empty() {
                None
            } else {
                Some(&frames[frame_index % frames.len()])
            }
        })
    }
    
    pub fn get_animation_frame_count(&self, anim_name: &str) -> usize {
        self.animations.get(anim_name).map(|f| f.len()).unwrap_or(0)
    }
    
    pub fn has_animation(&self, anim_name: &str) -> bool {
        self.animations.contains_key(anim_name)
    }
}
