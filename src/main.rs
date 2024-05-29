use macroquad::prelude::*;

mod bouncer;
mod ourguy;

use bouncer::Bouncer;
use ourguy::OurGuy;

const SCREEN_WIDTH: i32 = 720;
const SCREEN_HEIGHT: i32 = 720;

struct MapEntry {
    secs_from_last: i32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Midi Sim".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut guy: OurGuy = OurGuy::new(200f32, 200f32);

    let mut bouncers: Vec<Bouncer> = vec![];
    let map = vec![
        MapEntry { secs_from_last: 5 },
        MapEntry { secs_from_last: 5 },
        MapEntry { secs_from_last: 5 },
        MapEntry { secs_from_last: 5 },
    ];

    let mut nextvelo: Vec2 = guy.get_velo();
    let mut lastvelo: Vec2 = vec2(0., 0.);
    let mut lastpos: Vec2 = guy.get_pos();

    for i in 0..map.len() {
        bouncers.push(Bouncer::new(
            lastpos,
            nextvelo,
            lastvelo,
            map[i].secs_from_last,
        ));
        lastvelo = nextvelo;
        nextvelo = bouncers[i].get_next_velo();
        lastpos = bouncers[i].get_pos();

        if i == map.len() - 1 {
            bouncers[i].set_end(true);
        }
    }

    loop {
        let delta = get_frame_time();
//
        // UPDATE

        {
            let bounce = &bouncers[guy.get_checking() as usize];
            guy.get_collision(bounce.get_pos(), bounce.get_type(), bounce.get_end());
        }
        
        guy.update(delta);

        clear_background(LIGHTGRAY);

        set_camera(&Camera2D {
            target: guy.get_pos(),
            zoom: Vec2 {
                x: 0.5 / SCREEN_WIDTH as f32 * 2.,
                y: 0.5 / SCREEN_HEIGHT as f32 * 2.,
            },
            ..Default::default()
        });

        // DRAW

        guy.draw();
        for bounce in bouncers.iter() {
            bounce.draw();
        }

        next_frame().await;
    }
}
