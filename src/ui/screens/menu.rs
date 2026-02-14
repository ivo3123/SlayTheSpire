use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use crate::ui::rendering::theme::Theme;

pub struct MenuScreen {
    theme: Theme,
}

impl MenuScreen {
    pub fn new() -> Self {
        MenuScreen {
            theme: Theme::default(),
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<MenuAction> {
        Ok(MenuAction::None)
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

        let mut title = Text::new("SLAY THE SPIRE");
        title.set_scale(64.0);
        let title_pos = [350.0, 150.0];
        canvas.draw(&title, DrawParam::default().dest(title_pos).color(self.theme.text));

        let mut subtitle = Text::new("Rust Edition");
        subtitle.set_scale(32.0);
        let subtitle_pos = [520.0, 230.0];
        canvas.draw(&subtitle, DrawParam::default().dest(subtitle_pos).color(self.theme.text_secondary));

        let start_button_rect = Rect::new(450.0, 350.0, 500.0, 80.0);
        let button_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            start_button_rect,
            self.theme.button,
        )?;
        canvas.draw(&button_mesh, DrawParam::default());

        let button_border = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::stroke(3.0),
            start_button_rect,
            self.theme.card_border,
        )?;
        canvas.draw(&button_border, DrawParam::default());

        let mut button_text = Text::new("START NEW RUN");
        button_text.set_scale(32.0);
        canvas.draw(&button_text, DrawParam::default().dest([570.0, 380.0]).color(self.theme.text));

        let quit_button_rect = Rect::new(450.0, 460.0, 500.0, 80.0);
        let quit_button_mesh = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            quit_button_rect,
            self.theme.button,
        )?;
        canvas.draw(&quit_button_mesh, DrawParam::default());

        let quit_border = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::stroke(3.0),
            quit_button_rect,
            self.theme.card_border,
        )?;
        canvas.draw(&quit_border, DrawParam::default());

        let mut quit_text = Text::new("QUIT");
        quit_text.set_scale(32.0);
        canvas.draw(&quit_text, DrawParam::default().dest([665.0, 490.0]).color(self.theme.text));

        Ok(())
    }

    pub fn mouse_button_down(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<MenuAction> {
        if button == MouseButton::Left {
            let start_button_rect = Rect::new(450.0, 350.0, 500.0, 80.0);
            if x >= start_button_rect.x && x <= start_button_rect.x + start_button_rect.w &&
               y >= start_button_rect.y && y <= start_button_rect.y + start_button_rect.h {
                return Ok(MenuAction::StartRun);
            }
            
            let quit_button_rect = Rect::new(450.0, 460.0, 500.0, 80.0);
            if x >= quit_button_rect.x && x <= quit_button_rect.x + quit_button_rect.w &&
               y >= quit_button_rect.y && y <= quit_button_rect.y + quit_button_rect.h {
                return Ok(MenuAction::Quit);
            }
        }
        Ok(MenuAction::None)
    }
}

pub enum MenuAction {
    None,
    StartRun,
    Quit,
}
