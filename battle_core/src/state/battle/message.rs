use std::collections::HashMap;

use oc_core::morale::Morale;
use serde::{Deserialize, Serialize};

use crate::{
    behavior::{gesture::Gesture, Behavior},
    entity::soldier::WeaponClass,
    game::flag::FlagsOwnership,
    order::Order,
    physics::{
        event::{bullet::BulletFire, explosion::Explosion},
        visibility::Visibility,
    },
    types::{Angle, SoldierIndex, SquadUuid, VehicleIndex, WorldPoint},
};

use super::phase::Phase;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BattleStateMessage {
    IncrementFrameI,
    Soldier(SoldierIndex, SoldierMessage),
    Vehicle(VehicleIndex, VehicleMessage),
    PushBulletFire(BulletFire),
    PushExplosion(Explosion),
    SetVisibilities(HashMap<(SoldierIndex, SoldierIndex), Visibility>),
    SetPhase(Phase),
    SetAConnected(bool),
    SetBConnected(bool),
    SetAReady(bool),
    SetBReady(bool),
    SetAMorale(Morale),
    SetBMorale(Morale),
    SetFlagsOwnership(FlagsOwnership),
    SetSquadLeader(SquadUuid, SoldierIndex),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SoldierMessage {
    SetWorldPosition(WorldPoint),
    SetBehavior(Behavior),
    SetGesture(Gesture),
    SetOrder(Order),
    SetOrientation(Angle),
    SetAlive(bool),
    SetUnconscious(bool),
    ReachBehaviorStep,
    IncreaseUnderFire(u32),
    DecreaseUnderFire,
    ReloadWeapon(WeaponClass),
    WeaponShot(WeaponClass),
    SetLastShootFrameI(u64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum VehicleMessage {
    SetWorldPosition(WorldPoint),
    SetChassisOrientation(Angle),
    // SetMainTurretOrientation(Angle),
}

// TODO : Side effects should not exists : All side effects
// should be computed when original message is produced
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SideEffect {
    // FIXME Systematic gui side
    RefreshEntityAnimation(SoldierIndex),
    SoldierFinishHisBehavior(SoldierIndex, Option<Order>),
}
