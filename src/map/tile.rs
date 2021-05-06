pub enum TileId {
    ShortGrass,
    MiddleGrass,
    HighGrass,
    Dirt,
    Mud,
    Concrete,
    BrickWall,
}

pub struct Tile {
    id: TileId,
    opacity: f32,
}

impl Tile {
    pub fn from_str_id(id: &str) -> Self {
        match id {
            "ShortGrass" => Self {
                id: TileId::ShortGrass,
                opacity: 0.0,
            },
            "MiddleGrass" => Self {
                id: TileId::MiddleGrass,
                opacity: 0.1,
            },
            "HighGrass" => Self {
                id: TileId::HighGrass,
                opacity: 0.2,
            },
            "Dirt" => Self {
                id: TileId::Dirt,
                opacity: 0.0,
            },
            "Mud" => Self {
                id: TileId::Mud,
                opacity: 0.1,
            },
            "Concrete" => Self {
                id: TileId::Concrete,
                opacity: 0.0,
            },
            "BrickWall" => Self {
                id: TileId::BrickWall,
                opacity: 1.0,
            },
            &_ => {
                // FIXME BS NOW: manage errors
                panic!("Unknown tile id {}", id)
            }
        }
    }
}
