use crate::SCREEN_HEIGHT;
use crate::particle::Particle;
use macroquad::rand::gen_range;
use macroquad::audio::*;
use crate::delta_time;
use macroquad::prelude::*;
use crate::Context;
use crate::GameState;
use crate::circles::Circle;

pub enum ProjectileShooter {
	Player,
	Enemy,
}

pub struct Projectile {
	pub position: Vec2,
	pub shooter: ProjectileShooter,
}

impl Projectile {
	pub fn new(position: Vec2, shooter: ProjectileShooter) -> Self {
		Self {
			position,
			shooter,
		}
	}
}

pub fn projectiles_update(state: &mut GameState, context: &mut Context) {
	let mut to_destroy = vec![];
	let mut hit_player = false;
	for (i, projectile) in state.projectiles.iter_mut().enumerate() {
		let projectile_rect = Rect {
			x: projectile.position.x,
			y: projectile.position.y,
			w: 6.0,
			h: 6.0,
		};

		match projectile.shooter {
			ProjectileShooter::Player => {
				projectile.position.y -= 9.0 * delta_time();
				if projectile.position.y < -5.0 {
					to_destroy.push(i);
					continue;
				}

				if state.the_inevitable.hp > 0
				&& projectile_rect.overlaps(&Rect {
					x: state.the_inevitable.position.x,
					y: state.the_inevitable.position.y,
					w: 260.0,
					h: 84.0,
				}) {
					state.the_inevitable.hp -= 1;
					to_destroy.push(i);
					state.screen_shake += vec2(gen_range(-2.5, 2.5), gen_range(-2.5, 2.5));
					for _ in 0..3 {
						state.particles.push(Particle::new(projectile.position + vec2(4.0, 4.0), vec2(gen_range(-15.0, 15.0), gen_range(-2.0, 2.0)), -0.1, 5.0, context.resources.particle_tex));
					}
					state.circles.push(Circle::new(projectile.position + vec2(4.0, 4.0), 16.0));
				}
			}
			ProjectileShooter::Enemy => {
				projectile.position.y += 4.2 * delta_time();
				if projectile.position.y > SCREEN_HEIGHT + 4.0 {
					to_destroy.push(i);
					continue;
				}

				if state.player.hp <= 0
				|| state.the_inevitable.hp <= 0 {
					continue;
				}
				if projectile_rect.overlaps(&Rect {
					x: state.player.position.x + 3.0,
					y: state.player.position.y + 3.0,
					w: 18.0,
					h: 18.0,
				}) {
					state.player.hp -= 1;
					to_destroy.push(i);
					hit_player = true;
					state.screen_shake += vec2(gen_range(-7.0, 7.0), gen_range(-7.0, 7.0));
					for _ in 0..3 {
						state.particles.push(Particle::new(projectile.position + vec2(4.0, 4.0), vec2(gen_range(-15.0, 15.0), gen_range(-2.0, 2.0)), -0.1, 5.0, context.resources.particle_tex));
					}
					state.circles.push(Circle::new(projectile.position + vec2(4.0, 4.0), 16.0));
				}
			}
		}
	}
	for i in to_destroy.iter().rev() {
		state.projectiles.remove(*i);
	}

	if hit_player {
		play_sound(context.resources.player_hit_sfx.unwrap(), PlaySoundParams { volume: 0.8, looped: false });
	}
}

pub fn projectiles_render(state: &GameState, context: &Context) {
	for projectile in state.projectiles.iter() {
		draw_texture(match projectile.shooter {
			ProjectileShooter::Player => context.resources.player_projectile_tex,
			ProjectileShooter::Enemy => context.resources.enemy_projectile_tex,
		}, projectile.position.x.round(), projectile.position.y.round(), WHITE);
	}
}