use crate::Context;

pub enum UpdateStatus {
	Ok,
	ChangeState(Box<dyn State>),
}

pub trait State {
	fn init(&mut self, context: &mut Context);
	fn update(&mut self, context: &mut Context) -> UpdateStatus;
	fn render(&self, context: &Context);
}