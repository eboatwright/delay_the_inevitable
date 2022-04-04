use crate::delta_time;
use crate::SCREEN_WIDTH;
use macroquad::rand::gen_range;
use crate::GameState;
use crate::Context;
use macroquad::prelude::*;
use crate::SCREEN_HEIGHT;

pub struct Star {
	pub position: Vec3,
}

impl Star {
	pub fn new(position: Vec3) -> Self {
		Self {
			position,
		}
	}
}

pub fn stars_init(state: &mut GameState) {
	state.stars = vec![];
	for _ in 0..70 {
		state.stars.push(Star::new(vec3(gen_range(1.0, SCREEN_WIDTH - 8.0), gen_range(1.0, SCREEN_HEIGHT - 8.0), gen_range(0, 2) as f32)));
	}
}

pub fn stars_update(state: &mut GameState) {
	for star in state.stars.iter_mut() {
		star.position.y += (2.0 - star.position.z) * 2.0 * delta_time();
		if star.position.y >= SCREEN_HEIGHT {
			star.position.y = -7.0;
		}
	}
}

pub fn stars_render(state: &GameState, context: &Context) {
	for star in state.stars.iter() {
		draw_texture_ex(context.resources.star_tex, star.position.x.round(), star.position.y.round(), WHITE, DrawTextureParams {
			dest_size: Some(vec2(7.0, 7.0)),
			source: Some(Rect {
				x: star.position.z * 7.0,
				y: 0.0,
				w: 7.0,
				h: 7.0,
			}),
			..Default::default()
		});
	}
}