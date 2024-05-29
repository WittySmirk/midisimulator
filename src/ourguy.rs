use macroquad::prelude::*;

use crate::bouncer::BounceType;

pub struct OurGuy {
    position: Vec2,
    velocity: Vec2,
    checking: i32,
}

impl OurGuy {
    pub fn new(x: f32, y: f32) -> Self {
        return Self {
            position: Vec2 { x, y },
            velocity: Vec2 {
                x: 100f32,
                y: 100f32,
            },
            checking: 0,
        };
    }
    pub fn update(&mut self, delta: f32) {
        self.position.x += self.velocity.x * delta;
        self.position.y -= self.velocity.y * delta;
    }
    pub fn get_collision(&mut self, other_pos: Vec2, b_type: &BounceType, end: bool) {
        match b_type {
            &BounceType::BOTTOM => {
                if self.position.y >= other_pos.y {
                    if end {
                        self.velocity = vec2(0., 0.);
                        return;
                    }
                    self.velocity.y = -self.velocity.y;
                    self.checking += 1;
                }
            }
            &BounceType::TOP => {
                if self.position.y <= other_pos.y {
                    if end {
                        self.velocity = vec2(0., 0.);
                        return;
                    }
                    self.velocity.y = -self.velocity.y;
                    self.checking += 1;
                }
            }
            &BounceType::LEFT => {
                if self.position.x <= other_pos.x {
                    if end {
                        self.velocity = vec2(0., 0.);
                        return;
                    }
                    self.velocity.x = -self.velocity.x;
                    self.checking += 1;
                }
            }
            &BounceType::RIGHT => {
                if self.position.x >= other_pos.x {
                    if end {
                        self.velocity = vec2(0., 0.);
                        return;
                    }
                    self.velocity.x = -self.velocity.x;
                    self.checking += 1;
                }
            }
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, 20f32, 20f32, RED);
    }
    pub fn get_pos(&self) -> Vec2 {
        return self.position;
    }
    pub fn get_velo(&self) -> Vec2 {
        return self.velocity;
    }
    pub fn get_checking(&self) -> i32 {
        return self.checking;
    }
}
