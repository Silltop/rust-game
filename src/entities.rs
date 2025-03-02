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
}

pub struct Player {
    x: f32,
    y: f32,
    target_x: Option<f32>,
    target_y: Option<f32>,
    rotation: f32,
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
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            target_x: None,
            target_y: None,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.set_x(self.x() + 2.0);
            self.set_target_x(None);
            self.set_target_y(None);
        }
        if is_key_down(KeyCode::Left) {
            self.set_x(self.x() - 2.0);
            self.set_target_x(None);
            self.set_target_y(None);
        }
        if is_key_down(KeyCode::Up) {
            self.set_y(self.y() - 2.0);
            self.set_target_x(None);
            self.set_target_y(None);
        }
        if is_key_down(KeyCode::Down) {
            self.set_y(self.y() + 2.0);
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
            self.rotation = dy.atan2(dx);
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
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
    }
}
