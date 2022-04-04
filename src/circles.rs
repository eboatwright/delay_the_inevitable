use crate::GameState;
use macroquad::prelude::*;

pub struct Circle {
	pub position: Vec2,
	pub radius: f32,
}

impl Circle {
	pub fn new(position: Vec2, radius: f32) -> Self {
		Self {
			position,
			radius,
		}
	}
}

pub fn circles_update(state: &mut GameState) {
	let mut to_destroy = vec![];
	for (i, circle) in state.circles.iter_mut().enumerate() {
		circle.radius -= 2.0;
		if circle.radius <= 0.0 {
			to_destroy.push(i);
		}
	}
	for i in to_destroy.iter().rev() {
		state.circles.remove(*i);
	}
}

pub fn circles_render(state: &GameState) {
	for circle in state.circles.iter() {
		draw_circle(circle.position.x, circle.position.y, circle.radius, WHITE);
	}
}