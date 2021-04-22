use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer::check_update_time;
use ggez::{Context, GameResult};
use std::env;
use std::path;

const TARGET_ANIMATION_FPS: u32 = 10;

struct SpriteInfo {
    relative_start_y: f32,
    relative_tile_width: f32,
    relative_tile_height: f32,
    tile_count: u16,
    current_frame: u16,
}

impl SpriteInfo {
    pub fn new(
        relative_start_y: f32,
        relative_tile_width: f32,
        relative_tile_height: f32,
        tile_count: u16,
    ) -> Self {
        Self {
            relative_start_y,
            relative_tile_width,
            relative_tile_height,
            tile_count,
            current_frame: 0,
        }
    }

    pub fn from_type(
        type_: &SpriteType,
        sprite_sheet_width: f32,
        sprite_sheet_height: f32,
    ) -> Self {
        let (start_y, tile_width, tile_height, tile_count) = match type_ {
            SpriteType::WalkingSoldier => (0.0, 24.0, 24.0, 7),
            SpriteType::JumpingSoldier => (24.0, 24.0, 24.0, 2),
        };

        Self {
            relative_start_y: start_y / sprite_sheet_height,
            relative_tile_width: tile_width / sprite_sheet_width,
            relative_tile_height: tile_height / sprite_sheet_height,
            tile_count,
            current_frame: 0,
        }
    }
}

enum SpriteType {
    WalkingSoldier,
    JumpingSoldier,
}

fn sprite_batch_part_from_sprite_info(sprite_info: &SpriteInfo) -> graphics::DrawParam {
    let src = graphics::Rect::new(
        sprite_info.current_frame as f32 * sprite_info.relative_tile_width,
        sprite_info.relative_start_y,
        sprite_info.relative_tile_width,
        sprite_info.relative_tile_height,
    );
    graphics::DrawParam::new().src(src)
}

struct SceneItem {
    sprite_info: SpriteInfo,
    position: na::Point2<f32>,
}

impl SceneItem {
    pub fn new(
        sprite_type: SpriteType,
        position: na::Point2<f32>,
        sprite_sheet_width: f32,
        sprite_sheet_height: f32,
    ) -> Self {
        Self {
            sprite_info: SpriteInfo::from_type(
                &sprite_type,
                sprite_sheet_width,
                sprite_sheet_height,
            ),
            position,
        }
    }

    pub fn tick_frame(&mut self) {
        self.sprite_info.current_frame += 1;
        if self.sprite_info.current_frame >= self.sprite_info.tile_count {
            self.sprite_info.current_frame = 0;
        }
    }
}

enum Message {}

struct MainState {
    scene_items_sprite_batch: graphics::spritebatch::SpriteBatch,
    scene_items: Vec<SceneItem>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/test.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);

        let mut scene_items = vec![];
        for x in 0..1 {
            for y in 0..4 {
                let sprite_type = if y % 2 == 0 {
                    SpriteType::WalkingSoldier
                } else {
                    SpriteType::JumpingSoldier
                };

                scene_items.push(SceneItem::new(
                    sprite_type,
                    na::Point2::new(x as f32 * 24.0, y as f32 * 24.0),
                    168.0,
                    48.0,
                ));
            }
        }

        let s = MainState {
            scene_items_sprite_batch: batch,
            scene_items,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while check_update_time(ctx, TARGET_ANIMATION_FPS) {
            for scene_item in self.scene_items.iter_mut() {
                scene_item.tick_frame();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for scene_item in self.scene_items.iter() {
            let sprite_batch_part = sprite_batch_part_from_sprite_info(&scene_item.sprite_info)
                .dest(scene_item.position.clone());
            self.scene_items_sprite_batch.add(sprite_batch_part);
        }
        graphics::draw(
            ctx,
            &self.scene_items_sprite_batch,
            graphics::DrawParam::new().dest(na::Point2::new(0.0, 0.0)),
        )?;

        self.scene_items_sprite_batch.clear();
        graphics::present(ctx)?;

        println!("FPS: {}", ggez::timer::fps(ctx));
        Ok(())
    }
}
// TODO: spite i par objet, fabrication des sprite_info qu'une fois; channel pour modifs des objets ds update
pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("oc", "bux")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
