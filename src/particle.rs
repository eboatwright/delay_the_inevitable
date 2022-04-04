use macroquad::prelude::*;
use crate::delta_time;
use crate::GameState;

pub struct Particle {
	pub position: Vec2,
	pub velocity: Vec2,
	pub gravity: f32,
	pub life: f32,
	pub texture: Texture2D,
}

impl Particle {
	pub fn new(position: Vec2, velocity: Vec2, gravity: f32, life: f32, texture: Texture2D) -> Self {
		Self {
			position,
			velocity,
			gravity,
			life,
			texture,
		}
	}
}

pub fn particles_update(state: &mut GameState) {
	let mut to_destroy = vec![];
	for (i, particle) in state.particles.iter_mut().enumerate() {
		particle.life -= delta_time();
		if particle.life <= 0.0 {
			to_destroy.push(i);
		}

		particle.velocity.y += particle.gravity;
		particle.position += particle.velocity * delta_time();
	}
	for i in to_destroy.iter().rev() {
		state.particles.remove(*i);
	}
}

pub fn particles_render(state: &GameState) {
	for particle in state.particles.iter() {
		draw_texture(particle.texture, particle.position.x.round(), particle.position.y.round(), WHITE);
	}
}