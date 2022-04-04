use crate::particle::Particle;
use macroquad::rand::gen_range;
use crate::UpdateStatus;
use macroquad::audio::*;
use crate::projectile::*;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::delta_time;
use crate::GameState;
use crate::Context;
use macroquad::prelude::*;
use crate::circles::Circle;

const PLAYER_MOVE_SPEED: f32 = 0.96;
const PLAYER_FRICTION: f32 = 0.76;
const PLAYER_SHOOT_DELAY: f32 = 8.0;
const PLAYER_ANIMATION_TIME: f32 = 3.0;

pub struct Player {
	pub position: Vec2,
	pub velocity: Vec2,

	pub hp: i16,

	pub shoot_timer: f32,
	pub delay: f32,
	pub played_lose_sound: bool,

	pub animation_timer: f32,
	pub animation_src: Vec2,
}

impl Player {
	pub fn new() -> Self {
		Self {
			position: vec2(SCREEN_WIDTH * 0.5 - 12.0, SCREEN_HEIGHT * 0.8),
			velocity: Vec2::ZERO,

			hp: 3,

			shoot_timer: 0.0,
			delay: 40.0,
			played_lose_sound: false,

			animation_timer: 0.0,
			animation_src: Vec2::ZERO,
		}
	}
}

pub fn player_update(state: &mut GameState, context: &mut Context) -> UpdateStatus {
	if state.player.hp <= 0 {
		if !state.player.played_lose_sound {
			state.player.played_lose_sound = true;
			play_sound(context.resources.player_lose_sfx.unwrap(), PlaySoundParams {
				volume: 0.3,
				looped: false,
			});

			for _ in 0..5 {
				state.particles.push(Particle::new(state.player.position + vec2(10.0, 10.0), vec2(gen_range(-15.0, 15.0), gen_range(-15.0, 15.0)), -0.1, 10.0, context.resources.particle_tex));
			}
		}

		state.player.delay -= delta_time();
		if state.player.delay <= 0.0 {
			return UpdateStatus::ChangeState(Box::new(GameState::new()));
		}
		return UpdateStatus::Ok;
	}

	if is_key_down(KeyCode::Z)
	|| is_key_down(KeyCode::Space) {
		state.player.shoot_timer -= delta_time();
		if state.player.shoot_timer <= 0.0 {
			state.player.shoot_timer = PLAYER_SHOOT_DELAY;
			state.projectiles.push(Projectile::new(state.player.position + vec2(9.0, -3.0), ProjectileShooter::Player, ProjectileMovementType::StraightUp));
			play_sound(context.resources.player_shoot_sfx.unwrap(), PlaySoundParams { volume: 0.2, looped: false });
			for _ in 0..3 {
				state.particles.push(Particle::new(state.player.position + vec2(10.0, 0.0), vec2(gen_range(-15.0, 15.0), gen_range(-2.0, 2.0)), -0.1, 5.0, context.resources.particle_tex));
			}
			state.circles.push(Circle::new(state.player.position + vec2(12.0, 0.0), 16.0));
		}
	} else {
		state.player.shoot_timer = 0.0;
	}

	if is_key_down(KeyCode::Up)
	|| is_key_down(KeyCode::W) {
		state.player.velocity.y -= PLAYER_MOVE_SPEED * delta_time();
	}
	if is_key_down(KeyCode::Down)
	|| is_key_down(KeyCode::S) {
		state.player.velocity.y += PLAYER_MOVE_SPEED * delta_time();
	}
	if is_key_down(KeyCode::Left)
	|| is_key_down(KeyCode::A) {
		state.player.velocity.x -= PLAYER_MOVE_SPEED * delta_time();
	}
	if is_key_down(KeyCode::Right)
	|| is_key_down(KeyCode::D) {
		state.player.velocity.x += PLAYER_MOVE_SPEED * delta_time();
	}

	state.player.velocity *= PLAYER_FRICTION;
	state.player.position += state.player.velocity;

	if state.player.velocity.x.round() < 0.0 {
		state.player.animation_src.y = 1.0;
	} else if state.player.velocity.x.round() > 0.0 {
		state.player.animation_src.y = 2.0;
	} else {
		state.player.animation_src.y = 0.0;
	}

	state.player.animation_timer -= delta_time();
	if state.player.animation_timer <= 0.0 {
		state.player.animation_timer = PLAYER_ANIMATION_TIME;
		state.player.animation_src.x += 1.0;
		if state.player.animation_src.x > 3.0 {
			state.player.animation_src.x = 0.0;
		}
	}

	if state.player.position.x < -24.0 {
		state.player.position.x = SCREEN_WIDTH;
	}

	if state.player.position.x > SCREEN_WIDTH {
		state.player.position.x = -24.0;
	}

	if state.the_inevitable.hp > 0 {
		if state.player.position.y < 75.0 {
			state.player.position.y = 75.0;
		}
	} else if state.player.position.y < -10.0 {
		state.player.position.y = -10.0;
	}

	if state.player.position.y > SCREEN_HEIGHT - 20.0 {
		state.player.position.y = SCREEN_HEIGHT - 20.0;
	}

	UpdateStatus::Ok
}

pub fn player_render(state: &GameState, context: &Context) {
	if state.player.hp <= 0 {
		return;
	}
	draw_texture_ex(context.resources.player_tex, state.player.position.x.round(), state.player.position.y.round(), WHITE, DrawTextureParams {
		dest_size: Some(vec2(24.0, 36.0)),
		source: Some(Rect {
			x: state.player.animation_src.x * 24.0,
			y: state.player.animation_src.y * 36.0,
			w: 24.0,
			h: 36.0,
		}),
		..Default::default()
	});
}