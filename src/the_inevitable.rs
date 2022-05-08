use crate::SCREEN_WIDTH;
use crate::projectile::*;
use crate::win_state::WinState;
use crate::UpdateStatus;
use macroquad::audio::PlaySoundParams;
use macroquad::audio::play_sound;
use crate::particle::Particle;
use macroquad::rand::gen_range;
use crate::delta_time;
use crate::GameState;
use crate::Context;
use macroquad::prelude::*;

const EXPLOSION_DELAY: f32 = 8.0;
const ATTACK_TIME: f32 = 120.0;
const MAX_HEALTH: i16 = 240;

pub struct TheInevitable {
	pub position: Vec2,
	pub hp: i16,
	pub explosion_timer: f32,
	pub win_delay: f32,
	pub attack_timer: f32,
}

impl TheInevitable {
	pub fn new() -> Self {
		Self {
			position: vec2(-10.0, -10.0),
			hp: MAX_HEALTH,
			explosion_timer: 0.0,
			win_delay: 120.0,
			attack_timer: ATTACK_TIME,
		}
	}
}

pub fn the_inevitable_update(state: &mut GameState, context: &mut Context) -> UpdateStatus {
	if state.the_inevitable.hp <= 0 {
		state.the_inevitable.explosion_timer -= delta_time();
		if state.the_inevitable.explosion_timer <= 0.0 {
			state.the_inevitable.explosion_timer = EXPLOSION_DELAY;
			for _ in 0..3 {
				state.particles.push(Particle::new(state.the_inevitable.position + vec2(gen_range(0.0, 214.0), gen_range(0.0, 96.0)), vec2(gen_range(-6.0, 6.0), gen_range(-6.0, 6.0)), -0.6, gen_range(30.0, 60.0), context.resources.smoke_tex));
			}
			state.screen_shake += vec2(gen_range(-6.0, 6.0), gen_range(-6.0, 6.0));
			play_sound(context.resources.the_inevitable_explosion_sfx.unwrap(), PlaySoundParams {
				volume: 0.8,
				looped: false,
			});
		}

		state.the_inevitable.win_delay -= delta_time();
		if state.the_inevitable.win_delay <= 0.0 {
			return UpdateStatus::ChangeState(Box::new(WinState::new()));
		}
		return UpdateStatus::Ok;
	}

	state.the_inevitable.attack_timer -= delta_time();
	if state.the_inevitable.attack_timer <= 0.0 {
		state.the_inevitable.attack_timer = ATTACK_TIME;
		match gen_range(0u8, 3) {
			0 => {
				let x_offset = gen_range(-130.0, -90.0);
				for i in 0..8 {
					state.projectiles.push(Projectile::new(vec2(x_offset, -200.0) + vec2(i as f32 * 45.0, i as f32 * 38.0), ProjectileShooter::Enemy));
				}
			}
			1 => {
				let x_offset = gen_range(200.0, 240.0);
				for i in 0..8 {
					state.projectiles.push(Projectile::new(vec2(x_offset, -200.0) + vec2(i as f32 * -45.0, i as f32 * 38.0), ProjectileShooter::Enemy));
				}
			}
			2 => {
				let x_offset = gen_range(-140.0, -100.0);
				for i in 0..3 {
					for j in 0..8 {
						state.projectiles.push(Projectile::new(vec2(x_offset - (i % 2) as f32 * 25.0, -60.0) + vec2(j as f32 * 55.0, i as f32 * -160.0), ProjectileShooter::Enemy));
					}
				}
			}
			_ => {}
		}
	}

	UpdateStatus::Ok
}

pub fn the_inevitable_render(state: &GameState, context: &Context) {
	draw_texture(context.resources.the_inevitable_tex, state.the_inevitable.position.x + (f32::cos(get_time() as f32) * 10.0).round(), state.the_inevitable.position.y + (f32::sin(get_time() as f32 * 10.0) * 5.0).round(), WHITE);

	draw_rectangle(
		16.0,
		8.0,
		SCREEN_WIDTH - 32.0,
		16.0,
		Color {
			r: 0.133,
			g: 0.125,
			b: 0.203,
			a: 1.0,
		},
	);

	draw_rectangle(
		17.0,
		9.0,
		(SCREEN_WIDTH - 30.0) * (state.the_inevitable.hp as f32 / MAX_HEALTH as f32),
		14.0,
		Color {
			r: 0.674,
			g: 0.196,
			b: 0.196,
			a: 1.0,
		},
	);
}