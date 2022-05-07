/* TODO
Health bar for The Inevitable
Projectile trails?
*/

mod circles;
mod context;
mod game_state;
mod intro_state;
mod particle;
mod player;
mod projectile;
mod resources;
mod splash_state;
mod stars;
mod state;
mod the_inevitable;
mod win_state;

use crate::splash_state::SplashState;
use crate::state::*;
use crate::game_state::GameState;
use crate::resources::Resources;
use crate::context::Context;
use viewport::*;
use macroquad::prelude::*;

pub const SCREEN_WIDTH: f32 = 240.0;
pub const SCREEN_HEIGHT: f32 = 320.0;
pub const SCREEN_SCALE: i32 = 2;

fn window_conf() -> Conf {
	Conf {
		window_title: "Delay the Inevitable - eboatwright - Ludum Dare 50".to_string(),
		window_width: SCREEN_WIDTH as i32 * SCREEN_SCALE,
		window_height: SCREEN_HEIGHT as i32 * SCREEN_SCALE,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	let mut current_state: Box<dyn State> = Box::new(SplashState::new());
	let mut context = Context {
		resources: Resources::new(),
		viewport: Viewport::new(SCREEN_WIDTH, SCREEN_HEIGHT),
	};
	context.resources.load().await;
	context.viewport.camera.target = vec2(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5);

	current_state.init(&mut context);

	loop {
		let status = current_state.update(&mut context);
		match status {
			UpdateStatus::Ok => {}
			UpdateStatus::ChangeState(state) => {
				current_state = state;
				current_state.init(&mut context);
			}
		}

		clear_background(BLACK);

		current_state.render(&context);

		context.viewport.render();

		next_frame().await
	}
}

pub fn delta_time() -> f32 {
	get_frame_time() * 60.0
}