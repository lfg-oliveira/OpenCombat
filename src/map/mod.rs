pub mod tile;

use crate::map::tile::Tile;
use ggez::GameError;
use ggez::GameResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::{parse_with_path, Image as TiledImage, Map as TiledMap, Orientation, TiledError, Tileset, Image};

pub struct Map {
    pub tiled_map: TiledMap,
    pub background_image: TiledImage,
    pub tiles: HashMap<(i32, i32), Tile>,
}

impl Map {
    pub fn new(map_file_path: &Path) -> GameResult<Self> {
        let map_file = File::open(map_file_path)?;
        let map_file_reader = BufReader::new(map_file);
        let tiled_map = match parse_with_path(map_file_reader, map_file_path) {
            Ok(map) => map,
            Err(e) => return GameResult::Err(GameError::ResourceLoadError(format!(
                "Fail to parse map: {:?}",
                e
            ))),
        };

        if &tiled_map.orientation != &Orientation::Orthogonal {
            return GameResult::Err(GameError::ResourceLoadError("Map must be orthogonal orientation".to_string()))
        }
        // FIXME BS NOW: manage correctly error
        let background_image = match &(tiled_map.image_layers.first().unwrap())
            .image
            .as_ref() {
            None => {
                return GameResult::Err(GameError::ResourceLoadError("No image layer found in map ".to_string()))
            }
            Some(image) => {image.clone()}
        };

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
            tiled_map: tiled_map.clone(),
            background_image: background_image.clone(),
            tiles,
        })
    }
}
