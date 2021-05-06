pub mod tile;

use crate::map::tile::Tile;
use ggez::GameError;
use ggez::GameResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::{
    parse_with_path, Image as TiledImage, Map as TiledMap, Orientation, TiledError, Tileset,
};

pub fn map_from_tmx_file(file: File) -> GameResult<Map> {
    let reader = BufReader::new(file);
    // FIXME BS NOW: must give map path here !
    let map = match parse_with_path(reader, &Path::new("./resources/foo.tmx")) {
        Ok(map) => map,
        Err(e) => return GameResult::Err(GameError::CustomError(format!(
            "Fail to parse map: {:?}",
            e
        ))),
    };
    GameResult::Ok(Map::new(map)?)
}

pub struct Map {
    pub tiled_map: TiledMap,
    pub background_image: TiledImage,
    pub tiles: HashMap<(i32, i32), Tile>,
}

impl Map {
    fn new(tiled_map: TiledMap) -> GameResult<Self> {
        if tiled_map.orientation != Orientation::Orthogonal {
            // FIXME BS NOW: manage correctly error
            panic!("Map must be orthogonal orientation")
        }
        // FIXME BS NOW: manage correctly error
        let background_image = &(tiled_map.image_layers.first().unwrap())
            .image
            .as_ref()
            .unwrap()
            .clone();

        let terrain: Tileset = tiled_map
            .tilesets
            .clone()
            .into_iter()
            .filter(|t| t.name == "terrain")
            .collect::<Vec<Tileset>>()
            .first()
            .expect("No terrain tileset found")
            .clone();

        // FIXME BS NOW code it
        let tiles = vec![
            ((0, 0), Tile::from_str_id("SHORT_GRASS")),
            ((0, 0), Tile::from_str_id("SHORT_GRASS")),
            ((0, 0), Tile::from_str_id("SHORT_GRASS")),
            ((0, 0), Tile::from_str_id("SHORT_GRASS")),
        ]
        .into_iter()
        .collect();

        GameResult::Ok(Map {
            tiled_map,
            background_image: background_image.clone(),
            tiles,
        })
    }
}
