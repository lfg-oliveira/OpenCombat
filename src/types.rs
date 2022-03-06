use serde::{Deserialize, Serialize};
use std::ops::Add;

use glam::Vec2;

use crate::entity::Entity;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldX(f32);

impl From<f32> for WorldX {
    fn from(x: f32) -> Self {
        Self(x)
    }
}

impl Into<f32> for WorldX {
    fn into(self) -> f32 {
        self.0
    }
}

impl Add for WorldX {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldY(f32);

impl From<f32> for WorldY {
    fn from(y: f32) -> Self {
        Self(y)
    }
}

impl Into<f32> for WorldY {
    fn into(self) -> f32 {
        self.0
    }
}

impl Add for WorldY {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldPosition {
    pub x: WorldX,
    pub y: WorldY,
}

impl From<(WorldX, WorldY)> for WorldPosition {
    fn from(p: (WorldX, WorldY)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl Add for WorldPosition {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Into<Vec2> for WorldPosition {
    fn into(self) -> Vec2 {
        Vec2::new(self.x.into(), self.y.into())
    }
}

pub type EntityIndex = usize;
pub type SquadIndex = usize;
pub type SquadUuid = usize;
pub type ThreadSafeEntity = Box<dyn Entity + Send + Sync>;

pub struct SquadComposition(EntityIndex, Vec<EntityIndex>);

impl SquadComposition {
    pub fn new(leader: EntityIndex, members: Vec<EntityIndex>) -> Self {
        Self(leader, members)
    }
}

pub type Squads = Vec<SquadComposition>;
