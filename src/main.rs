#![allow(dead_code)]

#[derive(Clone)]
pub struct TileRef {
	pub tile:Tile,
	pub x:i32,
	pub y:i32,
}

#[derive(Clone)]
pub enum Tile {
	Wall(),
	Body(),
}

impl Tile {
	pub fn info(&self) -> TileInfo {
		match *self {
			Tile::Wall() => {
				TileInfo {
					is_body_like: false,
					is_movable: false,
				}
			}
			Tile::Body() => {
				TileInfo {
					is_body_like: true,
					is_movable: true,
				}
			}
		}
	}
}

#[derive(Clone)]
pub struct TileInfo {
	pub is_body_like:bool,
	pub is_movable:bool,
}

pub struct World {
	pub tiles:Vec<TileRef>,
	pub creatures:Vec<Creature>,
}
impl World {
	pub fn new() -> World {
		World { tiles:Vec::new(), creatures:Vec::new() }
	}
	
	pub fn add_tile(&mut self, tile:Tile, x:i32, y:i32) {
		self.tiles.push(TileRef { tile, x, y });
	}
	
	pub fn add_creature(&mut self, creature:Creature) {
		self.creatures.push(creature);
	}
}

pub struct Creature {
	tiles:Vec<TileRef>,
	brain:Brain,
}
impl Creature {
	pub fn new() -> Creature {
		Creature {
			tiles:Vec::new(),
			brain:Brain {},
		}
	}
}
pub struct Brain {
	
}


pub fn main() {
	println!("Evolution sim!");
	
	let mut world = World::new();
	
	let creature1 = Creature::new();
	
	world.add_creature(creature1);
}
