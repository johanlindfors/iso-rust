use tetra::graphics::{self, Color, DrawParams, Texture, Rectangle, Camera};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State, Event};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct MapData {
    image: String,
    tiles: HashMap<i32, TileData>,
    width: usize,
    height: usize,
    low: [[i32; 10]; 10],
    mid: [[i32; 10]; 10],
    high: [[i32; 10]; 10],
}

#[derive(Serialize, Deserialize)]
struct Clip {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Clip {
    fn as_rectangle(&self) -> Rectangle {
        Rectangle {
            x: self.x as f32 * ISO_WIDTH,
            y: self.y as f32 * ISO_HEIGHT, 
            width: self.width as f32, 
            height: self.height as f32
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
struct TileData {
    clip: Clip,
    origin: Point,
}

struct Map {    
    tiles: HashMap<i32, Tile>,
    low: [[i32; 10]; 10],
    mid: [[i32; 10]; 10],
    high: [[i32; 10]; 10],
}

impl Map {
    fn from_json(ctx: &mut Context, filename: &str) -> Self {
        let map_json = read_file(filename);
        let map_data: MapData = serde_json::from_str(&map_json).unwrap();
        let texture = Texture::new(ctx, map_data.image).unwrap();
        
        let mut tiles = HashMap::new();
        for (index, tile) in map_data.tiles {
            tiles.insert(index, Tile {
                texture: texture.clone(),
                clip: tile.clip.as_rectangle(),
                origin: Vec2::new(tile.origin.x as f32, tile.origin.y as f32),
            });
        }
        Self {
            low: map_data.low,
            mid: map_data.mid,
            high: map_data.high,
            tiles,
        }
    }
}

pub fn read_file(filepath: &str) -> String {
    let mut file = File::open(filepath)
        .expect("could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

const ISO_WIDTH: f32 = 64.0;
const ISO_HEIGHT: f32 = 64.0;

struct Tile {
    texture: Texture,
    clip: Rectangle,
    origin: Vec2<f32>,
}

impl Tile {
    fn draw(&self, ctx: &mut Context, x: i32, y: i32) {
        let position = cartesian_to_isometric(Vec2::new(x,y));
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(position)
                .origin(self.origin)
                .clip(self.clip),
        );
    }
}

fn cartesian_to_isometric(cartesian_position: Vec2<i32>) -> Vec2<f32> {
    Vec2::new(
        (cartesian_position.x - cartesian_position.y) as f32,
        (cartesian_position.x + cartesian_position.y) as f32 / 2.0
    )
}

fn isometric_to_cartesian(isometric_position: Vec2<f32>) -> Vec2<i32> {
    Vec2::new(
        (2.0 * isometric_position.y + isometric_position.x) as i32 / 2,
        (2.0 * isometric_position.y - isometric_position.x) as i32 / 2
    )
}

struct GameState {
    camera: Camera,
    map: Map,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut camera = Camera::with_window_size(ctx);
        camera.position.x = 32.0;
        camera.position.y = 48.0;
        camera.set_viewport_size(640.0, 480.0);
        camera.update();

        let map = Map::from_json(ctx, "./resources/lake.json");

        Ok(GameState {
            camera,
            map,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        for row in 0..10 {
            for col in 0..10 {
                let x = (col * 32) as i32;
                let y = (row * 32) as i32;
                let tile_index = self.map.low[row][col];
                if tile_index > 0 {
                    let tile = &self.map.tiles[&tile_index];
                    tile.draw(ctx, x, y);
                }

                let tile_index = self.map.mid[row][col];
                if tile_index > 0 {
                    let tile = &self.map.tiles[&tile_index];
                    tile.draw(ctx, x, y);
                }

                let tile_index = self.map.high[row][col];
                if tile_index > 0 {
                    let tile = &self.map.tiles[&tile_index];
                    tile.draw(ctx, x, y);
                }
            }
        }

        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.camera.set_viewport_size(width as f32, height as f32);
            self.camera.update();
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Rendering a Texture", 640, 480)
        .resizable(true)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}