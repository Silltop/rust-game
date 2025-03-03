use ::rand::{Rng, thread_rng};
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub struct Particle {
    position: Vec2,
    velocity: Vec2,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self { position, velocity }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 2.0, GRAY);
    }
}

pub struct Weather {
    wind_speed: f32,
    last_change: Instant,
    change_interval: Duration,
    particles: Vec<Particle>,
}

impl Weather {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut particles = Vec::new();
        for _ in 0..100 {
            particles.push(Particle::new(
                vec2(
                    rng.gen_range(0.0, screen_width()),
                    rng.gen_range(0.0, screen_height()),
                ),
                vec2(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0)),
            ));
        }
        Self {
            wind_speed: rng.gen_range(-10.0, 10.0),
            last_change: Instant::now(),
            change_interval: Duration::from_secs(rng.gen_range(1, 5)),
            particles,
        }
    }

    pub fn update(&mut self) {
        if self.last_change.elapsed() >= self.change_interval {
            let mut rng = thread_rng();
            self.wind_speed = rng.gen_range(-10.0, 10.0);
            self.last_change = Instant::now();
            self.change_interval = Duration::from_secs(rng.gen_range(1, 5));
        }

        for particle in &mut self.particles {
            particle.velocity.x += self.wind_speed * 0.01;
            particle.update();
            if particle.position.x < 0.0 {
                particle.position.x = screen_width();
            } else if particle.position.x > screen_width() {
                particle.position.x = 0.0;
            }
            if particle.position.y < 0.0 {
                particle.position.y = screen_height();
            } else if particle.position.y > screen_height() {
                particle.position.y = 0.0;
            }
        }
    }

    pub fn draw(&self) {
        draw_text(
            &format!("Wind Speed: {:.2} KT", self.wind_speed),
            10.0,
            20.0,
            30.0,
            WHITE,
        );

        for particle in &self.particles {
            particle.draw();
        }
    }
}
