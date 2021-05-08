pub mod tile;

use crate::map::tile::Tile;
use ggez::GameError;
use ggez::GameResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::{
    parse_with_path, Image as TiledImage, Image, Layer, LayerData, Map as TiledMap, Orientation,
    PropertyValue, Tile as TiledTile, TiledError, Tileset,
};

fn get_tile_from_terrain_tileset_with_id(terrain_tileset: &Tileset, id: u32) -> GameResult<Tile> {
    for tile in terrain_tileset.tiles.iter() {
        // FIXME why id is -1 ?
        if tile.id == id - 1 {
            let str_id = match tile.properties.get("ID") {
                None => {
                    return GameResult::Err(GameError::ResourceLoadError(format!(
                        "Tile {} have no ID property",
                        id
                    )))
                }
                Some(property_value) => match property_value {
                    PropertyValue::StringValue(str_id) => str_id.clone(),
                    _ => {
                        return GameResult::Err(GameError::ResourceLoadError(format!(
                            "Tile {} must have String ID property value",
                            id
                        )))
                    }
                },
            };
            return GameResult::Ok(Tile::from_str_id(&str_id));
        }
    }

    return GameResult::Err(GameError::ResourceLoadError(format!(
        "No tile with {} found",
        id
    )));
}

pub struct Map {
    pub tiled_map: TiledMap,
    pub background_image: TiledImage,
    pub terrain_image: TiledImage,
    pub tiles: HashMap<(u32, u32), Tile>,
}

impl Map {
    pub fn new(map_file_path: &Path) -> GameResult<Self> {
        let map_file = File::open(map_file_path)?;
        let map_file_reader = BufReader::new(map_file);
        let tiled_map = match parse_with_path(map_file_reader, map_file_path) {
            Ok(map) => map,
            Err(e) => {
                return GameResult::Err(GameError::ResourceLoadError(format!(
                    "Fail to parse map: {:?}",
                    e
                )))
            }
        };

        if &tiled_map.orientation != &Orientation::Orthogonal {
            return GameResult::Err(GameError::ResourceLoadError(
                "Map must be orthogonal orientation".to_string(),
            ));
        }
        // FIXME BS NOW: manage correctly error
        let background_image = match &(tiled_map.image_layers.first().unwrap()).image.as_ref() {
            None => {
                return GameResult::Err(GameError::ResourceLoadError(
                    "No image layer found in map ".to_string(),
                ))
            }
            Some(image) => image.clone(),
        };

        let terrain_tileset: Tileset = match tiled_map
            .tilesets
            .clone()
            .into_iter()
            .filter(|t| t.name == "terrain")
            .collect::<Vec<Tileset>>()
            .first()
        {
            None => {
                return GameResult::Err(GameError::ResourceLoadError(
                    "No terrain tileset found in map ".to_string(),
                ))
            }
            Some(tileset) => tileset.clone(),
        };

        let terrain_image = {
            match terrain_tileset.images.first() {
                None => {
                    return GameResult::Err(GameError::ResourceLoadError(
                        "No terrain image found in terrain tileset".to_string(),
                    ))
                }
                Some(terrain_image) => terrain_image.clone(),
            }
        };

        let terrain_layer: Layer = match tiled_map
            .layers
            .clone()
            .into_iter()
            .filter(|l| l.name == "terrain")
            .collect::<Vec<Layer>>()
            .first()
        {
            None => {
                return GameResult::Err(GameError::ResourceLoadError(
                    "No terrain layer found in map ".to_string(),
                ))
            }
            Some(layer) => layer.clone(),
        };

        let mut tiles: HashMap<(u32, u32), Tile> = HashMap::new();

        match terrain_layer.tiles {
            LayerData::Finite(layer_tiles) => {
                for (x, tiles_row) in layer_tiles.iter().enumerate() {
                    for (y, layer_tile) in tiles_row.iter().enumerate() {
                        let tile = get_tile_from_terrain_tileset_with_id(
                            &terrain_tileset,
                            layer_tile.gid,
                        )?;
                        tiles.insert((x as u32, y as u32), tile);
                    }
                }
            }
            LayerData::Infinite(_) => {
                return GameResult::Err(GameError::ResourceLoadError(
                    "Terrain layer must be finite".to_string(),
                ))
            }
        }

        GameResult::Ok(Map {
            tiled_map: tiled_map.clone(),
            background_image: background_image.clone(),
            terrain_image,
            tiles,
        })
    }
}
