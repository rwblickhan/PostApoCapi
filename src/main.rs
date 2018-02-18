extern crate ggez;
use ggez::conf;
use ggez::event::*;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;

struct MainState {
    item: ItemEntity
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        let item = ItemEntity::new(ctx).unwrap();
        let s = MainState {
            item: item
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw(ctx,
                       &self.item.image,
                       graphics::Point2::new((self.item.xpos as f32), (self.item.ypos as f32)),
                       0.0).unwrap();
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32) {
        self.item.xpos = x;
        self.item.ypos = y;
    }
}

struct ItemEntity {
    image: graphics::Image,
    xpos: i32,
    ypos: i32
}

impl ItemEntity {
    fn new(ctx: &mut Context) -> GameResult<ItemEntity> {
        let img = graphics::Image::new(ctx, "/gah-ur.png").unwrap();
        Ok(ItemEntity {
            image: img,
            xpos: 0,
            ypos: 0
        })
    }
}

fn main() {
    let mut cb = ContextBuilder::new("postapocapi", "ggez")
        .window_setup(conf::WindowSetup::default()
            .title("PostApoCapi"))
        .window_mode(conf::WindowMode::default()
            .dimensions(1024, 768));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Exited cleanly.");
            }
        }
    }
}

