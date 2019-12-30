use tetra::graphics::{self, Color, DrawParams, Texture, Rectangle, Camera};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State, Event};
use std::collections::HashMap;

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
        // position.x *= 64.0;
        // position.y *= 64.0;
        //let position = Vec2::new(x as f32, y as f32);
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
    tiles: HashMap<i32, Tile>,
    map: [[i32; 6]; 6],
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let texture = Texture::new(ctx, "./resources/iso-64x64-outside.png")?;
        let mut tiles = HashMap::new();
        tiles.insert(0, Tile {
            texture: texture.clone(),
            clip: Rectangle::new(0.0, 0.0, 64.0, 64.0),
            origin: Vec2::new(0.0, 0.0),
        });
        tiles.insert(1, Tile {
            texture: texture.clone(),
            clip: Rectangle::new(7.0 * ISO_WIDTH, 3.0 * ISO_HEIGHT, 64.0, 64.0),
            origin: Vec2::new(0.0, 0.0),
        });
        tiles.insert(2, Tile {
            texture: texture.clone(),
            clip: Rectangle::new(0.0, 0.0, 64.0, 64.0),
            origin: Vec2::new(0.0, 0.0),
        });
        tiles.insert(3, Tile {
            texture: texture.clone(),
            clip: Rectangle::new(8.0 * ISO_WIDTH, 3.0 * ISO_HEIGHT, 64.0, 64.0),
            origin: Vec2::new(0.0, 0.0),
        });
        tiles.insert(4, Tile {
            texture: texture.clone(),
            clip: Rectangle::new(0.0, 0.0, 64.0, 64.0),
            origin: Vec2::new(0.0, 0.0),
        });
        tiles.insert(5, Tile {
            texture: texture.clone(),
            clip: Rectangle::new(0.0, 0.0, 64.0, 64.0),
            origin: Vec2::new(0.0, 0.0),
        });
        tiles.insert(6, Tile {
            texture: texture.clone(),
            clip: Rectangle::new(0.0, 0.0, 64.0, 64.0),
            origin: Vec2::new(0.0, 0.0),
        });

        let mut camera = Camera::with_window_size(ctx);
        camera.position.x = 32.0;
        camera.position.y = 48.0;
        camera.set_viewport_size(640.0, 480.0);
        camera.update();

        Ok(GameState {
            camera,
            tiles,
            map:[
                [3,1,1,1,1,4],
                [2,0,0,0,0,2],
                [2,0,0,0,0,2],
                [2,0,0,0,0,2],
                [2,0,0,0,0,2],
                [6,1,1,1,1,5]
                ],
            
            // log: Tile {
            //     texture: texture.clone(),
            //     clip: Rectangle::new(4.0*64.0, 12.0*64.0, 64.0, 64.0),
            //     origin: Vec2::new(0.0, 0.0),
            // },
            // tree: Tile {
            //     texture: texture.clone(),
            //     clip: Rectangle::new(2.0*64.0, 12.0*64.0, 64.0, 128.0),
            //     origin: Vec2::new(0.0, 64.0),
            // },
            // tree2: Tile {
            //     texture: texture.clone(),
            //     clip: Rectangle::new(7.0*64.0, 13.0*64.0, 192.0, 192.0),
            //     origin: Vec2::new(64.0, 128.0),
            // },
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        for row in 0..6 {
            for col in 0..6 {
                let x = (col * 32) as i32;
                let y = (row * 32) as i32;
                let tile_index = self.map[row][col];
                let tile = &self.tiles[&tile_index];
                tile.draw(ctx, x, y);
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