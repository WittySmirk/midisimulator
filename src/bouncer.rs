use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum BounceType {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

pub struct Bouncer {
    position: Vec2,
    next_velo: Vec2,
    end: bool,
    t: BounceType,
}

impl Bouncer {
    pub fn new(player_pos: Vec2, player_velo: Vec2, last_velo: Vec2, secs: i32) -> Self {
        let delta_x = player_velo.x * secs as f32;
        let delta_y = player_velo.y * secs as f32;

        let mut new_pos: Vec2 = vec2(player_pos.x + delta_x, player_pos.y - delta_y);
        let mut t: BounceType = BounceType::RIGHT;
        let mut next_velo: Vec2 = vec2(0., 0.);

        if last_velo.x == 0. && last_velo.y == 0. || last_velo.x > 0. && last_velo.y < 0. {
            // Bounce off the right
            t = BounceType::RIGHT;
            new_pos.x = player_pos.x + delta_x;
            next_velo = vec2(-100.0f32, 100.0f32);
        } else if last_velo.x > 0. && last_velo.y > 0. {
            // Bouncing off top
            t = BounceType::TOP;
            next_velo = vec2(-100.0f32, -100.0f32);
        } else if last_velo.x < 0. && last_velo.y > 0. {
            // Bouncing off left
            t = BounceType::LEFT;
            new_pos.x = player_pos.x + delta_x - 20.;
            next_velo = vec2(100.0f32, -100.0f32);
        } else if last_velo.x < 0. && last_velo.y < 0. {
            // Bouncing off bottom
            t = BounceType::BOTTOM;
            new_pos.y = player_pos.x - delta_y + 20.;
            next_velo = vec2(100.0f32, 100.0f32);
        }

        Self {
            position: new_pos,
            next_velo,
            end: false,
            t,
        }
    }

    pub fn draw(&self) {
        let mut adjustedpos = self.position;
        if self.t == BounceType::RIGHT {
            adjustedpos = vec2(self.position.x + 20., self.position.y);
        }
        if self.t == BounceType::TOP {
            adjustedpos = vec2(self.position.x, self.position.y - 20.);
        }
        if self.t == BounceType::LEFT {
            adjustedpos = vec2(self.position.x - 20., self.position.y);
        }
        if self.t == BounceType::BOTTOM {
            adjustedpos = vec2(self.position.x, self.position.y + 20.);
        }
        draw_rectangle(adjustedpos.x, adjustedpos.y, 20., 20., BLUE);
    }

    pub fn get_next_velo(&self) -> Vec2 {
        return self.next_velo;
    }
    pub fn get_pos(&self) -> Vec2 {
        return self.position;
    }
    pub fn get_end(&self) -> bool {
        return self.end;
    }
    pub fn set_end(&mut self, deez: bool) {
        self.end = deez;
    }
    pub fn get_type(&self) -> &BounceType {
        return &self.t;
    }
}
