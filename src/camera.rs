use macroquad::prelude::*;

pub struct GameCamera {
    camera: Camera2D,
    position: Vec2,
    last_mouse_pos: Option<(f32, f32)>,
    bounds: (f32, f32, f32, f32),
}

impl GameCamera {
    pub fn new(bounds: (f32, f32, f32, f32)) -> Self {
        Self {
            camera: Camera2D {
                target: vec2(0.0, 0.0),
                zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
                rotation: 0.0,
                ..Default::default()
            },
            position: vec2(0.0, 0.0),
            last_mouse_pos: None,
            bounds,
        }
    }

    pub fn update(&mut self) {
        if is_mouse_button_down(MouseButton::Right) {
            let (mouse_dx, mouse_dy) = mouse_position();

            if let Some((last_x, last_y)) = self.last_mouse_pos {
                let delta_x = mouse_dx - last_x;
                let delta_y = mouse_dy - last_y;
                self.position.x -= delta_x;
                self.position.y -= delta_y;

                // Clamp position to bounds
                self.position.x = self.position.x.clamp(self.bounds.0, self.bounds.1);
                self.position.y = self.position.y.clamp(self.bounds.2, self.bounds.3);
            }
            self.last_mouse_pos = Some((mouse_dx, mouse_dy));
        } else if is_mouse_button_down(MouseButton::Middle) {
            let (mouse_dx, mouse_dy) = mouse_position();

            if let Some((last_x, _last_y)) = self.last_mouse_pos {
                let delta_x = mouse_dx - last_x;
                self.camera.rotation += delta_x * 0.1; // Adjust rotation speed as needed
            }
            self.last_mouse_pos = Some((mouse_dx, mouse_dy));
        } else {
            let scroll = mouse_wheel().1;
            if scroll != 0.0 {
                self.camera.zoom *= 1.0 + scroll * 0.1; // Adjust zoom speed as needed
            } else {
                self.last_mouse_pos = None;
            }
        }

        self.camera.target = self.position;
        set_camera(&self.camera);

        // Draw the camera rotation text
        self.draw_rotation_text();
    }

    pub fn draw_rotation_text(&self) {
        let rotation_degrees = self.camera.rotation.to_degrees() % 360.0;
        draw_text(
            &format!("Rotation: {}°", rotation_degrees),
            10.0,
            50.0,
            30.0,
            WHITE,
        );
    }
}
