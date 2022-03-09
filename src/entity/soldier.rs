use crate::{behavior::Behavior, types::*};
use serde::{Deserialize, Serialize};

use super::{Entity, EntityType};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Soldier {
    world_point: WorldPoint,
    squad_uuid: SquadUuid,
    behavior: Behavior,
    looking_direction: Angle,
}

impl Soldier {
    pub fn new(world_point: WorldPoint, squad_uuid: SquadUuid) -> Self {
        Self {
            world_point,
            squad_uuid,
            behavior: Behavior::Idle,
            looking_direction: Angle(0.0),
        }
    }

    pub fn from_entity(entity: &ThreadSafeEntity) -> Self {
        Self::new(entity.get_world_point(), entity.squad_uuid())
    }
}

impl Entity for Soldier {
    fn get_type(&self) -> EntityType {
        EntityType::Soldier
    }

    fn get_world_point(&self) -> WorldPoint {
        self.world_point
    }

    fn set_world_point(&mut self, point: WorldPoint) {
        self.world_point = point
    }

    fn squad_uuid(&self) -> SquadUuid {
        self.squad_uuid
    }

    fn get_behavior(&self) -> &Behavior {
        &self.behavior
    }

    fn set_behavior(&mut self, behavior: Behavior) {
        self.behavior = behavior
    }

    fn get_looking_direction(&self) -> Angle {
        self.looking_direction
    }

    fn set_looking_direction(&mut self, angle: Angle) {
        self.looking_direction = angle
    }
}
