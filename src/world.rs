use macroquad::prelude::*;

pub struct WorldPlane {
    position: Vec2,
    zoom: Vec2,
    rotation: f32,
    last_mouse_pos: Option<(f32, f32)>,
}

impl WorldPlane {
    pub fn new() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            zoom: vec2(1.0, 1.0),
            rotation: 0.0,
            last_mouse_pos: None,
        }
    }
    pub fn get_matrix(&self) -> Mat3 {
        Mat3::from_translation(self.position)
            * Mat3::from_scale(self.zoom)
            * Mat3::from_angle(self.rotation)
    }
    pub fn get_mouse_pos(&self) -> (f32, f32) {
        let camera_matrix = self.get_matrix();
        let mouse_pos = mouse_position();
        let world_pos = camera_matrix
            .inverse()
            .transform_point2(vec2(mouse_pos.0, mouse_pos.1));
        (world_pos.x, world_pos.y)
    }

    pub fn update(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if let Some((last_x, last_y)) = self.last_mouse_pos {
                let delta_x = mouse_pos.0 - last_x;
                let delta_y = mouse_pos.1 - last_y;
                self.position.x += delta_x / self.zoom.x;
                self.position.y += delta_y / self.zoom.y;
            }
            self.last_mouse_pos = Some(mouse_pos);
        } else {
            self.last_mouse_pos = None;
        }

        let scroll = mouse_wheel().1;
        if scroll != 0.0 {
            let zoom_factor = 1.1_f32.powf(scroll);
            self.zoom *= zoom_factor;
        }
    }

    // pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> (f32, f32) {
    //     let camera_matrix = self.get_matrix();
    //     let screen_pos = camera_matrix.transform_point2(vec2(world_x, world_y));
    //     (screen_pos.x, screen_pos.y)
    // }

    // pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
    //     let camera_matrix = self.get_matrix();
    //     let inv_camera_matrix = camera_matrix.inverse();
    //     let world_pos = inv_camera_matrix.transform_point2(vec2(screen_x, screen_y));
    //     (world_pos.x, world_pos.y)
    // }
}
