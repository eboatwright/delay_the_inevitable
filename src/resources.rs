use macroquad::audio::*;
use macroquad::prelude::*;

pub struct Resources {
	pub star_tex: Texture2D,
	pub player_tex: Texture2D,
	pub the_inevitable_tex: Texture2D,
	pub player_projectile_tex: Texture2D,
	pub enemy_projectile_tex: Texture2D,
	pub smoke_tex: Texture2D,
	pub particle_tex: Texture2D,

	pub font: Font,

	pub player_hit_sfx: Option<Sound>,
	pub player_shoot_sfx: Option<Sound>,
	pub player_lose_sfx: Option<Sound>,
	pub blip_sfx: Option<Sound>,
	pub splash_sfx: Option<Sound>,
	pub the_inevitable_explosion_sfx: Option<Sound>,
	pub music: Option<Sound>,

	pub loaded: bool,
}

impl Resources {
	pub fn new() -> Self {
		Self {
			star_tex: Texture2D::empty(),
			player_tex: Texture2D::empty(),
			the_inevitable_tex: Texture2D::empty(),
			player_projectile_tex: Texture2D::empty(),
			enemy_projectile_tex: Texture2D::empty(),
			smoke_tex: Texture2D::empty(),
			particle_tex: Texture2D::empty(),

			font: Font::default(),

			player_hit_sfx: None,
			player_shoot_sfx: None,
			player_lose_sfx: None,
			blip_sfx: None,
			splash_sfx: None,
			the_inevitable_explosion_sfx: None,
			music: None,

			loaded: false,
		}
	}

	pub async fn load(&mut self) {
		self.star_tex = load_texture("res/img/star.png").await.unwrap();
		self.player_tex = load_texture("res/img/spaceship.png").await.unwrap();
		self.the_inevitable_tex = load_texture("res/img/the_inevitable.png").await.unwrap();
		self.player_projectile_tex = load_texture("res/img/player_projectile.png").await.unwrap();
		self.enemy_projectile_tex = load_texture("res/img/enemy_projectile.png").await.unwrap();
		self.smoke_tex = load_texture("res/img/smoke.png").await.unwrap();
		self.particle_tex = load_texture("res/img/particle.png").await.unwrap();

		self.font = load_ttf_font("res/fnt/bangalore.ttf").await.unwrap();

		self.player_hit_sfx = Some(load_sound("res/sfx/player_hit.wav").await.unwrap());
		self.player_shoot_sfx = Some(load_sound("res/sfx/player_shoot.wav").await.unwrap());
		self.player_lose_sfx = Some(load_sound("res/sfx/player_lose.wav").await.unwrap());
		self.blip_sfx = Some(load_sound("res/sfx/blip.wav").await.unwrap());
		self.splash_sfx = Some(load_sound("res/sfx/splash.wav").await.unwrap());
		self.the_inevitable_explosion_sfx = Some(load_sound("res/sfx/the_inevitable_explosion.wav").await.unwrap());
		self.music = Some(load_sound("res/sfx/music.wav").await.unwrap());

		self.loaded = true;
	}
}