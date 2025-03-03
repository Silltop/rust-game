use macroquad::prelude::*;

trait Entity {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn target_x(&self) -> Option<f32>;
    fn target_y(&self) -> Option<f32>;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_target_x(&mut self, target_x: Option<f32>);
    fn set_target_y(&mut self, target_y: Option<f32>);
    fn _is_hovered(
        &self,
        mouse_x: f32,
        mouse_y: f32,
        texture_width: f32,
        texture_height: f32,
    ) -> bool;
}

pub struct Player {
    x: f32,
    y: f32,
    target_x: Option<f32>,
    target_y: Option<f32>,
    rotation: f32,
    texture_width: f32,
    texture_height: f32,
}

impl Entity for Player {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn target_x(&self) -> Option<f32> {
        self.target_x
    }

    fn target_y(&self) -> Option<f32> {
        self.target_y
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn set_target_x(&mut self, target_x: Option<f32>) {
        self.target_x = target_x;
    }

    fn set_target_y(&mut self, target_y: Option<f32>) {
        self.target_y = target_y;
    }

    fn _is_hovered(
        &self,
        mouse_x: f32,
        mouse_y: f32,
        texture_width: f32,
        texture_height: f32,
    ) -> bool {
        mouse_x >= self.x() - texture_width / 2.0
            && mouse_x <= self.x() + texture_width / 2.0
            && mouse_y >= self.y() - texture_height / 2.0
            && mouse_y <= self.y() + texture_height / 2.0
    }
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            target_x: None,
            target_y: None,
            rotation: 0.0,
            texture_width: 0.0,
            texture_height: 0.0,
        }
    }

    pub fn update(&mut self) {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        if is_key_down(KeyCode::Right) {
            dx += 2.0;
        }
        if is_key_down(KeyCode::Left) {
            dx -= 2.0;
        }
        if is_key_down(KeyCode::Up) {
            dy -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            dy += 2.0;
        }

        if dx != 0.0 || dy != 0.0 {
            let distance = (dx * dx + dy * dy).sqrt();
            self.set_x(self.x() + dx / distance * 2.0);
            self.set_y(self.y() + dy / distance * 2.0);
            self.rotation =
                (dy.atan2(dx) + std::f32::consts::PI * 2.0) % (std::f32::consts::PI * 2.0);
            self.set_target_x(None);
            self.set_target_y(None);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.set_target_x(Some(mouse_x));
            self.set_target_y(Some(mouse_y));
        }
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            self.set_target_x(Some(mouse_x));
            self.set_target_y(Some(mouse_y));
        }

        if let (Some(target_x), Some(target_y)) = (self.target_x(), self.target_y()) {
            let dx = target_x - self.x();
            let dy = target_y - self.y();
            let distance = (dx * dx + dy * dy).sqrt();
            let step = 2.0;
            if distance > step {
                self.set_x(self.x() + dx / distance * step);
                self.set_y(self.y() + dy / distance * step);
            } else {
                self.set_x(target_x);
                self.set_y(target_y);
                self.set_target_x(None);
                self.set_target_y(None);
            }
            self.rotation =
                (dy.atan2(dx) + std::f32::consts::PI * 2.0) % (std::f32::consts::PI * 2.0);
        }
    }

    pub fn is_hovered(&self, mouse_x: f32, mouse_y: f32) -> bool {
        return self._is_hovered(mouse_x, mouse_y, self.texture_width, self.texture_height);
    }

    pub fn draw(&mut self, texture: &Texture2D) {
        self.texture_width = texture.width();
        self.texture_height = texture.height();
        draw_texture_ex(
            texture,
            self.x(),
            self.y(),
            WHITE,
            DrawTextureParams {
                rotation: self.rotation,
                ..Default::default()
            },
        );
        draw_text(
            &format!("CURRENT POS {} {}", self.x(), self.y()),
            self.x,
            self.y,
            30.0,
            WHITE,
        );

        if let (Some(target_x), Some(target_y)) = (self.target_x(), self.target_y()) {
            draw_text(
                &format!("Going to: ({:.1}, {:.1})", target_x, target_y),
                self.x(),
                self.y() - 10.0,
                20.0,
                WHITE,
            );
        }
    }
}
